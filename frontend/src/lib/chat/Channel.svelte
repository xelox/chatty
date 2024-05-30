<script lang='ts'>
import type { SchemaMessageGroup, SchemaChannel, SchemaMessage, SchemaUpMessage } from "../../stores/messages";
import event_manager from '../../event_manager';
import { afterUpdate, beforeUpdate, onDestroy, onMount } from "svelte";
import { user_data } from "../../stores/data";
import { requests_manager, type RequestOptions } from "../../requests_manager";
import MessagesGroup from "./MessagesGroup.svelte";
import { BiSequencer, type SequentialID } from "../../util/sequencer";

export let channel_info: SchemaChannel;
const sequencer = new BiSequencer();
let groups = new Map<SequentialID, SchemaMessageGroup>();
const map_message_to_group = new Map<string, SequentialID>;
let first_group_id: SequentialID | null = null;
let last_group_id: SequentialID | null = null;
let oldest_loaded_ts: null | number = null;

const unsubscribe_callbacks: (()=>void)[] = [];

unsubscribe_callbacks.push(
  event_manager.subscribe({action: "message_add", channel_id: channel_info.id}, (message: SchemaMessage) => {
    insert_message(message, 'down');
  }),
);

let messages_wrap_dom: HTMLElement;
let fully_loaded = false;
const GROUPING_TD = 60_000;

const pick_group = (message: SchemaMessage, from: 'up' | 'down'): {group: SchemaMessageGroup, id: SequentialID} => {
  {
    const id = (() => {
      if (from === 'up') return first_group_id; 
      return last_group_id;
    })();

    if (id) {
      const group = groups.get(id)!;
      const same_sender = group.sender_id === message.sender_id;
      const t_delta = (()=> {
        if (from === 'up') return group.group_ts_end - message.sent_at;
        return message.sent_at - group.group_ts_end;
      })();

      if (same_sender && t_delta <= GROUPING_TD) {
        return {group, id};
      }
    }
  }

  {
    const id = (() => {
      if (from === 'up') {
        first_group_id = sequencer.up();
        return first_group_id;
      } 
      last_group_id = sequencer.down();
      return last_group_id;
    })();

    const group: SchemaMessageGroup = {
      sender_id: message.sender_id,
      group_ts_start: message.sent_at,
      group_ts_end: message.sent_at,
      messages: {}
    }

    groups.set(id, group);
    return {group, id};
  }
}

const insert_message = (message: SchemaMessage, to: 'up' | 'down') => {
  const {group, id} = pick_group(message, to);
  map_message_to_group.set(message.id, id);

  group.messages[message.id] = message;

  if (message.sent_at < group.group_ts_start ) {
    group.group_ts_start = message.sent_at
  }

  if (message.sent_at > group.group_ts_end ) {
    group.group_ts_end = message.sent_at
  }

  groups = groups;
  
  unsubscribe_callbacks.push(
    event_manager.subscribe({action: "message_delete", message_id: message.id}, (message: SchemaMessage) => {
      delete_message(message);
    })
  );
}


const delete_message = (message: SchemaMessage) => {
  const group_id = map_message_to_group.get(message.id);
  map_message_to_group.delete(message.id);
  if (!group_id) {
    return console.error("tried to delete a message who's group does not exist.");
  }

  const group = groups.get(group_id)!;
  delete group.messages[message.id];
  if (Object.keys(group.messages).length === 0) {
    groups.delete(group_id);
  }

  groups = groups;
}

let just_loaded_messages = false;
const load_messages = () => {
  if (fully_loaded) return;
  let ts = oldest_loaded_ts ?? new Date().getTime();

  requests_manager.load_messages(channel_info.id, ts).then(loaded_messages => {
    if (loaded_messages.length === 0) return fully_loaded = true;
    oldest_loaded_ts = loaded_messages[loaded_messages.length - 1].sent_at;

    for (const message of loaded_messages) {
      insert_message(message, 'up');
    }

    just_loaded_messages = true;
  })
}

let autoscroll = false;
let pre_update_scroll = 0;
onMount(() => {
  load_messages();
});

beforeUpdate(()=>{
  if (messages_wrap_dom) {
    pre_update_scroll = messages_wrap_dom.scrollHeight;
    const scrollable_dist = messages_wrap_dom.scrollHeight - messages_wrap_dom.offsetHeight;
    autoscroll = messages_wrap_dom.scrollTop > scrollable_dist - 20;
  }
})
afterUpdate(()=>{
  if (just_loaded_messages) {
    just_loaded_messages = false;
    messages_wrap_dom.scrollTo(0, messages_wrap_dom.scrollHeight - pre_update_scroll);
  }
  if (autoscroll) messages_wrap_dom.scrollTo(0, messages_wrap_dom.scrollHeight);
});

onDestroy(() => {
  for (const unsubscribe of unsubscribe_callbacks) {
    unsubscribe();
  }
});

let input_text = "";

const handle_keypress = (e: KeyboardEvent) => {
  if (e.key == 'Enter' && !e.shiftKey && !e.ctrlKey) {
    const user_id = $user_data?.id;
    if (!user_id) return;
    const message: SchemaUpMessage = {
      channel_id: channel_info.id,
      content: input_text,
      sender_id: user_id,
    }
    requests_manager.send_message(message);
    input_text = "";
    return;
  }
}

const handle_scroll = () => {
  if (messages_wrap_dom.scrollTop === 0) {
    load_messages();
  }
}

let input_height: number;
let wrap_height: number;

</script>

<main bind:clientHeight={wrap_height}>
  <div bind:this={messages_wrap_dom} on:scroll={handle_scroll} class="messages_wrap" style="height: calc({wrap_height}px - {input_height}px);">
    <div class="whitespace_up"></div>
    {#each [...groups.entries()].sort((a, b) => b[0] - a[0]) as [key, group] (key)}
      <MessagesGroup {group}/>
    {/each}
  </div>
  <div class="input_wrap" bind:clientHeight={input_height}>
    <textarea bind:value={input_text} on:keypress={handle_keypress}></textarea>
  </div>
</main>

<style>
.whitespace_up {
  height: 1em;
}
main {
  height: 100%;
  display: flex;
  flex-direction: column;
  background: var(--mantle);
}
.messages_wrap {
  flex-grow: 1;
  overflow-y: auto;
  position: relative;
}
.input_wrap {
  padding: 10px;
}
textarea {
  background: var(--base);
  resize: none;
  color: var(--text);
  width: calc(100% - 8px);
  border: 1px solid var(--overlay0);
  padding: 4px;
  border-radius: 4px; 
}
</style>

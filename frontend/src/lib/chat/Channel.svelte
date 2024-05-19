<script lang='ts'>
import type { MessageGroup, SchemaChannel, SchemaMessage, SchemaUpMessage } from "../../stores/messages";
import event_manager from '../../event_manager';
import { afterUpdate, beforeUpdate, onDestroy, onMount } from "svelte";
import { user_data } from "../../stores/data";
import MentionTool from "./MentionTool.svelte";
import { requests_manager, type RequestOptions } from "../../requests_manager";
import MessagesGroup from "./MessagesGroup.svelte";

export let channel_info: SchemaChannel;

let messages: MessageGroup[] = [];
let oldest_loaded_ts: null | number = null;

const unsubscribe_callbacks: (()=>void)[] = [];

unsubscribe_callbacks.push(
  event_manager.subscribe("message_add", (message: SchemaMessage) => {
    if (message.channel_id != channel_info.id) return;
    const last_group = messages.pop();
    if (last_group) {
      const same_sender = last_group.sender_id === message.sender_id;
      const time_delta = last_group.group_ts_start - message.sent_at;
      if (!same_sender || time_delta > 60_000) {
        const new_group: MessageGroup = {
          sender_id: message.sender_id,
          group_ts_start: message.sent_at,
          messages: {[message.id]: message},
          group_ts_end: message.sent_at,
        };
        messages = [...messages, last_group, new_group];
        return;
      }

      last_group.messages[message.id] = message;
      last_group.group_ts_end = message.sent_at;
      messages = [...messages, last_group];
      return;
    }

    const new_group: MessageGroup = {
      sender_id: message.sender_id,
      group_ts_start: message.sent_at,
      messages: {[message.id]: message},
      group_ts_end: message.sent_at,
    };
    messages = [...messages, new_group];
    return;
  })
);

unsubscribe_callbacks.push(
  event_manager.subscribe("message_delete", (message: SchemaMessage) => {
    if (message.channel_id != channel_info.id) return;
    for (const group of messages) {
      if (group.sender_id !== message.sender_id) continue; 
      const time_delta = Math.abs(group.group_ts_start - message.sent_at);
      if (time_delta > 60_000 || time_delta < 0) continue;
      delete group.messages[message.id];
    }
  })
);

let messages_wrap_dom: HTMLElement;
let fully_loaded = false;

const load_messages = () => {
  if (fully_loaded) return;
  let ts = oldest_loaded_ts ?? new Date().getTime();

  const opts: RequestOptions = {
    succeed_action: (messages_raw) => {
      const loaded_messages: SchemaMessage[] = JSON.parse(messages_raw);
      if (loaded_messages.length === 0) return fully_loaded = true;
      oldest_loaded_ts = loaded_messages[loaded_messages.length - 1].sent_at;

      for (const message of loaded_messages) {
        const first_group = messages.shift();
        if (first_group) {
          const same_sender = first_group.sender_id === message.sender_id;
          const time_delta = first_group.group_ts_end - message.sent_at;
          console.log(time_delta, same_sender, message.content);
          if (!same_sender || time_delta > 60_000) {
            const new_group: MessageGroup = {
              group_ts_end: message.sent_at,
              sender_id: message.sender_id,
              group_ts_start: message.sent_at,
              messages: {[message.id]: message}
            };
            messages = [new_group, first_group, ...messages];
            continue;
          }

          first_group.messages[message.id] = message;
          first_group.group_ts_start = message.sent_at;
          messages = [first_group, ...messages];
          continue;
        }

        const new_group: MessageGroup = {
          group_ts_end: message.sent_at,
          sender_id: message.sender_id,
          group_ts_start: message.sent_at,
          messages: {[message.id]: message}
        };
        messages = [new_group, ...messages];
        continue;
      }

      
    }
  }

requests_manager.get(`/api/load_messages/${channel_info.id}/${ts}`, opts);
}

onMount(() => {
  load_messages();
})

let autoscroll = false;
beforeUpdate(()=>{
  if (messages_wrap_dom) {
    const scrollable_dist = messages_wrap_dom.scrollHeight - messages_wrap_dom.offsetHeight;
    autoscroll = messages_wrap_dom.scrollTop > scrollable_dist - 20;
  }
})
afterUpdate(()=>{
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
    requests_manager.post("/api/send_message", message);
    input_text = "";
    return;
  }
}

let mention_search: string | null = null;

const handle_input = (e: Event) => {
  mention_search = null;
  const cursor_index = (e.target as HTMLTextAreaElement).selectionStart;
  let mention_start = input_text.lastIndexOf('@', cursor_index);
  if (mention_start === -1) {
    return;
  }

  let mention_end = input_text.indexOf(' ', mention_start);
  if (mention_end === -1) {
    mention_end = input_text.length;
  }

  const mention = input_text.substring(mention_start, mention_end);
  if (mention.length > 0 && cursor_index >= mention_start && cursor_index <= mention_end) {
    mention_search = mention;
    return;
  }
}

const handle_scroll = (e: Event) => {
  if (messages_wrap_dom.scrollTop === 0) {
    load_messages();
  }
}

let input_height: number;
let wrap_height: number;

</script>

<main bind:clientHeight={wrap_height}>
  <div bind:this={messages_wrap_dom} on:scroll={handle_scroll} class="messages_wrap" style="height: calc({wrap_height}px - {input_height}px);">
    {#each messages.sort((a, b) => a.group_ts_start - b.group_ts_start) as group}
      <MessagesGroup {group}/>
    {/each}
  </div>
  <div class="input_wrap" bind:clientHeight={input_height}>
    {#if mention_search}
      <MentionTool mention_search={mention_search}/>
    {/if}
    <textarea bind:value={input_text} on:keypress={handle_keypress}></textarea>
  </div>
</main>

<style>
main {
  height: 100%;
  display: flex;
  flex-direction: column;
  background: var(--base);
}
.messages_wrap {
  flex-grow: 1;
  overflow-y: auto;
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

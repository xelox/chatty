<script lang='ts'>
import type { SchemaChannel, SchemaMessageList, SchemaMessage, SchemaUpMessage } from "../../stores/messages";
import event_manager from '../../event_manager';
import { afterUpdate, beforeUpdate, onDestroy, onMount } from "svelte";
import Message from './Message.svelte';
import { user_data } from "../../stores/data";
import MentionTool from "./MentionTool.svelte";
import { requests_manager, type RequestOptions } from "../../requests_manager";

export let channel_info: SchemaChannel;

let messages: SchemaMessageList = {};
let oldest_loaded_ts: null | number = null;

const unsubscribe_callbacks: (()=>void)[] = [];
unsubscribe_callbacks.concat(
  event_manager.subscribe("message_add", (message: SchemaMessage) => {
    if (message.channel_id != channel_info.id) return;
    messages[message.id] = message;
  })
);

function to_time_str(ts: number) {
  const o = new Date(ts);
  const day = o.getDate().toString().padStart(2, '0');;
  const month = o.getMonth().toString().padStart(2, '0');;
  const year = o.getFullYear();
  const h = o.getHours().toString().padStart(2, '0');;
  const m = o.getMinutes().toString().padStart(2, '0');;
  const s = o.getSeconds().toString().padStart(2, '0');
  const ms = o.getMilliseconds();
  return `${year}/${month}/${day} ${h}:${m}:${s}.${ms}`;
}

unsubscribe_callbacks.push(
  event_manager.subscribe("channel_delete", (message: SchemaMessage) => {
    if (message.channel_id != channel_info.id) return;
    delete messages[message.id];
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

      const tmp: SchemaMessageList = {};
      for (const message of loaded_messages) {
        tmp[message.id] = message;
      }
      oldest_loaded_ts = loaded_messages[loaded_messages.length - 1].sent_at;
      console.log(to_time_str(oldest_loaded_ts));
      messages = {...messages, ...tmp};
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
    {#each Object.values(messages).sort((a, b) => {return a.sent_at - b.sent_at}) as message (message.id)} 
      <Message {message}/>
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
  scroll-behavior: smooth;
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

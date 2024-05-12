<script lang='ts'>
import type { SchemaChannel, SchemaMessageList, SchemaMessage } from "../../stores/messages";
import event_manager from '../../event_manager';
import { onDestroy } from "svelte";
import Message from './Message.svelte';
import { user_data } from "../../stores/data";
import { uuidv4 } from "uuidv7";
    import MentionTool from "./MentionTool.svelte";
    import CustomTextarea from "./CustomTextarea.svelte";

export let channel_info: SchemaChannel;
const messages: SchemaMessageList = {};

const unsubscribe_callbacks: (()=>void)[] = [];
unsubscribe_callbacks.concat(
  event_manager.subscribe_multiple(["message_add", "message_update"], (message: SchemaMessage) => {
    if (message.channel_id != channel_info.id) return;
    messages[message.id] = message;
  })
);

unsubscribe_callbacks.push(
  event_manager.subscribe("channel_delete", (message: SchemaMessage) => {
    if (message.channel_id != channel_info.id) return;
    delete messages[message.id];
  })
);

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
    const message: SchemaMessage = {
      id: uuidv4(),
      channel_id: channel_info.id,
      content: input_text,
      sender_id: user_id,
      is_sent: false,
    }
    event_manager.dispatch("message_add", message);
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

</script>

<main>
  <div class="messages_wrap">
    {#each Object.values(messages) as message} 
      <Message {message}/>
    {/each}
  </div>
  <div class="input_wrap">
    {#if mention_search}
      <MentionTool mention_search={mention_search}/>
    {/if}
    <CustomTextarea/>
  </div>
</main>

<style>
.messages_wrap {
  flex: 1;
}
main {
  height: 100%;
  display: flex;
  flex-direction: column;
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

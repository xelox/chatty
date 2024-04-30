<script lang='ts'>
import type { SchemaChannel, SchemaMessageList, SchemaMessage } from "../../stores/messages";
import event_manager from '../../event_manager';
import { onDestroy } from "svelte";
import Message from './Message.svelte';
    import { user_data } from "../../stores/data";
    import { uuidv4, uuidv7 } from "uuidv7";

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
      id: uuidv7(),
      channel_id: channel_info.id,
      content: input_text,
      sender_id: user_id,
      is_sent: false,
    }
    event_manager.dispatch("message_add", message);
  } else if (e.key == 'Enter') {
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
    <textarea bind:value={input_text} on:keypress={handle_keypress}></textarea>
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
  background: none;
  resize: none;
  color: var(--text);
  width: calc(100% - 8px);
  border: 1px solid var(--overlay0);
  padding: 4px;
  border-radius: 4px; 
}
</style>

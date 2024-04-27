<script lang="ts">
import { uuidv7 } from "uuidv7";
import { requests_manager, type RequestOptions } from "../../requests_manager";
import { active_channel, user_data, type SchemaMessage } from "../../stores/data"

let input_content = "";

const try_send_message = (e: KeyboardEvent) => {
  if (e.key == 'Enter') {
    let content = (e.target as HTMLInputElement).value;
    if ($active_channel) {
      let id = uuidv7();
      let message: SchemaMessage = {
        id,
        sender_id: $user_data!.id,
        channel_id: $active_channel.channel_id,
        content
      }
      active_channel.update(store => {
        store!.messages[id] = message;
        return store;
      })
      const opts: RequestOptions = {
        succeed_action: () => {
          input_content = "";
          active_channel.update(store => {
            store!.messages[id].is_sent = true;
            return store;
          })
        },
        notify_fail: true
      }
      requests_manager.post("/api/send_message", message, opts);
    }
  }
  return e;
}
</script>

<main>
  <div class="chat_wrapper">
    <div class="messages_wrapper">
      {#if $active_channel}
        {#each Object.values($active_channel.messages) as message}
          <div class="message_element {!message.is_sent ? 'unsent_message' : ''}">
            <p>{message.sender_id}</p>
            <p>{message.content}</p>
          </div>
        {/each}
      {/if}
    </div>
    <input type="text" class='input' placeholder="Chatty chatty chattttt..." on:keypress={try_send_message} bind:value={input_content}>
  </div>
</main>

<style>
.unsent_message {
  opacity: 0.8;
}

.messages_wrapper {
  display: flex;
  flex-direction: column;
  flex: 1;
}
.input {
  margin: 4px;
  border-radius: 4px;
  padding: 4px;
  background: var(--surface0);
  border: none;
  margin: 20px;
  padding: 10px;
  color: var(--vividtext);
}
.chat_wrapper {
  display: flex;
  flex: 1;
  flex-direction: column;
}
main {
  background: var(--base);
  display: flex;
  flex-direction: column;
  height: 100%;
}
</style>

<script lang="ts">
import { requests_manager, type RequestOptions } from "../../requests_manager";
import { active_channel, user_data } from "../../stores/data"

let input_content = "";

    // TODO: call send message api endpoint.
    const try_send_message = (e: KeyboardEvent) => {
      if (e.key == 'Enter') {
        let content = (e.target as HTMLInputElement).value;
        if ($active_channel) {
          const opts: RequestOptions = {
            succeed_action: () => {
              console.log("YAYYY!");
              input_content = "";
            },
            notify_fail: true
          }
          requests_manager.post("/api/send_message", {
            sender_id: $user_data.id,
            channel_id: $active_channel.channel_id,
            content
          }, opts);
        }
      }
      return e;
    }
</script>

<main>
  <div class="chat_wrapper">
    <div class="messages_wrapper">
    </div>
    <input type="text" class='input' placeholder="Chatty chatty chattttt..." on:keypress={try_send_message} bind:value={input_content}>
  </div>
</main>

<style>
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

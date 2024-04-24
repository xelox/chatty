<script lang="ts">

type message = {
  body: string,
  author: string,
  id: string
}

let messages: message[] = [];

// TODO: call send message api endpoint.
const try_send_message = (e: Event) => {
  let new_message = (e.target as HTMLInputElement).value;
}
</script>

<main>
  <div class="chat_wrapper">
    <div class="messages_wrapper">
    </div>
    <input type="text" class='input' placeholder="Chatty chatty chattttt..." on:change={try_send_message}>
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

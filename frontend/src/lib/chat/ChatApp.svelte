<script lang="ts">
  import Header from "./Header.svelte";
let self = {
  name: "John Snow"
}

let channel = {
  id: "slahdglkj"
}

const socket = new WebSocket("ws://localhost:8080/ws");
let i = 0;
socket.onopen = _ => {
  console.log("web socket open");
  setInterval(()=>{
    socket.send(`hello! ${i}`);
    i++;
  }, 1000);
}
socket.onmessage = e => {
  console.log("socket message:", e.data);
}
socket.onclose = _ => {
  console.log("web socket close");
}
socket.onerror = e => {
  console.error("web socket error", e);
}

type message = {
  body: string,
  author: string,
  id: string
}

let messages: message[] = [];
messages.push({
  author: 'Marco Polo',
  body: 'Some Random Test Message',
  id: 'salkdjgklasjhm'
});

messages.push({
  author: 'Marco Polo',
  body: 'Some Random Test Message',
  id: 'salkdjgklasjhm'
});

messages.push({
  author: 'John Snow',
  body: 'This is a message from John Snow',
  id: 'salkdjgklasjhm'
});

messages.push({
  author: 'Marco Polo',
  body: 'Some Random Test Message',
  id: 'salkdjgklasjhm'
});



const try_send_message = (e: Event) => {
  let new_message = (e.target as HTMLInputElement).value;
  messages.push({
    body: new_message,
    author: self.name,
    id: 'akshg'
  });
  messages = messages;

  fetch('http://localhost:8080/api/post_message', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({
      content: new_message,
      author_id: self.name,
      channel_id: channel.id,
      attachments: [],
    })
  }).then(res => {
      res.text().then(text => {
        console.log(text);
      }).catch(err => console.error(err))
    }).catch(err => console.error(err))
}
</script>

<main>
  <Header/>
  <div class="chat_wrapper">
    <div class="messages_wrapper">
      {#each messages as message}
        <div class="message_div {message.author == self.name ? 'self_message' : ''}">
          <p class="author">{message.author}</p>
          <p>{message.body}</p>
        </div>
      {/each}
    </div>
    <input type="text" class='input' placeholder="Chatty chatty chattttt..." on:change={try_send_message}>
  </div>
</main>

<style>
.author {
  color: var(--vividtext);
  font-size: 1rem;
}
.message_div {
  padding: 4px;
  border-bottom: 1px solid var(--overlay0);
  padding: 4px 20px 4px 20px;
  font-size: 0.9rem;
}
.message_div>p {
  margin: 0;
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
  height: 100vh;
}
</style>

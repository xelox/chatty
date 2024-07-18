<script lang='ts'>
import { requests_manager } from "../requests_manager";

let input = '';
const send_friend_request = () => {
  requests_manager.send_friend_req(input).finally(()=>{
    input = '';
  })
}
</script>

<main>
  <div class='wrap1'>
    <span>Send request</span>
    <div class="wrap2">
      <input bind:value={input} class='search_input' type="text">
      <button class='send_btn' on:click={send_friend_request}>Send</button>
    </div>
  </div>
</main>

<style>
  .search_input {
    width: 50vw;
    background: var(--surface0);
    color: var(--text);
    font-family: 'Noto Sans';
  }
  .send_btn {
    background: var(--green);
    color: var(--crust);
  }
  .wrap1 {
    position: fixed;
    top: 50%; left: 50%;
    transform: translate(-50%, -50%);
  }
  .wrap2 > * {
    padding: 10px;
    border-radius: 4px;
    border: none;
    font-weight: bolder;
  } 
  .wrap2 {
    display: flex;
    flex-direction: row;
    gap: 10px;
  }
  main {
    display: block;
    width: 100vw;
    height: 100vh;
    position: fixed;
  }
</style>

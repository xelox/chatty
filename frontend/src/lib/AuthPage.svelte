<script lang="ts">
import socket_manager from "../socket_manager";
import notification_manager from "../notification_manager";
import { requests_manager, type RequestOptions } from "../requests_manager";
import type { Notification } from "../stores/inbox";
import { router } from "../stores/router";

let username = ""
let password = ""
let unique_name_label_focus = false;
let password_label_focus = false;

const validate = (): boolean => {
  let valid = true;
  // TODO: propper username validation.
  if (username === "") {
    const notification: Notification = {
      ts: Number(new Date()),
      content: "Username can not be empty",
      source: "System",
    }
    notification_manager.notify(notification, 'system');
    valid = false;
  }
  // TODO: propper password stregth validation.
  if (password === "") {
    const notification: Notification = {
      ts: Number(new Date()),
      content: "Password can not be empty",
      source: "System",
    }
    notification_manager.notify(notification, 'system');
    valid = false;
  }
  return valid;
}

const signup = () => {
  if (!validate()) return;

  requests_manager.signup(username, password).then(() => {
      router.route("/app/chat");
      socket_manager.initialize_client();
  })
}

const signin = () => {
  if (!validate()) return;

  requests_manager.signin(username, password).then(() => {
      router.route("/app/chat");
      socket_manager.initialize_client();
  })
}

</script>

<main>
  <div class="outer_wrap">
    <h2>Log In</h2>
    <div class="input_wrap">
      <span  class={`label ${unique_name_label_focus || username !== "" ? 'small_label' : ''}`}>Unique Name</span>
      <input bind:value={username} type="text" 
        on:focusin={()=>{ unique_name_label_focus = true; }}
        on:focusout={()=>{ unique_name_label_focus = false; }}
      >
    </div>
    <div class="input_wrap">
      <span class={`label ${password_label_focus || password !== "" ? 'small_label' : ''}`}>Password</span>
      <input bind:value={password} type="password"
        on:focusin={()=>{ password_label_focus = true; }}
        on:focusout={()=>{ password_label_focus = false; }}
      >
    </div>
    <div class="input_wrap">
      <input type="button" value="Log In" on:click={signin}>
      <input type="button" value="Sign Up" on:click={signup}>
    </div>
  </div>
</main>

<style>
.label {
  position: absolute;
  transform: translate(6px, 8px);
  transition: 500ms;
}
.small_label {
  opacity: 0.5;
  font-size: var(--size-small);
  transform: translate(6px, -2px);
}
.input_wrap {
  display: flex;
}
.input_wrap > input {
  padding: 4px;
  margin: 6px;
  border-radius: 4px;
  border: none;
  color: white;
  background: black;
  outline: none;
}
input[type="text"], input[type="password"] {
  background: none;
  border-bottom: 1px solid gray;
  border-radius: 0;
}
.outer_wrap {
  display: flex;
  flex-direction: column;
  position: absolute;
  left: 50%;
  top: 50%;
  transform: translate(-50%, -50%);
}
h2 {
  text-align: center;
}
main {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
}
</style>

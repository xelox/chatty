<script lang="ts">
let unique_name = ""
let password = ""
let unique_name_label_focus = false;
let password_label_focus = false;

const signup = () => {
  if (unique_name === "" && password === "") {
    document.dispatchEvent(new CustomEvent('notification', {
      detail: {
        type: 'error',
        info: 'Please provide a unique_name and a password.'
      }
    }));
    return;
  }
  if (unique_name === "") {
    document.dispatchEvent(new CustomEvent('notification', {
      detail: {
        type: 'error',
        info: 'Please provide a unique_name.'
      }
    }));
    return;
  }
  if (password === "") {
    document.dispatchEvent(new CustomEvent('notification', {
      detail: {
        type: 'error',
        info: 'Please provide a unique_name.'
      }
    }));
    return;
  }

  fetch(`http://localhost:8080/api/signup`, {
    method: "POST",
    headers: [
      ["Content-Type", "application/json"]
    ],
    body: JSON.stringify({
      unique_name,
      password
    })
  }).then(fetch_res => {
      fetch_res.text().then(res => {
        console.log(res);
        if ( res === 'OK' ) {
          window.location.href = "chat"
        }
      })
    });
}

const login = () => {
  if (unique_name === "" && password === "") {
    document.dispatchEvent(new CustomEvent('notification', {
      detail: {
        type: 'error',
        info: 'Please provide a unique_name and a password.'
      }
    }));
    return;
  }
  if (unique_name === "") {
    document.dispatchEvent(new CustomEvent('notification', {
      detail: {
        type: 'error',
        info: 'Please provide a unique_name.'
      }
    }));
    return;
  }
  if (password === "") {
    document.dispatchEvent(new CustomEvent('notification', {
      detail: {
        type: 'error',
        info: 'Please provide a unique_name.'
      }
    }));
    return;
  }

  fetch(`http://localhost:8080/api/signin`, {
    method: "POST",
    headers: [
      ["Content-Type", "application/json"]
    ],
    body: JSON.stringify({
      unique_name,
      password
    })
  }).then(fetch_res => {
      fetch_res.text().then(res => {
        console.log(res);
        if ( res === 'OK' ) {
          window.location.href = "chat"
        }
      })
    });
}
</script>

<main>
  <div class="outer_wrap">
    <h2>Log In</h2>
    <div class="input_wrap">
      <span  class={`label ${unique_name_label_focus || unique_name !== "" ? 'small_label' : ''}`}>Unique Name</span>
      <input bind:value={unique_name} type="text" 
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
      <input type="button" value="Log In" on:click={login}>
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
  font-size: 0.7rem;
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

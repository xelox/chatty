<script lang='ts'>
import { router, router_state } from "../../stores/router";

const pannels_list = {
  account: "Account",
  privacy: "Privacy",
  appearence: "Appearence",
  autio_video: "Autio & Video",
  notifications: "Notifications",
  keybinds: "Keybinds"
};
</script>

<main>
  <div class="buttons_wrap">
    {#each Object.entries(pannels_list) as [key, title]} 
      <button class="pannel_btn" class:active_section={$router_state.settings_nav_section === key} on:click={()=>{ 
        router.route(`/app/settings/${key}`);
      }}>{title}</button>
    {/each}
  </div>
</main>

<style>
.pannel_btn{
  cursor: pointer;
  text-align: left;
  font-family: 'Noto Sans';
  font-size: var(--size-large);
  padding: 4px;
  border-radius: 4px; 
  margin: 1px;
}
.pannel_btn:hover {
  background: var(--surface0);
}
.active_section {
  background: var(--surface1);
}

.buttons_wrap {
  display: flex;
  flex-direction: column;
}

main{
  display: flex;
  flex-direction: column;
  justify-content: center;
  padding: 20px;
  background: var(--base);
  height: 100%;
  width: calc(100% - 40px);
  height: calc(100% - 40px);
}
</style>

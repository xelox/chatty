<script lang='ts'>
import { router, router_state, SETTINGS_NAV_SECTIONS } from "../../stores/router";
import { settings_left_pos } from "../../stores/ui";
let ui_width = 0;
$: if($settings_left_pos) {
  ui_width = $settings_left_pos - 40;
  console.log(ui_width);
}
</script>

<main style="width: {ui_width}px;">
  <div class="buttons_wrap">
    {#each Object.entries(SETTINGS_NAV_SECTIONS) as [key, title]} 
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
  height: calc(100% - 40px);
}
</style>

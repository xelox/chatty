<script lang='ts'>
import { router_state, SETTINGS_NAV_SECTIONS } from "../../stores/router";
import { settings_left_pos } from "../../stores/ui";
import Link from "../components/Link.svelte";
let ui_width = 0;
$: if($settings_left_pos) {
  ui_width = $settings_left_pos - 40;
}
</script>

<main>
  <div class="buttons_wrap">
    {#each Object.entries(SETTINGS_NAV_SECTIONS) as [key, title]} 
      <Link to={`/app/settings/${key}`}>
      <div class="pannel_btn" class:active_section={$router_state.settings_nav_section === key}>{title}</div>
      </Link>
    {/each}
  </div>
</main>

<style>
.pannel_btn{
  cursor: pointer;
  text-align: left;
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
  width: max-content;
}

main{
  height: 100%;
}
</style>

<script lang='ts'>
import { router_state, SETTINGS_NAV_SECTIONS } from "../../stores/router";
import Account from './Accont.svelte';
import { settings_left_pos } from '../../stores/ui';
import { onMount } from "svelte";
import SettingsLeftNav from "./SettingsLeftNav.svelte";
let main: HTMLElement;

const updateUiStore = () => {
  const rect = main.getBoundingClientRect();
  $settings_left_pos = rect.x;
}
window.addEventListener('resize', updateUiStore);
onMount(()=>{
  updateUiStore();
})
</script>

<main>
  <div class="left_empty"></div>
  <div class="wrap_all">
    <div class="section_wrap" bind:this={main}>
      <div class="inner_wrap">
        {#if $router_state.settings_nav_section === 'account' } <Account/>
        {:else} <h1>{SETTINGS_NAV_SECTIONS[$router_state.settings_nav_section]} is not implemented yet.</h1>
        {/if}
      </div>
    </div>
    <div class="nav_wrap">
      <div class="inner_wrap">
        <SettingsLeftNav/>
      </div>
    </div>
  </div>
  <div class="right_empty"></div>
</main>

<style>
.section_wrap {
  flex: 1;
  background: var(--base);
}
.inner_wrap {
  height: calc(100% - 40px);
  padding: 20px;
}
main > div {
  height: 100%;
  overflow-y: auto;
}
.wrap_all {
  height: 100%;
  width: 1000px;
  display: flex;
  flex-direction: row;
}
main {
  background: var(--mantle);
  display: flex;
  flex-direction: row;
  width: 100%;
  height: 100%;
}
.left_empty {
  background: var(--base);
  flex: 1;
}
.right_empty {
  flex: 1;
}
h1 {
  text-align: right;
}
</style>

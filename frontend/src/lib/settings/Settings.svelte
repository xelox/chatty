<script lang='ts'>
import { router_state, SETTINGS_NAV_SECTIONS } from "../../stores/router";
import Account from './Accont.svelte';
import { header_height, settings_left_pos } from '../../stores/ui';
    import { onMount } from "svelte";
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

<main style="height: calc(100vh - {$header_height})">
  <div class="wrap" bind:this={main}>
    {#if $router_state.settings_nav_section === 'account' } <Account/>
    {:else} <h1>{SETTINGS_NAV_SECTIONS[$router_state.settings_nav_section]} is not implemented yet.</h1>
    {/if}
  </div>
</main>

<style>
main {
  overflow: auto;
  position: fixed;
  height: 100vh;
  width: 800px;
  transform: translateX(-50%);
  left: 50%;
}
.wrap {
  position: absolute;
  min-height: calc(100% - 40px);
  padding: 20px;
  width: 800px;
  background: var(--crust);
}
</style>

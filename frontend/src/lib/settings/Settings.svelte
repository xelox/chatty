<script lang='ts'>
import { router_state, SETTINGS_NAV_SECTIONS } from "../../stores/router";
import Account from './Accont.svelte';
</script>

<main>
  <h1>Settings</h1>
  {#if $router_state.settings_nav_section === 'account' } <Account/>
  {:else} <h1>{SETTINGS_NAV_SECTIONS[$router_state.settings_nav_section]} is not implemented yet.</h1>
  {/if}
</main>

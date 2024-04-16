<script lang="ts">
import inbox from '../../stores/inbox'
// import type { notification, inbox_store } from '../../stores/inbox';
import Notification from './notification.svelte';
let group: 'dms' | 'guilds' | 'system' = 'dms';

</script>

<main>
  <div class="header_wrap">
    <button on:click={()=>{group = 'dms'}}>DMs</button>
    <button on:click={()=>{group = 'guilds'}}>Guilds</button>
    <button on:click={()=>{group = 'system'}}>System</button>
  </div>
  <div class="notifications_wrap">
    {#each ($inbox[group]).sort((a, b)=>{return a.ts - b.ts}) as n}
      <Notification {n}/>
    {/each}
  </div>
</main>

<style>
.header_wrap > button {
  background: none;
  border: none;
  color: var(--text);
  font-family: 'Noto Sans';
}
main{
  position: fixed;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  background: var(--crust);
  min-width: 80vw;
  min-height: 80vh;
  border-radius: 4px;
  padding: 4px;
  border: 1px solid var(--surface2);
}
</style>

<script lang="ts">
  import {header_end} from '../../stores/ui'
  import FriendsSection from "./FriendsSection.svelte"
  import GuildsSection from "./GuildsSection.svelte"
  import RequestsSection from "./RequestsSection.svelte"

  type section_enum = "friends" | "guilds" | "requests";  
  let section: section_enum = "friends";
</script>

<main style="top: {$header_end}px; height: calc(100vh - {$header_end}px)">
  <div class="buttons_wrap">
    <button class="button" on:click={()=>{section = "friends"}}> Friends </button>
    <button class="button" on:click={()=>{section = "guilds"}}> Guilds </button>
    <button class="button" on:click={()=>{section = "requests"}}> Requests </button>
  </div> 
  <div class="active_section_wrapper">
    {#if section == "friends"}
      <FriendsSection/>
    {:else if section == "guilds"}
      <GuildsSection/>
    {:else if section == "requests"}
      <RequestsSection/>
    {/if}
  </div>
</main>

<style>
.buttons_wrap {
  margin-top: 10px;
  display: flex;
  flex-direction: row;
  justify-content: space-evenly;
}
.button {
  font-style: italic;
  cursor: pointer;
}

.button:hover {
  text-decoration: underline;
}
main {
  position: fixed;
  background: var(--crust);
  height: 100vh;
  width: 260px;
  border: 1px solid var(--overlay0);
  border-top: none;
}
</style>

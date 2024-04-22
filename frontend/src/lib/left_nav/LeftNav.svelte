<script lang="ts">
  import {header_end} from '../../stores/ui'
  import FriendsSection from "./FriendsSection.svelte"
  import GuildsSection from "./GuildsSection.svelte"
  import RequestsSection from "./RequestsSection.svelte"

  type section_enum = "friends" | "guilds" | "requests";  
  let section: section_enum = "friends";
  let subpath: string = "";
  let path_extra: string | null = null;
  $: if(section) {
    const args: string[] = [section];
    if (path_extra) args.push(path_extra);
    subpath = "/" + args.join("/")
  }
</script>

<main style="top: {$header_end}px; height: calc(100vh - {$header_end}px)">
  <div class="buttons_wrap">
    <button class="button" on:click={()=>{section = "friends"}}> Friends </button>
    <button class="button" on:click={()=>{section = "guilds"}}> Guilds </button>
    <button class="button" on:click={()=>{section = "requests"}}> Requests </button>
  </div> 
  <p class="subpath">{subpath}</p>
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
.subpath {
  font-family: "maple";
  padding: 2px 10px 2px 10px;
  font-size: 0.65rem;
  margin: 0;
  border-bottom: 1px solid var(--overlay0);
}
.buttons_wrap {
  padding: 10px;
  display: flex;
  flex-direction: row;
  justify-content: space-between;
  border-bottom: 1px solid var(--overlay0);
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

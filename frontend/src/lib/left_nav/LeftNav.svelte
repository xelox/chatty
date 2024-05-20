<script lang="ts">
import { router, router_state } from "../../stores/router";
import FriendsSection from "./FriendsSection.svelte"
import GuildsSection from "./GuildsSection.svelte"
import RequestsSection from "./RequestsSection.svelte"

type section_enum = "friends" | "guilds" | "requests";  
let subpath: string = "";
let path_extra: string | null = null;
$: if($router_state.chat_nav_section) {
  const args: string[] = [$router_state.chat_nav_section];
  if (path_extra) args.push(path_extra);
  subpath = "/" + args.join("/")
}

const manipulate_path = (s: string) => {
  path_extra = s;
}
</script>

<main>
  <p class="subpath">{subpath}</p>
  <div class="buttons_wrap">
    <button class="button" on:click={()=>{router.route("/app/chat/friends")}} class:active_section={$router_state.chat_nav_section === 'friends'}> Friends </button>
    <button class="button" on:click={()=>{router.route("/app/chat/guilds")}} class:active_section={$router_state.chat_nav_section === 'guilds'}> Guilds </button>
    <button class="button" on:click={()=>{router.route("/app/chat/requests")}} class:active_section={$router_state.chat_nav_section === 'requests'}> Requests </button>
  </div> 
  <div class="active_section_wrapper">
    {#if $router_state.chat_nav_section === "friends"}
      <FriendsSection {manipulate_path}/>
    {:else if $router_state.chat_nav_section === "guilds"}
      <GuildsSection {manipulate_path}/>
    {:else if $router_state.chat_nav_section === "requests"}
      <RequestsSection {manipulate_path}/>
    {/if}
  </div>
</main>

<style>
.subpath {
  font-family: "maple";
  padding: 2px 10px 2px 10px;
  font-size: var(--size-mini);
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
  /* font-style: italic; */
  cursor: pointer;
  font-size: var(--size-normal);
}

.button:hover {
  text-decoration: underline;
}
main {
  background: var(--base);
  border: 1px solid var(--overlay0);
  border-top: none;
  height: 100%;
  min-width: 260px;
}
.active_section {
  color: var(--yellow);
}
</style>

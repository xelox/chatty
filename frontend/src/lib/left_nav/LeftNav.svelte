<script lang="ts">
import { router, router_state } from "../../stores/router";
import FriendsSection from "./FriendsSection.svelte"
import GuildsSection from "./GuildsSection.svelte"
import RequestsSection from "./RequestsSection.svelte"
</script>

<main>
  <div class="buttons_wrap">
    <button class="button" on:click={()=>{router.route("/app/chat/friends")}} class:active_section={$router_state.chat_nav_section === 'friends'}> Friends </button>
    <button class="button" on:click={()=>{router.route("/app/chat/guilds")}} class:active_section={$router_state.chat_nav_section === 'guilds'}> Guilds </button>
    <button class="button" on:click={()=>{router.route("/app/chat/requests")}} class:active_section={$router_state.chat_nav_section === 'requests'}> Requests </button>
  </div> 
  <div class="active_section_wrapper">
    {#if $router_state.chat_nav_section === "friends"}
      <FriendsSection/>
    {:else if $router_state.chat_nav_section === "guilds"}
      <GuildsSection/>
    {:else if $router_state.chat_nav_section === "requests"}
      <RequestsSection/>
    {/if}
  </div>
</main>

<style>
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
  border-top: none;
  height: 100%;
  width: 100%;
}
.active_section {
  color: var(--yellow);
}
</style>

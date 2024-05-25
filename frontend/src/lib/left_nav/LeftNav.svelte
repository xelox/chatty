<script lang="ts">
import { router_state } from "../../stores/router";
import Link from "../components/Link.svelte";
import FriendsSection from "./FriendsSection.svelte"
import GuildsSection from "./GuildsSection.svelte"
import RequestsSection from "./RequestsSection.svelte"
</script>

<main>
  <div class="buttons_wrap">
    <Link to='/app/chat/friends'> 
      <div class="button" class:active_section={$router_state.chat_nav_section === 'friends'}> Friends</div>
    </Link>
    <Link to='/app/chat/guilds'> 
      <div class="button" class:active_section={$router_state.chat_nav_section === 'guilds'}> Guilds</div>
    </Link>
    <Link to='/app/chat/requests'> 
      <div class="button" class:active_section={$router_state.chat_nav_section === 'requests'}> Requests</div>
    </Link>
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
  padding: 4px 16px;
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

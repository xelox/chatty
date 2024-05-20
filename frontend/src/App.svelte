<script lang="ts">
import ChatApp from './lib/chat/ChatApp.svelte';
import AuthPage from './lib/AuthPage.svelte';
import FriendRequestUi from './lib/FriendRequestUi.svelte';
import Header from './lib/Header.svelte';
import Inbox from './lib/inbox/main.svelte';
import search_query from './stores/search_query';
import socket_manager from './socket_manager';
import ActiveNotification from './lib/active_notification.svelte';
import LeftNav from './lib/left_nav/LeftNav.svelte';
import { user_data } from './stores/data';
import { router_state } from './stores/router';
import Settings from './lib/settings/Settings.svelte';
import SettingsLeftNav from './lib/settings/SettingsLeftNav.svelte';

socket_manager.initialize_client(); 

</script>

<main>
  {#if $user_data}
    <div class="top_zone">
      <Header></Header>
    </div>
  <div class="bottom_zone">
      {#if $router_state.show_left_nav}
        <div class="left_zone">
          {#if $router_state.main_section === 'chat'} <LeftNav/> {/if}
          {#if $router_state.main_section === 'settings'} <SettingsLeftNav/> {/if}
        </div>
      {/if}
    <div class="middle_zone">
      {#if $router_state.main_section === 'chat'} <ChatApp/> {/if}
      {#if $router_state.main_section === 'friend_req_tool'} <FriendRequestUi/> {/if}
      {#if $router_state.main_section === 'settings'} <Settings/> {/if}
    </div>
  </div>
  {/if}
  {#if $router_state.main_section === 'auth'}
    <AuthPage/>
  {/if}
</main>
{#if $user_data && $search_query.active_overlay === 'inbox'}
  <Inbox/>
{/if}

<ActiveNotification/>

<style>
main{
  display: flex;
  flex-direction: column;
  height: 100vh;
  width: 100vw;
}
.bottom_zone {
  display: flex;
  flex-grow: 1;
}
.middle_zone {
  flex-grow: 1; 
}
</style>

<script lang="ts">
import { Router, Route } from 'svelte-routing'
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

socket_manager.initialize_client(); 

export let url = ""
</script>

<main>
  {#if $user_data}
    <div class="top_zone">
      <Header></Header>
    </div>
  {/if}
  <div class="bottom_zone">
      {#if $user_data}
    <div class="left_zone">
      <LeftNav/>
    </div>
      {/if}
    <div class="middle_zone">
      <Router {url}>
        <Route path="/app/chat"> <ChatApp/> </Route>
        <Route path="/app/auth"> <AuthPage/> </Route>
        <Route path="/app/add_friend"> <FriendRequestUi/> </Route>
      </Router>
    </div>
  </div>
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
  flex: 1;
}
.middle_zone {
  flex: 1; 
}
</style>

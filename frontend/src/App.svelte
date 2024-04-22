<script lang="ts">
import { Router, Route } from 'svelte-routing'
import ChatApp from './lib/chat/ChatApp.svelte';
import AuthPage from './lib/AuthPage.svelte';
import AddFriend from './lib/add_friend/main.svelte';
import Header from './lib/Header.svelte';
import Inbox from './lib/inbox/main.svelte';
import search_query from './stores/search_query';
import _socket_manager from './socket_manager';
import ActiveNotification from './lib/active_notification.svelte';
import LeftNav from './lib/left_nav/LeftNav.svelte';
export let url = ""
</script>

<main>
  <LeftNav/>
  <Router {url}>
    <Header></Header>
    <Route path="/app/chat"> <ChatApp/> </Route>
    <Route path="/app/auth"> <AuthPage/> </Route>
    <Route path="/app/a/add_friend/:id" let:params> <AddFriend id={Number(params.id)}/> </Route>
    <Route path="/app/a/add_friend"> <AddFriend id={null}/> </Route>
  </Router>

  {#if $search_query.active_overlay === 'inbox'}
    <Inbox/>
  {/if}

  <ActiveNotification/>
</main>

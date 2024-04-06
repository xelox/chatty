<script lang="ts">
    import { Router, Route } from 'svelte-routing'
    import ChatApp from './lib/chat/ChatApp.svelte';
    import AuthPage from './lib/AuthPage.svelte';
    let url = ""
</script>

<main>
    <Router {url}>
        <Route path="/app/chat"> <ChatApp/> </Route>
        <Route path="/app/auth"> <AuthPage/> </Route>
    </Router>
</main>

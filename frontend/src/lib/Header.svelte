<script lang='ts'>
  import { navigate } from "svelte-routing";
  import search_query from "../stores/search_query"
  import {erase} from "../stores/data"
    import { requests_manager, type RequestOptions} from "../requests_manager";

  const nav_click = (to: {url?: string, overlay?: string}) => {
    const url = to.url ?? `${window.location.href.split('?')[0]}`;
    const query_params = new URLSearchParams(window.location.search);
    if(to.overlay !== undefined) {
      if(query_params.get('active_overlay') === to.overlay) to.overlay = 'none';
      query_params.set('active_overlay', to.overlay);
    }
    navigate(`${url}?${query_params.toString()}`);
    search_query.set(Object.fromEntries(query_params.entries()));
  }

  const logout = () => {
    const opts: RequestOptions = {
      succeed_action: () => {
        erase();
        nav_click({url: '/app/auth'});
      },
      notify_fail: true,
    } 

    requests_manager.get('/api/logout', opts);
  }
</script>

<main>
  <div class="group">
    <button class="btn" on:click={() => {nav_click({url: '/app/chat'})}}>
      <img class="btn_svg" src="/svg-files/Communication/comments-alt-2.svg" title="chat" alt="link to chat"/>
    </button>
    <button class="btn" on:click={() => {nav_click({url: '/app/add_friend'})}}>
      <img class="btn_svg" src="/svg-files/Interface and Sign/circle-plus.svg" title="add friend" alt="link to add friend page"/>
    </button>
    <button class="btn" on:click={() => {nav_click({overlay: 'inbox'})}}>
      <img class="btn_svg" src="/svg-files/Web and Technology/alarm.svg" title="inbox" alt="button to inbox"/>
    </button>
  </div>

  <div class="group">
    <button class="btn" on:click={logout}>
      <img class="btn_svg" src="/svg-files/Direction/exit.svg" title="inbox" alt="button to inbox"/>
    </button>
  </div>
</main>

<style>
  .btn {
    background: none;
    border: none;
    width: 30px;
  }
  .btn_svg {
    filter: invert(87%) sepia(6%) saturate(987%) hue-rotate(192deg) brightness(98%) contrast(94%);
  }
  .group {
    display: flex;
    gap: 4px;
    padding: 4px;
  }
  main {
    background: var(--surface0);
    border-bottom: 1px solid var(--overlay0);
    display: flex;
    justify-content: space-between;
    width: 100%;
  }
</style>

<script lang='ts'>
import { header_height } from "../stores/ui";
import Link from "./components/Link.svelte";
let rect: DOMRect;
$: if(rect) {
  $header_height = rect.bottom;
}
</script>

<main bind:contentRect={rect}>
  <div class="group">
    <Link to='/app/chat'>
      <img class="btn_svg" src="/svg-files/Communication/comments-alt-2.svg" title="chat" alt="link to chat"/>
    </Link>
    <Link to='/app/add_friend'>
      <img class="btn_svg" src="/svg-files/Interface and Sign/circle-plus.svg" title="add friend" alt="link to add friend page"/>
    </Link>
  </div>

  <div class="group">
    <Link to='/app/settings'>
      <img class="btn_svg" src="/svg-files/Web and Technology/user.svg" title="inbox" alt="button to inbox"/>
    </Link>
    <Link to='/app/logout'>
      <img class="btn_svg" src="/svg-files/Direction/exit.svg" title="inbox" alt="button to inbox"/>
    </Link>
  </div>
</main>

<style>
  .btn_svg {
    filter: invert(87%) sepia(6%) saturate(987%) hue-rotate(192deg) brightness(98%) contrast(94%);
    width: 20px;
    aspect-ratio: 1/1;
    margin: 4px;
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

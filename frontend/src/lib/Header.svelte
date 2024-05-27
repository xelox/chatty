<script lang='ts'>
import { header_height } from "../stores/ui";
    import Icon from "./components/Icon.svelte";
import Link from "./components/Link.svelte";
let rect: DOMRect;
$: if(rect) {
  $header_height = rect.bottom;
}
</script>

<main bind:contentRect={rect}>
  <div class="group">
    <Link to='/app/chat'>
      <Icon variant='chat'/>
    </Link>
    <Link to='/app/add_friend'>
      <Icon variant='plus'/>
    </Link>
  </div>

  <div class="group">
    <Link to='/app/settings'>
      <Icon variant='settings'/>
    </Link>
    <Link to='/app/logout'>
      <Icon variant='exit'/>
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

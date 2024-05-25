<script lang='ts'>
import { router } from "../../stores/router";
export let to: string;
</script>

<button on:click={()=>{router.route(to)}}>
  <slot/>
</button>

<style>
button {
  width: 100%;
  background: transparent;
  outline: none;
  border: none;
  padding: 0;
  margin: 0;
}
</style>

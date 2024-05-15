<script lang='ts'>
import { onMount } from "svelte";


type Segment = {
  type: 'plain' | 'mention' | 'br',
  content?: string
}

const segments: Segment[] = [];

let segments_index = 0;
let cursor = 0;

const handle_keypress = (e: KeyboardEvent) => {
}

</script>

{cursor} <br>
<div class="view">
  {#each segments as seg}
    {#if seg.type === 'br'} <br>
    {:else if seg.type === 'plain'} {seg.content}
    {/if}
  {/each}
</div>
<main contenteditable on:keydown={handle_keypress}></main>

<style>
main{
  height: 2em;
  background: var(--base);
  resize: none;
  color: var(--text);
  width: calc(100% - 8px);
  border: 1px solid var(--overlay0);
  padding: 4px;
  border-radius: 4px; 
  opacity: 0;
}
main:focus {
  border: 1px solid white;
}
</style>

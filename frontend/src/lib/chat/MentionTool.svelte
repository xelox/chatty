<script lang='ts'>
import Fuse, { type FuseResult } from "fuse.js";
import known_users from '../../stores/users'
import type { SchemaPeer } from "../../stores/data";
import { onDestroy } from "svelte";

var fuse: Fuse<SchemaPeer> = new Fuse([], {
  keys: ['username'],
  threshold: 0.6,
});
const unsubscribe = known_users.subscribe(list => {
  fuse.setCollection(Object.values(list));
});

export let mention_search: string;
let result: SchemaPeer[] = [];

$: if(mention_search) {
  result = fuse.search(mention_search).map(match => {
    return match.item;
  });
}


onDestroy(() => {
  unsubscribe();
});
</script>

<main>
  {#each result as suggestion, idx}
    <p class="{idx === 0 ? 'best' : ''}">@{suggestion.display_name ?? suggestion.username}</p>
  {/each}
</main>

<style>
p{
  padding: 4px;
  color: var(--blue);
}
.best {
  background: var(--base);
}
main{
  background: var(--mantle);
  border-top-left-radius: 10px;
  border-top-right-radius: 10px;
  overflow: hidden;
  border: 1px solid var(--overlay0);
  border-bottom: none;
}
</style>

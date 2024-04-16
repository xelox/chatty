<script lang="ts">
  import Search from "./search.svelte";
  export let id: number | null = 0;
</script>

<main>
  {#if id === null}
    <Search/>
  {:else if isNaN(id)}
    Invalid input
  {:else}
    Searched user info {id}
  {/if}
</main>

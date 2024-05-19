<script lang='ts'>
import type { SchemaMessage } from "../../stores/messages";
import Tag from './Tag.svelte';
import SvelteMarkdown from "svelte-markdown";
import UserIdWrap from "./UserIdWrap.svelte";
export let message: SchemaMessage;


type MessageFragment = {
  kind: "text" | "user_mention" | "channel_mention",
  content: string,
}

const content_segments: MessageFragment[] = message.content.split(/([@%][0-9a-z-]+)/g).map(item => {
  if (item.startsWith('@')) return { 
    kind: 'user_mention',
    content: item.substring(1, item.length) 
  }
  if (item.startsWith('%')) return {
    kind: 'channel_mention',
    content: item.substring(1, item.length)
  }
  return {
    kind: 'text',
    content: item
  }
});
</script>

<main class="{message.id ? '' : 'unsent'}">
  <p class="content">
    {#each content_segments as segment}
      {#if segment.kind === 'text'}
        <SvelteMarkdown source={segment.content}/>
      {:else}
        <Tag tag_type={segment.kind} id={segment.content}/>
      {/if}
    {/each}
  </p>
</main>

<style>
main {
  padding: 4px;
}
.unsent {
  opacity: 0.8;
}
</style>

<script lang='ts'>
    import { user_data } from "../../stores/data";
import type { SchemaMessage } from "../../stores/messages";
    import Button from "../components/Button.svelte";
    import Icon from "../components/Icon.svelte";
import Tag from './Tag.svelte';
import SvelteMarkdown from "svelte-markdown";

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

<main class:unsent={!message.id}>
  {#each content_segments as segment}
    {#if segment.kind === 'text'}
      <SvelteMarkdown source={segment.content} isInline={true}/>
    {:else}
      <Tag tag_type={segment.kind} id={segment.content}/>
    {/if}
  {/each}

  <div class="controlls_wrap">
    {#if message.sender_id === $user_data?.id}
      <Button bg='blue'><Icon variant='edit' size='16px'/></Button> 
    {:else}
      <Button bg='blue'><Icon variant='reply' size='16px'/></Button> 
    {/if}
    <Button bg='red'><Icon variant='delete' size='16px'/></Button> 
  </div>
</main>

<style>
main{
  white-space: pre-line;
  padding: 0 8px;
  border-radius: 2px;
  position: relative;
}
main:hover {
  background: var(--surface0);
}

.controlls_wrap {
  background: var(--surface0);
  padding: 4px 10px;
  border-radius: 4px;
  position: absolute;
  top: 0; right: 0;
  flex-direction: row;
  gap: 4px;
  display: none;
  transform: translateY(-50%);
}

main:hover .controlls_wrap {
  display: flex;
}
.unsent {
  opacity: 0.5;
}
</style>

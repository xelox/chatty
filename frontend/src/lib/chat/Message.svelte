<script lang='ts'>
import type { SchemaMessage } from "../../stores/messages";
import Tag from './Tag.svelte';
import SvelteMarkdown from "svelte-markdown";
import UserIdWrap from "./UserIdWrap.svelte";
export let message: SchemaMessage;

function to_time_str(ts: number) {
  const o = new Date(ts);
  const h = o.getHours().toString().padStart(2, '0');;
  const m = o.getMinutes().toString().padStart(2, '0');;

  const today = new Date();

  if (today.toDateString() === o.toDateString()) {
    return `${h}:${m}`;
  }

  const yesterday = new Date(today.setDate(today.getDate() - 1));
  if (yesterday.toDateString() === o.toDateString()) {
    return `Yesterday at ${h}:${m}`;
  }

  const day = o.getDate().toString().padStart(2, '0');;
  const month = o.getMonth().toString().padStart(2, '0');;
  const year = o.getFullYear();

  return `${day}/${month}/${year} ${h}:${m}`;
}

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
  <UserIdWrap id={message.sender_id}/>
  <span class='time'>{to_time_str(message.sent_at)}</span>
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

.time {
  opacity: 0.8;
  font-size: var(--size-small);
}
</style>

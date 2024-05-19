<script lang="ts">
import type { MessageGroup } from "../../stores/messages";
import Message from "./Message.svelte";
import UserIdWrap from "./UserIdWrap.svelte";
export let group: MessageGroup;

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

</script>

<main>
  <UserIdWrap id={group.sender_id}/>
  <span class='time'>{to_time_str(group.group_ts_start)}</span>
  {#each Object.values(group.messages).sort((a, b) => {return a.sent_at - b.sent_at}) as message (message.id)} 
    <Message {message}/>
  {/each}
</main>

<style>
.time {
  opacity: 0.8;
  font-size: var(--size-small);
}
</style>

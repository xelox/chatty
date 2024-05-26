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
  <div class="left">
    <img src={`/media/pfp/${group.sender_id}.png`} alt='' class='pfp'>
  </div>
  <div class="right">
    <div class="head">
      <UserIdWrap id={group.sender_id}/>
      <span class='time'>{to_time_str(group.group_ts_start)}</span>
    </div>
    {#each Object.values(group.messages).sort((a, b) => {return a.sent_at - b.sent_at}) as message (message.id)} 
      <Message {message}/>
    {/each}
  </div>
</main>

<style>
.time {
  opacity: 0.8;
  font-size: var(--size-small);
}
.head{
  padding: 2px 8px 2px 8px;
}
main {
  margin-bottom: 1em;
  display: flex;
}
.pfp {
  width: 46px;
  aspect-ratio: 1/1;
  border-radius: 100vh;
  margin: 4px 10px;
}
.right {
  flex: 1;
}
</style>

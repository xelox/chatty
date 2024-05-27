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
  <div class="middle">
    <div class="head">
      <span class="user"><UserIdWrap id={group.sender_id}/></span>
      <span class='time'>{to_time_str(group.group_ts_start)}</span>
    </div>
    {#each Object.values(group.messages).sort((a, b) => {return a.sent_at - b.sent_at}) as message (message.id)} 
      <Message {message}/>
    {/each}
  </div>
  <div class="right"></div>
</main>

<style>
.left, .right {
  padding: 4px 4px 0 12px;
  width: 46px;
}
.user {
  color: var(--blue);
  font-weight: bolder;
}
.time {
  opacity: 0.8;
  font-size: var(--size-small);
}
.head{
  padding: 0 8px;
}
main {
  margin-bottom: 1em;
  display: flex;
  flex-direction: row;
}
.pfp {
  width: inherit;
  aspect-ratio: 1/1;
  border-radius: 100vh;
}
.middle {
  flex: 1;
  width: 0; /* This is weird I know... */
}
</style>

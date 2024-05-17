<script lang="ts">
import {friend_list} from '../../stores/data'
import {active_channel, channels_store, type SchemaChannel} from '../../stores/messages';

export let manipulate_path: (s: string) => void;
manipulate_path("");
const open_channel = (channel_id: string, friend_name: string) => {
  if (!$channels_store[channel_id]) {
    channels_store.add_channel({ id: channel_id, channel_name: friend_name });
  }
  manipulate_path(friend_name);
  active_channel.set(channel_id);
}
</script>

<main>
  {#each Object.values($friend_list) as friend_item}
    <button class="friend_item" style="background: {$active_channel == friend_item.relation_id ? 'var(--base)' : ''};" on:click={()=>{open_channel(friend_item.relation_id, friend_item.username)}}>
      <span class="left"> <img src="" title="{friend_item.display_name ?? friend_item.username}" alt=""> </span>
      <div class="right">
        <span class="display_name">{friend_item.display_name ?? friend_item.username}</span>
      </div>
    </button>
  {/each}
</main>

<style>
.display_name {
  font-size: 1em;
}
.left img {
  border-radius: 100%;
  width: 36px;
  aspect-ratio: 1/1;
  border: none;
  margin: 0;
}
.right {
  display: flex;
  flex-direction: column;
  justify-content: space-around
}
.friend_item {
  font-family: "maple";
  padding: 4px;
  display: flex;
  flex-direction: row;
  gap: 4px;
  border-radius: 4px;
  width: 100%;
}
main {
  padding: 10px 4px 10px 4px;
}
</style>

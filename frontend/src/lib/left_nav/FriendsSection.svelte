<script lang="ts">
import { friend_list } from '../../stores/data'
import { channels_store } from '../../stores/messages';
import { router, router_state } from '../../stores/router';

export let manipulate_path: (s: string) => void;
const open_channel = (channel_id: string) => {
  router.route(`/app/chat/friends/${channel_id}`);
}
</script>

<main>
  {#each Object.entries($friend_list) as [relation_id, friend_item]}
    <button class="friend_item" class:active_channel={relation_id === $router_state.active_channel} on:click={()=>{open_channel(relation_id)}}>
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
.active_channel {
  background-color: var(--surface0);
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

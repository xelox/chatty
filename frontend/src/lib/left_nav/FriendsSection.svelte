<script lang="ts">
import { friend_list } from '../../stores/data'
import { router_state } from '../../stores/router';
import Link from '../components/Link.svelte';
</script>

<main>
  {#each Object.entries($friend_list) as [relation_id, friend_item]}
    <Link to={`/app/chat/friends/${relation_id}`}>
    <div class='friend_item' class:active_channel={relation_id === $router_state.active_channel}>
      <span class="left"> <img src="{friend_item?.pfp_url}" title="{friend_item.display_name ?? friend_item.username}" alt=""> </span>
      <div class="right">
        <span class="display_name">{friend_item.display_name ?? friend_item.username}</span>
      </div>
    </div>
    </Link>
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
}
main {
  padding: 10px 4px 10px 4px;
}
</style>

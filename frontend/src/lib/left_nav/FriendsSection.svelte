<script lang="ts">
import { friend_list } from '../../stores/data'
import { router_state } from '../../stores/router';
import Link from '../components/Link.svelte';
</script>

<main>
  {#each Object.entries($friend_list) as [relation_id, friend_item]}
    <Link to={`/app/chat/friends/${relation_id}`}>
    <div class='friend_item' class:active_channel={relation_id === $router_state.active_channel}>
      <span class="left"> 
        <img src={`/media/pfp/${friend_item.id}.png`} title="{friend_item.display_name ?? friend_item.username}" alt="">
      </span>
      <div class="right">
        <p class="line_one">{friend_item.display_name ?? friend_item.username}</p>
        {#if friend_item.custom_status}
          <p class="line_two">{friend_item.custom_status}</p>
        {/if}
      </div>
    </div>
    </Link>
  {/each}
</main>

<style>
.line_one {
  font-size: var(--size-large);
  color: var(--vividtext);
}
.left img {
  border-radius: 100%;
  width: 46px;
  aspect-ratio: 1/1;
  border: none;
}
.active_channel {
  background-color: var(--surface0);
}
.left {
  display: flex;
  padding: 4px;
}
.right {
  display: flex;
  flex-direction: column;
  justify-content: center;
  flex: 1;
  text-align: left;
}
.friend_item {
  display: flex;
  flex-direction: row;
  gap: 10px;
  border-radius: 4px;
}
main {
  padding: 10px 4px 10px 4px;
}
</style>

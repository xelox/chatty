<script lang="ts">
import {friend_list, type SchemaChannel, active_channel} from '../../stores/data'

export let manipulate_path: (s: string) => void;
manipulate_path("");
const open_channel = (channel: SchemaChannel) => {
  active_channel.set(channel);
}
</script>

<main>
  {#each Object.entries($friend_list) as [relation_id, friend_item]}
    <button class="friend_item" on:click={()=>{open_channel({
      channel_id: relation_id,
      channel_name: relation_id,
      messages: {}
    })}}>
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
  padding: 2px 10px 2px 10px;
  margin: 0;
  display: flex;
  flex-direction: row;
  gap: 4px;
}
main {
  padding-top: 10px;
}
</style>

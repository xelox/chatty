<script lang="ts">
import { onDestroy } from 'svelte';
import { channels_store, type SchemaChannelList } from '../../stores/messages';
import { router_state } from '../../stores/router';
import Channel from './Channel.svelte';

let channels: SchemaChannelList = {};
const unsubscribe = channels_store.subscribe(channels_ => {
  channels = channels_;
});

onDestroy(() => {
  unsubscribe();
})
</script>

<main>
  {#each Object.values(channels) as channel_info }  
    <div class="channel_wrapper" class:active_channel={channel_info.id === $router_state.active_channel}>
      <Channel {channel_info}/>
    </div>
  {/each}
</main>

<style>
main{
  height: 100%;
}
.channel_wrapper {
  height: 100%;
  display: none;
}
.active_channel {
  display: block;
}
</style>

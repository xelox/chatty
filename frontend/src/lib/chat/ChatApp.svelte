<script lang="ts">
import { onDestroy } from 'svelte';
import { channels_store, type SchemaChannelList, active_channel } from '../../stores/messages';
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
    <div class="channel_wrapper" style="display: {$active_channel === channel_info.id ? "block" : "none"};">
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
}
</style>

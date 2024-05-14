<script lang="ts">
import { onDestroy } from 'svelte';
import { channels_store, type SchemaChannelList, active_channel } from '../../stores/messages';
import Channel from './Channel.svelte';
import { uuidv4 } from 'uuidv7';

let channels: SchemaChannelList = {};
const unsubscribe = channels_store.subscribe(channels_ => {
  channels = channels_;
});

const channels_test: string[] = [];
for (let i = 0; i < 10; i++) {
  const id = uuidv4();
  channels_test.push(id);
  channels[id] = {
    id: id,
    channel_name: id,
  }
}
let idx = 0;

$active_channel = channels_test[idx];

document.addEventListener("keypress", (e: KeyboardEvent) => {
  if (e.key == "`") {
    idx += 1; 
    if (idx >= channels_test.length) {
      idx = 0;
    }
    $active_channel = channels_test[idx];
  }
  return e;
})

onDestroy(() => {
  unsubscribe();
})
</script>

<main>
  {#each Object.values(channels) as channel_info }  
    <div class="channel_wrapper" style="display: {$active_channel == channel_info.id ? "block" : "none"};">
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

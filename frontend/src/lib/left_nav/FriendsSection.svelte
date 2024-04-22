<script lang="ts">
import {friend_list} from '../../stores/data'

fetch("/api/initial_data_request").then(response=>{
  response.json().then(json=>{
    console.log(json)
  }).catch(e=>console.error(e))
}).catch(e=>console.error(e))

</script>

<main>
  {#each $friend_list as friend_item}
    <div class="friend_item">
      {friend_item.display_name}
    </div>
  {/each}
</main>

<style>
.friend_item {
  /* border-bottom: 1px solid var(--overlay0); */
  font-family: "maple";
}
</style>

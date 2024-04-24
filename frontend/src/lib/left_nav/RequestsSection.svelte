<script lang="ts">
import { requests_manager, type RequestOptions } from "../../requests_manager";
import { pending_friends_in, pending_friends_out } from "../../stores/data";
const interact = (interaction: "cancel" | "accept" | "refuse", relation_id: string) => {
  const options: RequestOptions = {
    succeed_action: () => {
      if (interaction === "accept" || interaction === "refuse") {
        pending_friends_in.update(store => {
          delete store[relation_id];
          return store;
        })
      } else if (interaction === "cancel") {
        pending_friends_out.update(store => {
          delete store[relation_id];
          return store;
        })
      }
    },
    notify_fail: true,
  }
  requests_manager.post(`/api/friendship/${interaction}`, {relation_id}, options)
}
export let manipulate_path: (s: string)=>void;
let section: "inbound" | "outbound";
const set_section = (s: "inbound" | "outbound") => {
  section = s;
  manipulate_path(s);
}
set_section("inbound");
</script>

<main>
  <div class="section_selection_wrap">
    <div>
      <button on:click={()=>set_section("inbound")} style="color: {section === "inbound" ? "var(--yellow)" : ""}">Incoming</button>
      <button on:click={()=>set_section("outbound")} style="color: {section === "outbound" ? "var(--yellow)" : ""}">Outbound</button>
    </div>
  </div>
  {#if section === "inbound"}
  {#each Object.entries($pending_friends_in) as [relation_id, item]}
    <div class="friend_item">
      <span class="left"> <img src="" title="{item.display_name ?? item.unique_name}" alt=""> </span>
      <div class="right">
        <span class="display_name">{item.display_name ?? item.unique_name}</span>
        <div class="interations_wrap">
        <button on:click={()=>interact("accept", relation_id)}>Accept</button>
        <button on:click={()=>interact("refuse", relation_id)}>Refuse</button>
        </div>
      </div>
    </div>
  {/each}
  {:else if section === "outbound"}
  {#each Object.entries($pending_friends_out) as [relation_id, item]}
    <div class="friend_item">
      <span class="left"> <img src="" title="{item.display_name ?? item.unique_name}" alt=""> </span>
      <div class="right">
        <span class="display_name">{item.display_name ?? item.unique_name}</span>
        <div class="interations_wrap">
        <button on:click={()=>interact("cancel", relation_id)}>Cancel</button>
        </div>
      </div>
    </div>
  {/each}
  {/if}
</main>

<style>
.interations_wrap {
  display: flex;
  justify-content: right;
}
button:hover {
  text-decoration: underline;
}
.section_selection_wrap {
  padding: 10px;
  display: flex;
  justify-content: space-around;
  border-bottom: 1px solid var(--overlay0);
  margin-bottom: 10px;
}
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
</style>

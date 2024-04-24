<script lang="ts">
import { pending_friends_in, pending_friends_out } from "../../stores/data";
const interact = (interaction: "cancel" | "accept" | "refuse", relation_id: string) => {

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
  {#each $pending_friends_in as item}
    <div class="friend_item">
      <span class="left"> <img src="" title="{item.display_name ?? item.unique_name}" alt=""> </span>
      <div class="right">
        <span class="display_name">{item.display_name ?? item.unique_name}</span>
        <div class="interations_wrap">
        <button on:click={()=>interact("accept", item.relation_id)}>Accept</button>
        <button on:click={()=>interact("refuse", item.relation_id)}>Refuse</button>
        </div>
      </div>
    </div>
  {/each}
  {:else if section === "outbound"}
  {#each $pending_friends_out as item}
    <div class="friend_item">
      <span class="left"> <img src="" title="{item.display_name ?? item.unique_name}" alt=""> </span>
      <div class="right">
        <span class="display_name">{item.display_name ?? item.unique_name}</span>
        <div class="interations_wrap">
        <button on:click={()=>interact("cancel", item.relation_id)}>Cancel</button>
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
  display: flex;
  justify-content: space-around;
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
main {
}
</style>

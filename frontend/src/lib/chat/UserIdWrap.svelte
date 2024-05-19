<script lang="ts">
    import { onDestroy } from "svelte";
import users from "../../stores/users";
export let id: string;

let username: string = "Unknown";
const u = users.subscribe(store => {
  username = store[id]?.display_name ?? store[id]?.username ?? "Unknown";
});

onDestroy(u);
</script>

<span>{username}</span>

<style>
span{
  color: var(--blue);
  font-weight: bold;
}
</style>

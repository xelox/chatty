<script lang='ts'>
import { router } from "../../stores/router";

const pannels_list = {
  account: "Account",
  privacy: "Privacy",
  appearence: "Appearence",
  autio_video: "Autio & Video",
  notifications: "Notifications",
  keybinds: "Keybinds"
};
</script>

<main>
  {#each Object.entries(pannels_list) as [key, title]} 
    <button class="pannel_btn" on:click={()=>{ 
      router.route(`/app/settings/${key}`);
    }}>{title}</button>
  {/each}
</main>

<style>
.pannel_btn{
  cursor: pointer;
  text-align: left;
  font-family: 'Noto Sans';
  font-size: var(--size-large);
  padding: 4px;
  border-radius: 4px; 
  margin: 1px;
}
.pannel_btn:hover {
  background: var(--surface0);
}

main{
  display: flex;
  flex-direction: column;
  padding: 20px;
  background: var(--base);
  height: 100%;
}
</style>

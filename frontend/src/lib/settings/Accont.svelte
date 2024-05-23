<script lang='ts'>
import type { ChangeEventHandler } from "svelte/elements";
import { user_data } from "../../stores/data";
    import CropperTool from "./CropperTool.svelte";

let pfp_picker: HTMLInputElement;
let banner_picker: HTMLInputElement;
let pfp_preview: HTMLImageElement;
let banner_preview: HTMLImageElement;

const file_handler = (e: Event, prev: HTMLImageElement) => {
  const target = e.target as HTMLInputElement;
  const file = target.files![0];
  const reader = new FileReader();
  reader.onload = (e) => {
    const local_url = e.target?.result;
    if (typeof local_url !== 'string') return console.log('not a string');
    prev.src = local_url;
  }
  reader.readAsDataURL(file);
}

const pfp_handler: ChangeEventHandler<HTMLInputElement> = (e) => {
  file_handler(e, pfp_preview);
}

const banner_handler: ChangeEventHandler<HTMLInputElement> = (e) => {
  file_handler(e, banner_preview);
}
</script>

<main>
  <h1>Account Settings</h1>
  <div class="sections_wrap">
    <div class="">
      <p class='label'>Profile</p>
      <!-- svelte-ignore a11y-no-static-element-interactions -->
      <div class="profile_banner_wrap">
        <img class='banner_img' src="#" alt="" bind:this={banner_preview} />
        <!-- svelte-ignore a11y-click-events-have-key-events -->
        <div class="edit_hover" on:click={()=>{banner_picker.click()}}>
          <img class='edit_icon' src="/svg-files/Education/pencil.svg" alt=""/>
          <input type="file" accept="image/*" class='file_input' bind:this={banner_picker} on:change={banner_handler}>
        </div>
        <div class="profile_banner">
          <div class="pfp_wrap">
            <img bind:this={pfp_preview} class="pfp_img" alt='' src="#"/>
            <!-- svelte-ignore a11y-click-events-have-key-events -->
            <div class="edit_hover" on:click={()=>{pfp_picker.click()}}>
              <img class='edit_icon' src="/svg-files/Education/pencil.svg" alt=""/>
              <input type="file" accept="image/*" class='file_input' bind:this={pfp_picker} on:change={pfp_handler}>
            </div>
          </div>
        </div>
      </div>
    </div>
    <div class="field_wrap">
      <p class='label'>Username</p>
      <p class="ed">{$user_data?.username}</p>
    </div>
    <div class="field_wrap">
      <p class='label'>Display Name</p>
      <input type="text" class="ed" placeholder="Display Name">
    </div>
  </div>

  <CropperTool subject_src='/cat-portrait.jpg' round={false} aspect={{x: 30, y:7}} />
</main>

<style>
.pfp_img, .banner_img{
  width: 100%;
  aspect-ratio: 1/1;
  object-fit: cover;
  position: absolute;
}
.file_input {
  display: none;
}
.profile_banner_wrap {
  position: relative;
  overflow: hidden;
  border-radius: 4px;
}
.profile_banner {
  background: var(--surface0);
  width: 100%;
  aspect-ratio: 30/7;
}
.pfp_wrap {
  background: var(--mantle);
  border-radius: 100%;
  width: 90px;
  aspect-ratio: 1/1;
  border: 0;
  margin: 0;
  position: absolute;
  overflow: hidden;
  bottom: 8px;
  left: 8px;
}
.edit_hover {
  opacity: 0;
  transition: 200ms;
  position: relative;
  cursor: pointer;
  position: absolute;
  width: 100%;
  height: 100%;
  left: 0;
  top: 0;
  background: var(--overlay2);
}
.edit_hover:hover {
  opacity: 0.5;
}
.edit_icon {
  filter: invert(87%) sepia(6%) saturate(987%) hue-rotate(192deg) brightness(98%) contrast(94%);
  width: 40px;
  height: 40px;
  position: absolute;
  top: 50%; left: 50%;
  transform: translate(-50%, -50%);
}
.field_wrap {
  display: flex;
  flex-direction: column;
  align-content: center;
  justify-content: center;
}
.field_wrap {
  padding-bottom: 20px;
  margin-top: 20px;
  border-bottom: 1px solid var(--surface0);
}
.label {
  margin-bottom: 4px;
  font-family: 'Maple';
  font-weight: bold;
  font-size: var(--size-large)
}
.ed{
  font-size: var(--size-normal);
  padding: 8px;
  border-radius: 4px;
  background: var(--mantle);
}
.sections_wrap {
  display: flex;
  flex-direction: column;
}
h1 {
  margin-bottom: 40px;
}
</style>

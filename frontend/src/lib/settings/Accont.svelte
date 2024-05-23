<script lang='ts'>
import type { ChangeEventHandler } from "svelte/elements";
import { user_data } from "../../stores/data";
    import CropperTool from "./CropperTool.svelte";

let pfp_picker: HTMLInputElement;
let banner_picker: HTMLInputElement;
let pfp_preview: HTMLImageElement;
let banner_preview: HTMLImageElement;

const changes: {
  new_pfp: string | null
  new_banner: string | null
} = {
  new_pfp: null,
  new_banner: null,
}

let tmp_pfp_input: string | null;
let tmp_banner_input: string | null;

const file_handler = async (e: Event) => {
  return new Promise<string>((res, rej) => {
    const target = e.target as HTMLInputElement;
    const file = target.files![0];
    const reader = new FileReader();
    reader.onload = (e) => {
      const local_url = e.target?.result;
      if (typeof local_url !== 'string') return rej();
      res(local_url);
    }
    reader.readAsDataURL(file);
  })
}

const pfp_handler: ChangeEventHandler<HTMLInputElement> = (e) => {
  file_handler(e).then(tmp => {
    tmp_pfp_input = tmp;
  });
}

const banner_handler: ChangeEventHandler<HTMLInputElement> = (e) => {
  file_handler(e).then(tmp => {
    tmp_banner_input = tmp;
  });
}

const submit_pfp = (output: string) => {
  changes.new_pfp = output;
  tmp_pfp_input = null;
  pfp_preview.src = output;
}

const submit_banner = (output: string) => {
  changes.new_banner = output;
  tmp_banner_input = null;
  banner_preview.src = output;
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

  {#if tmp_pfp_input}
    <CropperTool subject_src={tmp_pfp_input} round={true} aspect={{x: 1, y: 1}} on_submit={submit_pfp}/>
  {/if}

  {#if tmp_banner_input}
    <CropperTool subject_src={tmp_banner_input} round={false} aspect={{x: 30, y: 7}} on_submit={submit_banner}/>
  {/if}
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
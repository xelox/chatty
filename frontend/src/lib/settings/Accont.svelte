<script lang='ts'>
import { requests_manager, type RequestOptions } from "../../requests_manager";
import { user_data, type SchemaUserInfo } from "../../stores/data";
import Button from "../components/Button.svelte";
import CropperTool from "./CropperTool.svelte";

let pfp_picker: HTMLInputElement;
let banner_picker: HTMLInputElement;

const changes: {
  display_name?: string,
  about_me?: string,
  status?: string,
} = {};

const limits = <const>{
  display_name: 25,
  about_me: 500,
  status: 50,
};

$: over_limit = (key: keyof typeof limits) => {
  return (changes[key]?.length || 0) > limits[key];
}

const pictures: {
  pfp?: {
    url: string,
    blob: Blob
  },
  banner?: {
    url: string,
    blob: Blob,
  }
} = {}

const tmp: {
  pfp?: string,
  banner?: string
} = {}

const image_file_handler = async (e: Event, key: 'pfp' | 'banner') => {
  const target = e.target as HTMLInputElement;
  const file = target.files![0];
  const reader = new FileReader();
  reader.onload = (e) => {
    const local_url = e.target?.result;
    // WARNING: Possibly neccessary to properly handle this error.
    if (typeof local_url !== 'string') return console.error("Something went quite wrong.");
    tmp[key] = local_url;
  }
  reader.readAsDataURL(file);
}

let there_are_changes = true;
$: if(changes || pictures) {
  const any_changes = (): boolean => {
    for (const change of Object.values(pictures)) {
      if (change) return true; 
    }
    for (const [key_, change] of Object.entries(changes)) {
      const key = key_ as keyof typeof changes;
      if (over_limit(key)) return false;
      if (change === $user_data![key]) continue
      if(change !== undefined) return true;
    }
    return false;
  }
  there_are_changes = any_changes();
}

const reset = () => {
  pictures.pfp = undefined;
  pictures.banner = undefined;
  changes.display_name = $user_data?.display_name ?? undefined;
  changes.about_me = $user_data?.about_me ?? undefined;
}

reset();

const save = () => {
  const opts: RequestOptions = {
    succeed_action: (new_data) => {
      const data: SchemaUserInfo = JSON.parse(new_data); 
      user_data.set(data);
      reset();
    },
  }
  const form = new FormData();
  for(const [key, data] of Object.entries(pictures)) {
    form.append(key, data.blob);
  }
  for(const [key, change] of Object.entries(changes)) {
    if (change) {
      console.log(key, change);
      form.append(key, change);
    }
  }
  requests_manager.post_form('/api/update_profile', form, opts);
}
</script>

<main>
  <h1>Account Settings</h1>
  <div class="sections_wrap">
    <div class="field_wrap">
      <p class='label'>Profile</p>
      <!-- svelte-ignore a11y-no-static-element-interactions -->
      <div class="profile_banner_wrap">
        <img class='banner_img' src={pictures.banner?.url ?? ($user_data?.has_banner ? `/media/banner/${$user_data?.id}.png` : '#')} alt="" />
        <!-- svelte-ignore a11y-click-events-have-key-events -->
        <div class="edit_hover" on:click={()=>{banner_picker.click()}}>
          <img class='edit_icon' src="/svg-files/Education/pencil.svg" alt=""/>
          <input type="file" accept="image/*" class='file_input' bind:this={banner_picker} on:change={e=>image_file_handler(e, 'banner')}>
        </div>
        <div class="profile_banner">
          <div class="pfp_wrap">
            <img src={pictures.pfp?.url ?? ($user_data?.has_pfp ? `/media/pfp/${$user_data?.id}.png` : '#')} class="pfp_img" alt=''/>
            <!-- svelte-ignore a11y-click-events-have-key-events -->
            <div class="edit_hover" on:click={()=>{pfp_picker.click()}}>
              <img class='edit_icon' src="/svg-files/Education/pencil.svg" alt=""/>
              <input type="file" accept="image/*" class='file_input' bind:this={pfp_picker} on:change={e=>image_file_handler(e, 'pfp')}>
            </div>
          </div>
        </div>
      </div>
    </div>

    <div class="field_wrap">
      <p class='label'>Display Name</p>
      <div class="input_wrap">
        <input type="text" class="ed" placeholder="Display Name" bind:value={changes.display_name}>
        <span class="length_limit" class:over_limit={over_limit('display_name')}>
          {changes.display_name?.length || 0}/{limits.display_name}
        </span>
      </div>
    </div>

    <div class="field_wrap">
      <p class='label'>Status</p>
      <div class="input_wrap">
        <input type="text" class="ed" placeholder="Custom Status" bind:value={changes.status}>
        <span class="length_limit" class:over_limit={over_limit('status')}>
          {changes.status?.length || 0}/{limits.status}
        </span>
      </div>
    </div>

    <div class="field_wrap">
      <p class="label">About Me</p>
      <div class="input_wrap">
        <textarea class='ed' rows="4" bind:value={changes.about_me}></textarea>
        <span class="length_limit" class:over_limit={over_limit('about_me')}>
          {changes.about_me?.length || 0}/{limits.about_me}
        </span>
      </div>
    </div>
  </div>

  {#if tmp.pfp}
    <CropperTool input_src={tmp.pfp} 
    round={true} 
    output_res={{x: 600, y: 600}} 
    on_submit={output => {tmp.pfp = undefined; pictures.pfp = output}}/>
  {/if}

  {#if tmp.banner}
    <CropperTool input_src={tmp.banner} 
    round={false} 
    output_res={{x: 1200, y: 280}} 
    on_submit={output => {tmp.banner = undefined; pictures.banner = output}}/>
  {/if}

  {#if there_are_changes}
    <div class="changes_to_commit_wrap">
      <p> You have un-saved changes! </p>
      <div class="buttons_wrap">
        <Button bg={'red'} action={reset}>Reset</Button>
        <Button bg={'green'} action={save}>Save</Button>
      </div>
    </div>
  {/if}
</main>

<style>
.pfp_img, .banner_img{
  width: 100%;
  object-fit: cover;
  position: absolute;
  border-radius: 4px;
}
.file_input {
  display: none;
}
.profile_banner_wrap {
  position: relative;
  /* overflow: hidden; */
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
  border: 4px solid var(--base);
  margin: 0;
  position: absolute;
  overflow: hidden;
  bottom: -14px; left: -14px;
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
  padding-bottom: 20px;
  margin-top: 20px;
  border-bottom: 1px solid var(--surface0);
}
.label {
  margin-bottom: 4px;
  font-weight: bold;
  font-size: var(--size-large)
}
.ed{
  font-size: var(--size-normal);
  flex: 1;
  outline: none;
}
.sections_wrap {
  display: flex;
  flex-direction: column;
}
h1 {
  margin-bottom: 40px;
}
.changes_to_commit_wrap {
  background: var(--crust);
  position: fixed;
  bottom: 20px;
  left: 50%;
  transform: translateX(-50%);
  display: flex;
  gap: 100px;
  padding: 8px 30px 8px 30px;
  border-radius: 4px;
  border: 1px solid var(--overlay0);
}
.changes_to_commit_wrap > p {
  margin-top: auto;
  margin-bottom:  auto;
}
.input_wrap {
  padding: 8px 16px;
  border-radius: 4px;
  background: var(--mantle);
  position: relative;
  display: flex;
  flex-direction: row;
}
.over_limit {
  color: var(--red);
}

</style>

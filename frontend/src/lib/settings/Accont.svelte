<script lang='ts'>
import { user_data } from "../../stores/data";
</script>

<main>
  <h1>Account Settings</h1>
  <div class="sections_wrap">
    <div class="">
      <p class='label'>Profile</p>
      <div class="profile_banner_wrap">
        <div class="edit_hover">
          <img class='edit_icon' src="/svg-files/Education/pencil.svg" alt=""/>
        </div>
        <div class="profile_banner">
          <div class="pfp_wrap">
            <div class="pfp"></div>
            <div class="edit_hover">
              <img class='edit_icon' src="/svg-files/Education/pencil.svg" alt=""/>
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
</main>

<style>
.profile_banner_wrap {
  position: relative;
  overflow: hidden;
  border-radius: 4px;
}
.profile_banner {
  background: var(--surface0);
  height: 140px;
  padding: 8px;
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

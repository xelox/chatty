<script lang='ts'>
import { user_data } from "../../stores/data";
</script>

<main>
  <h1>Account Settings</h1>
  <div class="sections_wrap">
    <div class="field_wrap">
      <p class='label'>Profile Pictrue</p>
      <div class="pfp_wrap">
        <img class='pfp_img' src="{$user_data?.pfp_url}" alt="" title="pfp">
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
.pfp_wrap {
  width: max-content;
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
  border-bottom: 1px solid var(--overlay0);
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
img {
  background: var(--overlay0);
  border-radius: 100%;
  width: 100px;
  aspect-ratio: 1/1;
  border: 0;
  margin: 0;
}
h1 {
  margin-bottom: 40px;
}
</style>

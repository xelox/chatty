<script lang='ts'>
import { user_data } from "../../stores/data";
</script>

<main>
  <h1>Account Settings</h1>
  <div class="user_style_elements_wrap">
    <div class="textual_wrap">
      <div class="field_wrap">
        <p class='label'>Username</p>
        <p class="ed">{$user_data?.username}</p>
      </div>
      <div class="field_wrap">
        <p class='label'>Display Name</p>
        <input type="text" class="ed" placeholder="Display Name">
      </div>
    </div>
    <div class="pfp_wrap">
      <img class='pfp_img' src="{$user_data?.pfp_url}" alt="" title="pfp">
    </div>
  </div>
</main>

<style>
.field_wrap {
  padding-bottom: 20px;
  margin-top: 20px;
  border-bottom: 1px solid var(--overlay0);
}
.field_wrap > .label {
  margin-bottom: 4px;
}
.ed{
  padding: 8px;
  border-radius: 4px;
  background: var(--base);
  width: 200px;
}
.user_style_elements_wrap {
  display: flex;
  gap: 4px;
}
.user_style_elements_wrap > div {
  flex: 1;
}
img {
  float: right;
  background: var(--overlay0);
  border-radius: 100%;
  width: 100px;
  aspect-ratio: 1/1;
  border: 0;
  margin: 0;
}
</style>

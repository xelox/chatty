<script lang='ts'>
import type { SchemaMessage } from "../../stores/messages";
export let message: SchemaMessage;
</script>

<main class="{message.is_sent ? '' : 'unsent'}">
  <p class="sender_tag">
    {message.sender_id} 
  </p>
  <p class="content">
    {message.content}
  </p>
</main>

<style>
main {
  padding: 4px;
}
.unsent {
  opacity: 0.8;
}
</style>

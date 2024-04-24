<script lang='ts'>
  import type { Notification } from "../../stores/inbox";
  export let n: Notification;
</script>
<main>
  {n.content}
</main>

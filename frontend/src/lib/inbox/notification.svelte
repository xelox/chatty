<script lang='ts'>
  import type { notification } from "../../stores/inbox";
  export let n: notification;
</script>
<main>
  {n.content}
</main>

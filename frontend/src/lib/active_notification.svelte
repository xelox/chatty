<script lang="ts">
import type { Notification } from "../stores/inbox";
type ShortlyLivedNotification = Notification & {
  render_ts?: number;
}

const TTLms = 1_000; // Alloted time to show the active notification (in miliseconds)
let notification_queue: ShortlyLivedNotification[] = [];
let active_notification: ShortlyLivedNotification | null = null;
let elapsed_bar: number = TTLms;

window.addEventListener('C_E_notification', (event) => {
  const new_notification = (event as CustomEvent).detail as Notification;
  notification_queue.push(new_notification);
  if(active_notification === null && notification_queue.length === 1) {
    active_notification = notification_queue.shift()!;
    active_notification.render_ts = Number(new Date());
    render_update_callback();
  }
});

const render_update_callback = () => {
  if (active_notification === null) return;
  const ts_now = Number(new Date());
  elapsed_bar = ts_now - active_notification.render_ts!;
  if (elapsed_bar >= TTLms) {
    elapsed_bar = TTLms;
    active_notification = notification_queue.shift() ?? null;
    if (active_notification !== null) active_notification.render_ts = Number(new Date());
  }
  requestAnimationFrame(render_update_callback);
}

</script>

{#if active_notification !== null}
<main>
    <span class="elapsed_bar" style="width: {elapsed_bar / TTLms * 100}%"></span>
    <p class="source_p">{active_notification?.source}</p>
    <p>{active_notification?.content}</p>
</main>
{/if}

<style>
.source_p {
  color: var(--sky);
  font-size: var(--size-normal);
}
.elapsed_bar {
  display: block;
  height: 1px;
  width: 12%;
  background: var(--peach);
  margin-bottom: 8px;
}
main>p {
  font-size: var(--size-small);
  margin: 0;
}
main{
  display: block;
  position: fixed;
  top: 20px;
  left: 50%;
  transform: translate(-50%, 0);
  background: var(--crust);
  padding: 4px;
  border-radius: 4px;
  border: 1px solid var(--overlay0);
  width: 70vw;
  z-index: 10;
}
</style>

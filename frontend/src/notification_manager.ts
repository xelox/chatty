import inbox, { type notification } from './stores/inbox'
class NotificationManager {
  public notify(n: notification, group: 'dms' | 'guilds' | 'system') {
    inbox.update(i => {
      i[group].push(n);
      return i;
    });
    const event = new CustomEvent('C_E_notification', {detail: n});
    window.dispatchEvent(event);
  }
}

const nm = new NotificationManager();
export default nm;

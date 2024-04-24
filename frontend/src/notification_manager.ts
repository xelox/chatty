import inbox, { type Notification } from './stores/inbox'
class NotificationManager {
  public notify(n: Notification, group: 'dms' | 'guilds' | 'system') {
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

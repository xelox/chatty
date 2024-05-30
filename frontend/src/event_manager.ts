import { uuidv4 } from "uuidv7";

type EventKey = {
  action: "notification",
} | {
  action: "message_add",
  channel_id: string,
} | {
  action: "message_update",
  message_id: string,
} | {
  action: "message_delete",
  message_id: string,
} | {
  action: "channel_add"
} | {
  action: "channel_delete"
} | {
  action: "channel_update"
}

type SubscriptionId = string;
type Listener = (...args: any[]) => void;
type SubscriptionPool = {[id: SubscriptionId]: Listener};

class EventManager {
  private events = new Map<string, SubscriptionPool>();

  public subscribe(key: EventKey, callback: Listener) {
    const key_str = Object.values(key).join('.');
    const new_subscription_id: SubscriptionId = uuidv4();

    const pool = this.events.get(key_str);
    if (pool === undefined) {
      const new_pool: SubscriptionPool = {};
      new_pool[new_subscription_id] = callback;
      this.events.set(key_str, new_pool);
      return () => {
        delete new_pool[new_subscription_id];
      }
    } else {
      pool[new_subscription_id] = callback;
      this.events.set(key_str, pool);
      return () => {
        delete pool[new_subscription_id];
      }
    }
  }

  public dispatch = (key: EventKey, ...args: any[]) => {
    const key_str = Object.values(key).join('.');
    const pool = this.events.get(key_str);
    if (!pool) return;
    for ( const callback of Object.values(pool)) {
      callback(...args);
    }
  }
}

export default new EventManager();

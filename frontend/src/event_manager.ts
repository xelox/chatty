import { uuidv4 } from "uuidv7";

export type EventTag = "notification" 
| "message_update" 
| "message_delete" 
| "message_add" 

| "channel_update"
| "channel_delete"
| "channel_add"

type SubscriptionId = string;
type Listener = (...args: any[]) => void;
type SubscriptionPool = {[id: SubscriptionId]: Listener};

class EventManager {
  private events = new Map<EventTag, SubscriptionPool>();

  public subscribe(tag: EventTag, callback: Listener) {
    const new_subscription_id: SubscriptionId = uuidv4();

    const pool = this.events.get(tag);
    if (pool === undefined) {
      const new_pool: SubscriptionPool = {};
      new_pool[new_subscription_id] = callback;
      this.events.set(tag, new_pool);
      return () => {
        delete new_pool[new_subscription_id];
      }
    } else {
      pool[new_subscription_id] = callback;
      this.events.set(tag, pool);
      return () => {
        delete pool[new_subscription_id];
      }
    }
  }

  public subscribe_multiple(tags: EventTag[], callback: Listener) {
    const subscribtions: (() => void)[] = [];
    for( const tag of tags ) {
      subscribtions.push(
        this.subscribe(tag, callback)
      );
    }
    return subscribtions;
  }

  public dispatch = (tag: EventTag, ...args: any[]) => {
    const pool = this.events.get(tag);
    if (!pool) return;
    for ( const callback of Object.values(pool)) {
      callback(...args);
    }
  }
}

export default new EventManager();

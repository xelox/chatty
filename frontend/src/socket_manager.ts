import notification_manager from "./notification_manager";
import {friend_list, pending_friends_in, pending_friends_out, user_data, type SchemaPeerList } from "./stores/data";
import type { Notification } from "./stores/inbox";
class SocketManager {
  private socket: WebSocket | null = null;

  public initialize_client = async (): Promise<boolean> => {
    type response_schema = {
      user_info: {
        id: string,
        username: string,
        display_name: string | null,
      },
      relations: {
        relation:{
          id: string,
          a: string,
          b: string,
          sender: string,
          accepted: boolean,
        }
        user: {
          id: string,
          username: string,
          display_name: string | null,
        }
      }[]
    }

    try {
      const response = await fetch("http://localhost:8080/api/initial_data_request")
      const json = await response.json() as response_schema;

      user_data.set(json.user_info);
      
      const friends_list_: SchemaPeerList = {}
      const pending_friends_out_: SchemaPeerList = {}
      const pending_friends_in_: SchemaPeerList = {}

      for (const item of json.relations) {
        if (item.relation.accepted) {
          friends_list_[item.relation.id] = {...item.user, relation_id: item.relation.id}
        } else if (item.relation.sender === json.user_info.id) {
          pending_friends_out_[item.relation.id] = {...item.user, relation_id: item.relation.id}
        } else {
          pending_friends_in_[item.relation.id] = {...item.user, relation_id: item.relation.id}
        }
      }

      friend_list.set(friends_list_);
      pending_friends_in.set(pending_friends_in_);
      pending_friends_out.set(pending_friends_out_);

      console.log(json)
    } catch (err) {
      // TODO: *maybe* more error handeling could be nicer here?
      let n: Notification = {
        ts: Number(new Date()),
        content: "Failed to initialize",
        source: "System",
      };
      notification_manager.notify(n, "system");

      return false;
    }

    this.socket = new WebSocket("ws://localhost:8080/ws");
    this.socket.onopen = this.onopen;
    this.socket.onmessage = this.onmessage;
    this.socket.onclose = this.onclose;
    this.socket.onerror = this.onerror;

    return true;
  }

  private onopen = () => {
    notification_manager.notify({
      ts: Number(new Date()),
      content: 'socket connection established',
      source: 'System'
    }, 'system');
  }

  private onmessage = (e: MessageEvent) => {
    notification_manager.notify({
      ts: Number(new Date()),
      content: `socket message: ${e.data}`,
      source: 'System'
    }, 'system');
  }
  
  private onclose = () => {
    notification_manager.notify({
      ts: Number(new Date()),
      content: 'socket connection closed',
      source: 'System'
    }, 'system');
  }

  private onerror = (e: Event) => {
    notification_manager.notify({
      ts: Number(new Date()),
      content: `socket error: ${e}`,
      source: 'System'
    }, 'system');
  }
}

const socket_manager = new SocketManager();
export default socket_manager;

import event_manager from "./event_manager";
import notification_manager from "./notification_manager";
import { requests_manager } from "./requests_manager";
import {friend_list, pending_friends_in, pending_friends_out, user_data, type SchemaUserInfo } from "./stores/data";
import {type SchemaMessage } from "./stores/messages";
import users from "./stores/users";

class SocketManager {
  private socket: WebSocket | null = null;

  public initialize_client = async (): Promise<void> => {
    const data = await requests_manager.init_client();
    
    user_data.set(data.user);
    users.update_peers(data.known_users);
    friend_list.set(data.friends);
    pending_friends_in.set(data.pending_in);
    pending_friends_out.set(data.pending_out);

    this.socket = new WebSocket("ws://localhost:8080/ws");
    this.socket.onopen = this.onopen;
    this.socket.onmessage = this.onmessage;
    this.socket.onclose = this.onclose;
    this.socket.onerror = this.onerror;
  }

  private onopen = () => {
    notification_manager.notify({
      ts: Number(new Date()),
      content: 'socket connection established',
      source: 'System'
    }, 'system');
  }

  private onmessage = (e: MessageEvent) => {
    const e_json: { signals: any[] } = JSON.parse(e.data);

    for (const s of e_json.signals) {
      if (s.message) this.handle_message(s.message);
      if (s.profile_patch) this.handle_profile_patch(s.profile_patch);
    }
  }

  private handle_message = (input: ['send' | 'delete' | 'patch', SchemaMessage]) => {
    const [action, message] = input;
    if (action === "send") return event_manager.dispatch({action: "message_add", channel_id: message.channel_id}, message);
    if (action === "delete") return event_manager.dispatch({action: "message_delete", message_id: message.id}, message);
    if (action === "patch") return event_manager.dispatch({action: "message_update", message_id: message.id}, message);
  }

  private handle_profile_patch = (input: SchemaUserInfo) => {
    users.update_peers([input]);
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

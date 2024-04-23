import notification_manager from "./notification_manager";
import { user_data, type schema_peer, type schema_user_info } from "./stores/data";
import type { notification } from "./stores/inbox";
class SocketManager {
  private socket: WebSocket | null = null;

  public initialize_client = async (): Promise<boolean> => {
    type response_schema = {
      user_info: schema_user_info,
      relations: schema_peer[],
    }

    try {
      const response = await fetch("/api/initial_data_request")
      const json_response = await response.json();
      const json = json_response as response_schema;
      user_data.set(json_response.user_info);
      console.log(json)
    } catch (err) {
      // TODO *maybe* propper error handeling could be nicer here?
      let n: notification = {
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
      source: 'system'
    }, 'system');
  }

  private onmessage = (e: MessageEvent) => {
    notification_manager.notify({
      ts: Number(new Date()),
      content: `socket message: ${e.data}`,
      source: 'system'
    }, 'system');
  }
  
  private onclose = () => {
    notification_manager.notify({
      ts: Number(new Date()),
      content: 'socket connection closed',
      source: 'system'
    }, 'system');
  }

  private onerror = (e: Event) => {
    notification_manager.notify({
      ts: Number(new Date()),
      content: `socket error: ${e}`,
      source: 'system'
    }, 'system');
  }
}

const socket_manager = new SocketManager();
export default socket_manager;

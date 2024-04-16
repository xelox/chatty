import notification_manager from "./notification_manager";
class SocketManager {
  private socket = new WebSocket("ws://localhost:8080/ws");
  constructor(){
    this.socket.onopen = this.onopen;
    this.socket.onmessage = this.onmessage;
    this.socket.onclose = this.onclose;
    this.socket.onerror = this.onerror;
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

const sm = new SocketManager();
export default sm;

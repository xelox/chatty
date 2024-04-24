import { writable } from "svelte/store";
export type Notification = {
  ts: number,
  content: string,
  source: string,
}
export type inbox_store = {
  dms: Notification[],
  guilds: Notification[],
  system: Notification[],
}

export default writable<inbox_store>({
  dms: [], guilds: [], system: []
})

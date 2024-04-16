import { writable } from "svelte/store";
export type notification = {
  ts: number,
  content: string,
  source: string,
}
export type inbox_store = {
  dms: notification[],
  guilds: notification[],
  system: notification[],
}

export default writable<inbox_store>({
  dms: [], guilds: [], system: []
})

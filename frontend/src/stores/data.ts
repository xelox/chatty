import { writable } from "svelte/store"

export type schema_user_data = {
  username: string,
};

export type schema_channel = {
  channel_id: number,
  channel_name: string,
  last_message: string,
};

export type schema_peer = {
  unique_name: string,
  display_name: string,
  pfp_resource: string,
  status: string,
  last_message: string,
};

export const user_data = writable<schema_user_data | null>(null);
export const friend_list = writable<schema_peer[]>([]);

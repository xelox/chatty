import { writable } from "svelte/store"

export type schema_user_info = {
  unique_name: string,
  pfp_url?: string,
  dislplay_name: string | null,
  status?: string,
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

export const user_data = writable<schema_user_info | null>(null);
export const friend_list = writable<schema_peer[]>([]);

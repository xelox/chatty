import { writable } from "svelte/store"

export type schema_user_info = {
  unique_name: string,
  pfp_url?: string,
  display_name: string | null,
  status?: string,
};

export type schema_channel = {
  channel_id: number,
  channel_name: string,
  last_message: string,
};

export type schema_peer = schema_user_info & { last_message?: string };

export const user_data = writable<schema_user_info | null>(null);
export const friend_list = writable<schema_peer[]>([
  {
    unique_name: "pablo",
    display_name: null,
    pfp_url: undefined, 
    last_message: undefined,
  },
  {
    unique_name: "mark",
    display_name: "Markus Primus",
    pfp_url: undefined, 
    last_message: "What's cooking fam",
  }
]);


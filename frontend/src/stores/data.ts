import { writable } from "svelte/store"

export type SchemaUserInfo = {
  unique_name: string,
  pfp_url?: string,
  display_name: string | null,
  status?: string,
};

export type SchemaChannel = {
  channel_id: number,
  channel_name: string,
  last_message: string,
};

export type SchemaPeer = SchemaUserInfo & { 
  last_message?: string,
  relation_id: string,
};

export const user_data = writable<SchemaUserInfo | null>(null);
export const friend_list = writable<SchemaPeer[]>([]);

export type SchemaPeerList = {[key: string]: SchemaPeer};

export const pending_friends_out = writable<SchemaPeerList>({});
export const pending_friends_in = writable<SchemaPeerList>({});


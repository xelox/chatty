import { writable } from "svelte/store"

export type SchemaUserInfo = {
  id: string,
  username: string,
  pfp_url?: string,
  display_name: string | null,
  status?: string,
};

export type SchemaPeer = SchemaUserInfo & { 
  last_message?: string,
  relation_id: string | null,
};

export type SchemaPeerList = {[key: string]: SchemaPeer};

export const user_data = writable<SchemaUserInfo | null>({
  id: "88f38804-b19a-490b-8be8-d40b44245ee4",
  username: "xelox",
  display_name: "Big Xelos",
});

export const friend_list = writable<SchemaPeerList>({});
export const pending_friends_out = writable<SchemaPeerList>({});
export const pending_friends_in = writable<SchemaPeerList>({});

export const erase = () => {
  user_data.set(null);
  friend_list.set({})
  pending_friends_out.set({})
  pending_friends_in.set({})
}

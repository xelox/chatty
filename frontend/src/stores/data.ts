import { writable } from "svelte/store"

export type SchemaInitData = {
  user: SchemaUserInfo,
  known_users: SchemaPeer[],
  pending_in: SchemaPeerList, 
  pending_out: SchemaPeerList,
  friends: SchemaPeerList,
}

export type SchemaUserInfo = {
  id: string,
  username: string,
  has_pfp?: boolean,
  has_banner?: boolean,
  display_name: string | null,
  about_me: string,
  custom_status?: string,
};

export type SchemaPeer = SchemaUserInfo & { 
  last_message?: string,
  relation_id?: string,
};

export type SchemaPeerList = {[key: string]: SchemaPeer};

export const user_data = writable<SchemaUserInfo | null>(null);
export const friend_list = writable<SchemaPeerList>({});
export const pending_friends_out = writable<SchemaPeerList>({});
export const pending_friends_in = writable<SchemaPeerList>({});

export const erase = () => {
  user_data.set(null);
  friend_list.set({})
  pending_friends_out.set({})
  pending_friends_in.set({})
}

import { writable } from "svelte/store"

export type SchemaUserInfo = {
  id: string,
  username: string,
  pfp_url?: string,
  display_name: string | null,
  status?: string,
};

export type SchemaChannel = {
  channel_id: string,
  channel_name: string,
  messages: {
    [message_id: string]: SchemaMessage 
  }
};

export type SchemaMessage = {
  id: string,
  sender_id: string, 
  channel_id: string,
  content: string,
  is_sent?: boolean
}

export type SchemaChannelList = {
  [key: string]: SchemaChannel,
}

export type SchemaPeer = SchemaUserInfo & { 
  last_message?: string,
  relation_id: string,
};

export type SchemaPeerList = {[key: string]: SchemaPeer};

export const user_data = writable<SchemaUserInfo | null>(null);
export const friend_list = writable<SchemaPeerList>({});
export const pending_friends_out = writable<SchemaPeerList>({});
export const pending_friends_in = writable<SchemaPeerList>({});
export const active_channel = writable<SchemaChannel | null>(null);

export const erase = () => {
  user_data.set(null);
  friend_list.set({})
  pending_friends_out.set({})
  pending_friends_in.set({})
}

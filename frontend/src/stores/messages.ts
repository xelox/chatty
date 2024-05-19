import { writable, derived } from "svelte/store";

export type SchemaUpMessage = {
  sender_id: string,
  channel_id: string,
  content: string,
}
export type SchemaMessage = SchemaUpMessage & {
  id: string,
  sent_at: number,
}
export type MessageGroup = {
  sender_id: string,
  group_ts_start: number,
  group_ts_end: number,
  messages: SchemaMessageList
}

export type SchemaMessageList = {
  [message_id: string]: SchemaMessage,
}

export type SchemaChannel = {
  id: string,
  channel_name: string,
};

export type SchemaChannelList = {
  [key: string]: SchemaChannel,
}

const create_channels_store = () => {
  const store = writable<SchemaChannelList>({});
  const { update, subscribe } = store;
  const add_channel = (channel: SchemaChannel) => {
    update(channels => {
      channels[channel.id] = channel;
      return channels;
    });
  }
  const remove_channel = ((channel: SchemaChannel) => {
    update(channels => {
      delete channels[channel.id];
      return channels;
    })
  });
  const update_channel = add_channel;

  return {
    add_channel, update_channel, remove_channel, subscribe, store
  };
}

export const channels_store = create_channels_store();
export const active_channel = writable<string | null>("12567514614595584");

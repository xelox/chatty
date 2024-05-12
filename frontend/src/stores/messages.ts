import { writable, derived } from "svelte/store";

export type SchemaMessage = {
  id: string,
  sender_id: string, 
  channel_id: string,
  content: string,
  is_sent?: boolean
}

export type SchemaMessageList = {
  [message_id: string]: SchemaMessage,
}

export type SchemaChannel = {
  id: string,
  channel_name: string,
};

export type SchemaChannelList = {
  [channel_id: string]: SchemaChannel,
}

const create_channels_store = () => {
  const { update, subscribe } = writable<SchemaChannelList>({});
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
    add_channel, update_channel, remove_channel, subscribe
  };
}

export const channels_store = create_channels_store();
export const active_channel = writable<string | null>(null);
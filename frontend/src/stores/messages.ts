import { writable } from "svelte/store";

export type SchemaMessage = {
  id: string,
  sender_id: string, 
  channel_id: string,
  content: string,
  is_sent?: boolean
}

export type SchemaChannel = {
  channel_id: string,
  channel_name: string,
  messages: {
    [message_id: string]: SchemaMessage 
  }
};

export type SchemaChannelList = {
  [channel_id: string]: SchemaChannel,
}

const create_active_channel_store = () => {
  const { set, subscribe } = writable<string | null>(null);

  const active_channel_set = (id: string) => {
    set(id);
  }

  return {
    subscribe,
    active_channel_set,
  }
}

const create_channel_store = () => {
  const { update, subscribe } = writable<SchemaChannelList>({});
  const insert_message = (message: SchemaMessage) => {
    update(channels => {
      channels[message.channel_id].messages[message.id] = message;
      return channels;
    });
  }
  const delete_message = (message: SchemaMessage) => {
    update(channels => {
      delete channels[message.channel_id].messages[message.id];
      return channels;
    });
  }
  const update_message = insert_message;
  const remove_channel = (channel: SchemaChannel) => {
    update(channels => {
      delete channels[channel.channel_id];
      return channels;
    });
  }

  return {
    insert_message,
    delete_message,
    update_message,
    remove_channel,
  }
}

export const active_channel = create_active_channel_store();

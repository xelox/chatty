import navaid, { type Params } from 'navaid';
import { readable, type Readable } from 'svelte/store';
import { channels_store, type SchemaChannelList } from './messages';

const r = navaid('app', (url) => {
  console.log('Used undefined route:', url);
});

const MAIN_SECTIONS = <const>["auth", "chat", "settings", "friend_req_tool"];
type MainSectionsSections = typeof MAIN_SECTIONS[number]; 

const CHAT_NAV_SECTIONS = <const>["friends", "guilds", "requests"];
type ChatNavSections = typeof CHAT_NAV_SECTIONS[number];

export const SETTINGS_NAV_SECTIONS = <const>{
  account: "Account",
  privacy: "Privacy",
  appearence: "Appearence",
  autio_video: "Autio & Video",
  notifications: "Notifications",
  keybinds: "Keybinds"
};
type SettingsNavSections = keyof typeof SETTINGS_NAV_SECTIONS;

type RouterState = {
  main_section: MainSectionsSections,
  chat_nav_section: ChatNavSections,
  settings_nav_section: SettingsNavSections,
  show_left_nav: boolean,
  active_channel_for: {
    friends: string | null,
    guilds: string | null,
  }
  active_channel: string | null,
};

const def: RouterState = {
  main_section: "chat",
  chat_nav_section: "friends",
  settings_nav_section: "account",
  show_left_nav: true,
  active_channel_for: {
    friends: null,
    guilds: null
  },
  active_channel: null,
};


let update: (state: Partial<RouterState>) => void;

export const router_state: Readable<RouterState> = readable(def, (_, u) => {
  update = (new_state: Partial<RouterState>) => {
    u((state: RouterState) => {
      for (const [key, val] of Object.entries(new_state)) {
        if (val !== undefined) {
          (state as any)[key] = val; // sadge typescript skill issue moment.
        }
      }
      return state;
    })
  }
});

let synced_state: RouterState = def;
router_state.subscribe(state => {
  state.show_left_nav = ["chat", "settings"].includes(state.main_section);
  synced_state = state;
});

let synced_channels: SchemaChannelList;
channels_store.subscribe(store => {
  synced_channels = store;
});

r.on('chat/:section?/:channel_id?', (params) => {
  if (!params) {
    return console.error("params must be defined");
  }
  if (!params.section) {
    return r.route(`/app/chat/${synced_state.chat_nav_section}`, true);
  }

  const section = params.section as ChatNavSections;
  if (!CHAT_NAV_SECTIONS.includes(section)) {
    return console.error(section, "is not valid for chat route");
  }

  const channel_id: string | null = params.channel_id;
  if (!channel_id) {
    if (section !== 'requests') {
      const active_channel_for_section = synced_state.active_channel_for[section]
      if (active_channel_for_section) {
        return r.route(`/app/chat/${section}/${active_channel_for_section}`, true);
      }
    }
  }

  const u: Partial<RouterState> = {
    main_section: 'chat',
    chat_nav_section: section,
  }

  if (channel_id && section !== 'requests') {
    u.active_channel_for = {...synced_state.active_channel_for};
    u.active_channel_for[section] = channel_id;
    u.active_channel = channel_id;

    if (!synced_channels[channel_id]) {
      channels_store.add_channel(channel_id);
    }
  }
  else {
    u.active_channel = null;
  }

  update(u);
})
.on('settings/:section?', params => {
    if (!params) return console.error("params need to be set");
    let section = params.section as SettingsNavSections | undefined;
    if (!section) {
      return r.route(`/app/settings/${synced_state.settings_nav_section}`, true);
    }
    if (!Object.keys(SETTINGS_NAV_SECTIONS).includes(section)) {
      return console.error(section, "is not valid for settings route");
    }

    update({
      main_section: 'settings',
      settings_nav_section: section,
    });
})
.on('add_friend', _ => {
    update({
      main_section: 'friend_req_tool',
    });
})
.on('auth', _ => {
    update({
      main_section: 'auth',
    });
})

export const router = r.listen();

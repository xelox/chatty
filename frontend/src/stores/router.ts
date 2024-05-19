import navaid from 'navaid';
import { writable, type Writable } from 'svelte/store';

const r = navaid('app', (url) => {
  console.log('Used undefined route:', url);
});

type RouterState = {
  show_auth: boolean,
  show_chat: boolean,
  show_settings: boolean,
  show_friend_requester: boolean,
  show_left_nav: boolean,
}
const all_off: RouterState = {
  show_auth: false,
  show_chat: false,
  show_settings: false,
  show_friend_requester: false,
  show_left_nav: false,
};

export const router_state: Writable<RouterState> = writable(all_off);

router_state.subscribe(state => {
  state.show_left_nav = state.show_chat || state.show_settings;
});

r.on('chat', _ => {
  const off_cpy = {...all_off};
  off_cpy.show_chat = true;
  router_state.set(off_cpy);
})
.on('settings/profile', _ => {
  const off_cpy = {...all_off};
  off_cpy.show_settings = true;
  router_state.set(off_cpy);
})
.on('add_friend', _ => {
  const off_cpy = {...all_off};
  off_cpy.show_friend_requester = true;
  router_state.set(off_cpy);
})
.on('auth', _ => {
  const off_cpy = {...all_off};
  off_cpy.show_auth = true;
  router_state.set(off_cpy);
})

export const router = r.listen();

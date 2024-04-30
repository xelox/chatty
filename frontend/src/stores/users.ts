import { writable } from "svelte/store";
import { type SchemaPeerList, type SchemaPeer } from "./data";


// const { subscribe, update } = writable<SchemaPeerList>({})
const { subscribe, update } = writable<SchemaPeerList>({
  'de7091ba-a7dc-4af4-9250-99d265752513': {
    id: 'de7091ba-a7dc-4af4-9250-99d265752513',
    username: 'pablo',
    display_name: null,
    relation_id: null
  },
  '91112343-61aa-4310-8dc5-84cff2b5772c': {
    id: '91112343-61aa-4310-8dc5-84cff2b5772c',
    username: 'pedro',
    display_name: null,
    relation_id: null
  }
})



const update_peers = (users: SchemaPeer[]) => {
  update(list=>{
    for (const user of users) {
      list[user.id] = user;
    }
    return list;
  })
}

const delete_peers = (user_ids: string[]) => {
  update(list => {
    for (const id of user_ids) {
      delete list[id];
    }
    return list;
  })
}

export default {
  update_peers,
  delete_peers,
  subscribe
}

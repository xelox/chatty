import { writable } from "svelte/store";
import { type SchemaPeerList, type SchemaPeer } from "./data";


// const { subscribe, update } = writable<SchemaPeerList>({})
const { subscribe, update } = writable<SchemaPeerList>({})



const update_peers = (users: SchemaPeer[]) => {
  update(list=>{
    for (const user of users) {
      list[user.id] = user;
    }
    return list;
  })
}

const delete_peers = (user_ids: number[]) => {
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

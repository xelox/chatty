import { writable } from "svelte/store"

export type schema_user_data = {
  username: String,
}

export const user_data = writable<schema_user_data | null>(null)

import { writable } from "svelte/store";
export default writable(Object.fromEntries(new URLSearchParams(window.location.search).entries()))

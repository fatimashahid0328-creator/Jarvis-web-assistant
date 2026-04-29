import { writable } from 'svelte/store';

export const voiceStore = writable({
  active: false,
  listening: false,
  transcript: ''
});

export const dataStore = writable(null);

export const commandStore = writable({
  history: [],
  current: null
});

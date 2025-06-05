import { defineStore } from "pinia";
import { AudioBook } from "../../src-tauri/bindings/AudioBook";
// import { useAudioBookLibrary } from "./library";

interface Store {
  currentBook?: AudioBook | null;
  lastPlayed?: AudioBook | null;
}
export const useCurrentBook = defineStore("currentBook", {
  state: (): Store => ({
    currentBook: null,
    lastPlayed: null,
  }),
  getters: {
    currentlyPlaying() {},
  },
  actions: {},
});

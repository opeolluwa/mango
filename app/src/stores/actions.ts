import { defineStore } from "pinia";

interface Store {
  isProcessingNewBook: boolean;
  isPlayingBook: boolean;
}

export const useBookProcesses = defineStore("bookActions", {
  state: (): Store => ({
    isPlayingBook: false,
    isProcessingNewBook: false,
  }),
  getters: {
    currentlyPlaying() {},
  },
  actions: {},
});

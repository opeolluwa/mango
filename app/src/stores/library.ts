import { invoke } from "@tauri-apps/api/core";
import { defineStore } from "pinia";
import { type AudioLibrary } from "../../src-tauri/bindings/AudioLibrary";
import { AudioBook } from "../../src-tauri/bindings/AudioBook";

//lib
//playlists
// settings

export const useMusicLibary = defineStore("musicLibrary", {
  state: () => ({
    audioLibrary: undefined as unknown as AudioLibrary,
    audioBooks: [] as AudioBook[],
    isProcessingPdf: false,
  }),
  actions: {
    async loadMusicLibrary() {
      const library: AudioLibrary = await invoke("read_library");
      this.audioLibrary = library;
      this.audioBooks = library.audioBooks
      console.log({ lib: this.audioLibrary });
    },
  },
  getters: {
    // processingPdf: (state) => state.isProcessingPdf,
  },
});

import { invoke } from "@tauri-apps/api/core";
import { defineStore } from "pinia";
import { type AudioLibrary } from "../../src-tauri/bindings/AudioLibrary";

//lib
//playlists
// settings

export const useMusicLibary = defineStore("musicLibrary", {
  state: () => ({
    audioLibrary: undefined as unknown as AudioLibrary,
    isProcessingPdf: false,
  }),
  actions: {
    async loadMusicLibrary() {
      const library: AudioLibrary = await invoke("read_library");
      this.audioLibrary = library;
      console.log({ lib: this.audioLibrary });
    },
  },
  getters: {
    // processingPdf: (state) => state.isProcessingPdf,
  },
});

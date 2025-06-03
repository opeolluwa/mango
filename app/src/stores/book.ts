import { defineStore } from "pinia";
import { AudioBook } from "../../src-tauri/bindings/AudioBook";
import { useMusicLibary } from "./library";

export const useCurrentBook = defineStore("currentBook", {
  state: () => ({
    currentBook: null as unknown as AudioBook,
    lastPlayed: null as unknown as AudioBook,
  }),
  getters: {
    currentlyPlaying() {
      const library = useMusicLibary();
      console.log("library ->", library);
      console.log("audioLibrary ->", library.audioLibrary);
      console.log("audioBooks ->", library.audioLibrary?.audioBooks);
      console.log("current ->", library.audioLibrary?.audioBooks[0]);

      return library.audioLibrary?.audioBooks[0];
    },
  },
  actions: {},
});

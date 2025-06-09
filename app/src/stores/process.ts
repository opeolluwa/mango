import { defineStore } from "pinia";
import { useAudioBookLibrary } from "./library";
import { useCurrentBook } from "./book.ts";

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
    isPlayingLastBook() {
      const bookStore = useCurrentBook();
      const library = useAudioBookLibrary();

      if (bookStore.currentBook === null) {
        return false;
      }
      return (
        bookStore.currentBook?.title ===
          library.audioBooks[library.audioBooks.length - 1].title || false
      );
    },
    isPlayingFirstBook() {
      const bookStore = useCurrentBook();
      const library = useAudioBookLibrary();
      if (bookStore.currentBook === null) {
        return false;
      }
      return (
        bookStore.currentBook?.title === library.audioBooks[0].title || false
      );
    },
  },

  actions: {},
});

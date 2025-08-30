import { defineStore } from "pinia";
import api from "../plugins/api";
import { AudioBookEntity } from "../types/audioBook";
// import { parseBlob, parseFile, parseWebStream } from "music-metadata";
interface State {
  audioBooks: AudioBookEntity[];
  //   stats:
}

export const useBookStore = defineStore("book_store", {
  state: (): State => ({
    audioBooks: [],
  }),
  actions: {
    async fetchBooks(): Promise<AudioBookEntity[]> {
      const response = await api.get("/books");
      this.audioBooks = response.data.data.records;

      // for (const entry of this.audioBooks) {
      //   const metadata = await parseWebStream(entry.src).catch((error)=>{
      //     console.log(error)
      //   });
      //   console.log({ metadata });
      // }
      return this.audioBooks;
    },
  },
  getters: {},
  persist: true,
});

import { defineStore } from "pinia";
import api from "../plugins/api";
import { AudioBookEntity } from "../types/audioBook";

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
      this.audioBooks = response.data.data.data;
      console.log(JSON.stringify(this.audioBooks, null, 2));
      return this.audioBooks;
    },
  },
  getters: {},
  persist: true,
});

import {defineStore} from "pinia";
import {AudioBook} from "../../src-tauri/bindings/AudioBook";

interface Store {
    currentBook?: AudioBook | null;
    lastPlayed?: AudioBook | null;
    isCurrentlyPlaying?: boolean;
}

export const useCurrentBook = defineStore("currentBook", {
    state: (): Store => ({
        currentBook: null,
        lastPlayed: null,
        isCurrentlyPlaying: false,
    }),
    getters: {
        currentlyPlaying() {
        },
    },
    actions: {},
});

import { defineStore } from "pinia";
import { AudioBook } from "../../src-tauri/bindings/AudioBook";
import { type AudioLibrary } from "../../src-tauri/bindings/AudioLibrary";


interface State {
    audioLibrary: AudioLibrary;
    audioBooks: AudioBook[];
    isProcessingPdf: boolean;

}

export const useAudioBookLibrary = defineStore("musicLibrary", {
    state: () => ({
       
    }),
    actions: {
       
    },
    getters: {
    },
});

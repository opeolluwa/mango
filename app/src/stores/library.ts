import {invoke} from "@tauri-apps/api/core";
import {defineStore} from "pinia";
import {type AudioLibrary} from "../../src-tauri/bindings/AudioLibrary";
import {AudioBook} from "../../src-tauri/bindings/AudioBook";


interface State {
    audioLibrary: AudioLibrary;
    audioBooks: AudioBook[];
    isProcessingPdf: boolean;

}

export const useAudioBookLibrary = defineStore("musicLibrary", {
    state: (): State => ({
        audioLibrary: {} as AudioLibrary,
        audioBooks: [] as AudioBook[],
        isProcessingPdf: false,
    }),
    actions: {
        async loadMusicLibrary() {
            const library: AudioLibrary = await invoke("read_library");
            this.audioLibrary = library;
            this.audioBooks = library.audioBooks
            console.log({lib: this.audioLibrary});
        },
    },
    getters: {
        // processingPdf: (state) => state.isProcessingPdf,
    },
});

import {invoke} from "@tauri-apps/api/core";
import {useCurrentBook} from "../stores/book.ts";
import {useAudioBookLibrary} from "../stores/library.ts";
import {AudioBook} from "../../src-tauri/bindings/AudioBook.ts";
import {open} from "@tauri-apps/plugin-dialog";
import {useBookProcesses} from "../stores/actions.ts";

export async function playAudioBook(bookTitle: string) {
    const bookStore = useCurrentBook();
    const bookLibrary = useAudioBookLibrary();
    const processes = useBookProcesses();

    invoke("play_audio_book", {bookTitle})
        .then(() => {
            bookStore.currentBook = bookLibrary.audioBooks.find(
                (b: AudioBook) => b.fileName === bookTitle
            );
            processes.isPlayingBook = true

            console.log(bookStore.currentBook?.fileName || "error getting file ");
        })
        .catch((err) => {
            console.log(err);
        });
}

export async function pauseAudioBook() {
    const processes = useBookProcesses();
    invoke("pause_audio_book")
        .then(() => {
            processes.isPlayingBook = false
        })
        .catch((err) => {
            console.log(err);
        });
}

export async function createNewBook() {
    const musicLibStore = useAudioBookLibrary();
    const processes = useBookProcesses();
    const file = await open({
        multiple: false,
        directory: false,
        extension: ["pdf"],
    });
    if (!file) return;

    musicLibStore.isProcessingPdf = true;
    invoke("synthesize_audio", {pdfPath: file})
        .then(() => {
            processes.isProcessingNewBook = true;
            musicLibStore.loadMusicLibrary();
        })
        .catch((err) => {
            musicLibStore.isProcessingPdf = false;
            console.log(err);
        });
    processes.isProcessingNewBook = false;
}

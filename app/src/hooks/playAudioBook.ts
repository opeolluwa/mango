import {invoke} from "@tauri-apps/api/core";

export async function playAudioBook(bookTitle: string) {
    invoke("play_audio_book", {bookTitle})
        .then(() => {
            // musicLibStore.isProcessingPdf = false;
            // musicLibStore.loadMusicLibrary();
        })
        .catch((err) => {
            // musicLibStore.isProcessingPdf = false;
            console.log(err);
        });
}
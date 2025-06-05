import { invoke } from "@tauri-apps/api/core";
import { useCurrentBook } from "../stores/book.ts";
import { useAudioBookLibrary } from "../stores/library.ts";
import { AudioBook } from "../../src-tauri/bindings/AudioBook.ts";

export async function playAudioBook(bookTitle: string) {
  const bookStore = useCurrentBook();
  const bookLibrary = useAudioBookLibrary();

  invoke("play_audio_book", { bookTitle })
    .then(() => {
      bookStore.currentBook = bookLibrary.audioBooks.find(
        (b: AudioBook) => b.fileName === bookTitle
      );
      console.log(bookStore.currentBook?.fileName || "error getting file ");
    })
    .catch((err) => {
      console.log(err);
    });
}

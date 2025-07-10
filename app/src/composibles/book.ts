import { invoke } from "@tauri-apps/api/core";
import { useCurrentBook } from "../stores/book.ts";
import { useAudioBookLibrary } from "../stores/library.ts";
import { AudioBook } from "../../src-tauri/bindings/AudioBook.ts";
import { open } from "@tauri-apps/plugin-dialog";
import { useBookProcesses } from "../stores/process.ts";

export async function playAudioBook(bookTitle: string) {
  const bookStore = useCurrentBook();
  const bookLibrary = useAudioBookLibrary();
  const processes = useBookProcesses();

  invoke("play_audio_book", { bookTitle })
    .then(() => {
      bookStore.currentBook = bookLibrary.audioBooks.find(
        (b: AudioBook) => b.title === bookTitle
      );
      processes.isPlayingBook = true;

      console.log(bookStore.currentBook?.title || "error getting file ");
    })
    .catch((err) => {
      console.log(err);
    });
}

export async function pauseAudioBook() {
  const processes = useBookProcesses();
  invoke("pause_audio_book")
    .then(() => {
      processes.isPlayingBook = false;
    })
    .catch((err) => {
      console.log(err);
    });
}

export async function resumeAudioBook() {
  const processes = useBookProcesses();
  const bookStore = useCurrentBook();
  if (!bookStore.currentBook) return;
  invoke(" resume_playing_audio_book")
    .then(() => {
      processes.isPlayingBook = true;
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
  invoke("synthesize_audio", { pdfPath: file })
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

export const playTheNextBook = async () => {
  const library = useAudioBookLibrary();
  const bookStore = useCurrentBook();

  if (!bookStore.currentBook) {
    await playAudioBook(library.audioBooks[0].title);
  }

  if (bookStore.currentBook) {
    let currentBookIndex = library.audioBooks.findIndex(
      (b) => b.title === bookStore.currentBook?.title
    );

    if (currentBookIndex === library.audioBooks.length - 1) {
      return;
    } else {
      currentBookIndex++;
      await playAudioBook(library.audioBooks[currentBookIndex].title);
    }
  }
};

export const playThePreviousBook = async () => {
  const library = useAudioBookLibrary();
  const bookStore = useCurrentBook();

  if (!bookStore.currentBook) {
    await playAudioBook(library.audioBooks[0].title);
  }

  if (bookStore.currentBook) {
    let currentBookIndex = library.audioBooks.findIndex(
      (b) => b.title === bookStore.currentBook?.title
    );
    if (currentBookIndex === 0) {
      return;
    } else {
      currentBookIndex--;
      await playAudioBook(
        library.audioBooks[currentBookIndex].title
      );
    }
  }
};

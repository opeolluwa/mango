import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { useMusicLibary } from "../stores/library";

export async function useCreateNewBook() {
  const musicLibStore = useMusicLibary();

  const file = await open({
    multiple: false,
    directory: false,
    extension: ["pdf"],
  });
  if (!file) return;

  musicLibStore.isProcessingPdf = true;
  invoke("synthesize_audio", { pdfPath: file })
    .then(() => {
      musicLibStore.isProcessingPdf = false;
      musicLibStore.loadMusicLibrary();
    })
    .catch((err) => {
      musicLibStore.isProcessingPdf = false;
      console.log(err);
    });
}

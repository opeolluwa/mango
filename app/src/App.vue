<template>
  <AppLayout>
    <AppNavigation class="col-span-3 xl:col-span-2" />
    <AppMain class="col-span-9 xl:col-span-8" />
    <aside class="hidden xl:block xl:col-span-2"></aside>
    <AudioPlayer :file-name="fileName" :audio-src="audioSource" />
  </AppLayout>
</template>

<script lang="ts" setup>
import { onMounted, ref } from "vue";
import AppLayout from "./components/layouts/AppLayout.vue";
import AudioPlayer from "./components/AudioPlayer.vue";
import AppMain from "./components/uiBlocks/AppMain.vue";
import AppNavigation from "./components/uiBlocks/AppNavigation.vue";
import { useMusicLibary } from "./stores/library";
import * as path from "@tauri-apps/api/path";
import { convertFileSrc } from "@tauri-apps/api/core";

const audioSource = ref("");
const fileName = ref("the great asb.pdf");
const store = useMusicLibary();

const tauriSrc = convertFileSrc("source.mp3", "assets");
onMounted(async () => {
  const dir = await path.audioDir();
  audioSource.value = await path.join(
    dir,
    // "projects",
    "audify",
    "cover letter template.docx.mp3"
  );

  await store.loadMusicLibrary();
});
</script>

<style scoped>
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.5s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>

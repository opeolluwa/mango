<template>
  <AppLayout>
    <AppNavigation class="col-span-3 xl:col-span-2" />
    <Transition name="fade">
      <AppMain class="col-span-9 xl:col-span-8" />
    </Transition>
    <aside class="hidden xl:block xl:col-span-2">hey</aside>

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

const audioSource = ref("source.mp3");
const fileName = ref("the great asb.pdf");
const store = useMusicLibary();

onMounted(async () => {
  const dir = await path.desktopDir();
  audioSource.value = await path.join(
    dir,
    "projects",
    "audify",
    "app/public/source.mp3"
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

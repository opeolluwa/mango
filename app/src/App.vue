<template>
  <AppLayout>
    <AppNavigation class="col-span-3 xl:col-span-2" />
    <AppMain class="col-span-9 xl:col-span-8" />
    <aside class="hidden xl:block xl:col-span-2"></aside>
    <AudioPlayer />
  </AppLayout>
</template>

<script lang="ts" setup>
import { onMounted } from "vue";
import AudioPlayer from "./components/AudioPlayer.vue";
import AppLayout from "./components/layouts/AppLayout.vue";
import AppMain from "./components/uiBlocks/AppMain.vue";
import AppNavigation from "./components/uiBlocks/AppNavigation.vue";
import { useAudioBookLibrary } from "./stores/library.ts";

const store = useAudioBookLibrary();
onMounted(async () => {
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

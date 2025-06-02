<template>
  <AppLayout>
    <AppNavigation class="col-span-3 xl:col-span-2" />
    <Transition name="fade">
      <AppMain class="col-span-9 xl:col-span-8" />
    </Transition>
    <aside class="hidden xl:block xl:col-span-2">hey</aside>
    <AppFooter :file-name="fileName" :audio-src="audioSource" />
  </AppLayout>
</template>

<script lang="ts" setup>
import { onMounted, ref } from "vue";
import AppLayout from "./components/layouts/AppLayout.vue";
import AppFooter from "./components/uiBlocks/AppFooter.vue";
import AppMain from "./components/uiBlocks/AppMain.vue";
import AppNavigation from "./components/uiBlocks/AppNavigation.vue";
import { useMusicLibary } from "./stores/library";

const audioSource = ref("source.mp3");
const fileName = ref("the great asb.pdf");
const store = useMusicLibary();

onMounted(() => {
  store.loadMusicLibrary();
});
</script>
<style scoped>
@reference "./assets/styles.css";

.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.5s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>

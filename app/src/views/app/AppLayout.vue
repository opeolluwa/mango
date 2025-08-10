<template>
  <div class="overflow-x-hidden">
    <AppHeader @toggle-nav="showSideNav = $event" />
    <AppNavigation
      v-if="showSideNav"
      class="fixed left-0 bottom-16 w-[70vw] z-[6000] dark:bg-app-dark bg-white"
    />
    <main class="layout pt-3 parent-element w-screen dark:bg-app-dark">
      <RouterView v-slot="{ Component }">
        <Transition name="fade">
          <Component :is="Component" />
        </Transition>
      </RouterView>
    </main>
    <AppDock />
    <SensationalTint
      
    />
  </div>
</template>

<script lang="ts" setup>
import SensationalTint from "@/components/uiBlocks/SensationalTint.vue";
import { listen } from "@tauri-apps/api/event";
import { onMounted, ref } from "vue";
import { type AudioSynthesisEvent } from "../../../src-tauri/bindings/AudioSynthesisEvent.ts";
import { type CurrentAudioMetadata } from "../../../src-tauri/bindings/CurrentAudioMetadata.ts";
import { type Events } from "../../../src-tauri/bindings/Events.ts";
import AppDock from "../../components/uiBlocks/AppDock.vue";
import AppHeader from "../../components/uiBlocks/AppHeader.vue";
import AppNavigation from "../../components/uiBlocks/AppNavigation.vue";
import { useAudioBookLibrary } from "../../stores/library.ts";
const store = useAudioBookLibrary();
const showSideNav = ref(false);
onMounted(async () => {
  await store.loadMusicLibrary();

  listen<AudioSynthesisEvent>("processing-audio", (event) => {
    console.log(event.payload);
  });

  listen<AudioSynthesisEvent>(
    "finished-processing-audio" as Events,
    (event) => {
      console.log(event.payload);
    }
  );

  listen<CurrentAudioMetadata>("finished-processing-audio", (event) => {
    console.log(event.payload);
  });

  listen<CurrentAudioMetadata>("currently-playing-audio", (event) => {
    console.log(event.payload);
  });
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

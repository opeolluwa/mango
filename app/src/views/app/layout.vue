<template>
  <div class="overflow-x-hidden">
    <main class="layout pt-3 parent-element w-screen">
      <RouterView v-slot="{ Component }">
        <Transition name="fade">
          <Component :is="Component" />
        </Transition>
      </RouterView>
    </main>
    <AppDock />
    <AppDockPlayer />
  </div>
</template>

<script lang="ts" setup>
import { listen } from "@tauri-apps/api/event";
import AppDockPlayer from "../../components/uiBlocks/AppDockPlayer.vue";
import { onMounted } from "vue";
import { type AudioSynthesisEvent } from "../../../src-tauri/bindings/AudioSynthesisEvent.ts";
import { type CurrentAudioMetadata } from "../../../src-tauri/bindings/CurrentAudioMetadata.ts";
import { type Events } from "../../../src-tauri/bindings/Events.ts";
import AppDock from "../../components/uiBlocks/AppDock.vue";
import { useAudioBookLibrary } from "../../stores/library.ts";

const store = useAudioBookLibrary();
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

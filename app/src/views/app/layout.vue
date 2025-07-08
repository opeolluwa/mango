<template>
  <AppLayout class="overflow-x-hidden">
    <AppMain class="px-2" />
    <AppDock />
  </AppLayout>
</template>

<script lang="ts" setup>
import AppLayout from "@/components/layouts/AppLayout.vue";
import AppMain from "@/components/uiBlocks/AppMain.vue";
import { listen } from "@tauri-apps/api/event";
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

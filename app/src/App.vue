<template>
  <template v-if="totalBooks == 0">
    <EmptyState @click="createNewBook" />
  </template>

  <template v-else>
    <ProcessingState v-show="processingPdf" />

    <div
      id="container"
      class="h-screen overflow-hidden bg-secondary bg-[url(cover.jpg)] bg-center-center bg-blend-multiply bg-cover bg-no-repeat bg-accent/60 text-accent-secondary relative"
    >
      <div class="flex justify-between rounded bg-blend-multiply px-4 py-6">
        <Bars3Icon
          class="size-7 cursor-pointer hover:opacity-65 hover:scale-95"
        />
        <PlusCircleIcon class="size-7 control" @click="createNewBook" />
      </div>

      <AudioPlayer
        :file-name="fileName"
        :is-playing="false"
        :audio-src="audioSource"
      />
    </div>
  </template>
</template>

<script lang="ts" setup>
import { Bars3Icon, PlusCircleIcon } from "@heroicons/vue/24/solid";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { computed, onMounted, ref, watch } from "vue";
import { type AudioLibrary } from "../src-tauri/bindings/AudioLibrary";
import AudioPlayer from "./components/AudioPlayer.vue";
import EmptyState from "./components/EmptyState.vue";
import ProcessingState from "./components/ProcessingState.vue";

// State refs
const currentTime = ref(0);
const processingPdf = ref(false);
const audioLibrary = ref<AudioLibrary | null>(null);

// Derived reactive refs
const audioBooks = computed(() => audioLibrary.value?.audioBooks ?? []);
const totalBooks = computed(() => audioBooks.value.length);

const audioSource = ref("source.mp3");
const fileName = ref("the great asb.pdf");
const loadLibrary = () =>
  invoke("read_library").then((res) => {
    audioLibrary.value = res as AudioLibrary;
  });

//hooks
onMounted(() => {
  loadLibrary();
  if (audioBooks.value.length > 0) {
  }
});
watch(currentTime, (val) => {
  currentTime.value = Math.round(val);
});

// PDF -> Audio book creation
async function createNewBook() {
  const file = await open({
    multiple: false,
    directory: false,
    extension: ["pdf"],
  });
  if (!file) return;

  processingPdf.value = true;
  invoke("synthesize_audio", { pdfPath: file })
    .then(() => {
      processingPdf.value = false;
      loadLibrary(); // Reload library after processing
    })
    .catch(() => {
      processingPdf.value = false;
    });
}
</script>

<style scoped></style>

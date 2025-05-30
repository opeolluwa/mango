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
          @click="previewLibrary"
        />
        <PlusCircleIcon class="size-7 control" @click="createNewBook" />
      </div>

      <div
        id="wrapper"
        class="flex flex-col items-center gap-x-2 gap-y-2 justify-evenly absolute left-0 right-0 bottom-5 mb-4"
        v-for="(audio, index) in audioBooks"
        :key="index"
        v-bind:class="{ isActive: isCurrentSong(index) }"
      >
        <div id="info" class="text-center">
          <p class="leading-loose">You are listening to...</p>
          <h2 class="text-xl my-2 font-medium overflow-hidden text-ellipsis">
            {{ audio.fileName }}
          </h2>
        </div>

        <div id="progress" class="flex flex-col px-12">
          <div class="flex justify-between">
            <!----<small>{{ currentTime | +fancyTimeFormat }}</small>

            <small>{{ trackDuration | +fancyTimeFormat }}</small>-->
          </div>

          <Progressbar />
        </div>

        <div id="controls" class="flex items-center justify-evenly">
          <button
            @click="prevSong"
            class="size-8 disabled:text-accent/95 control"
            :disabled="index == 0"
          >
            <BackwardIcon />
          </button>
          <button
            class="bg-accent-secondary text-accent flex justify-center items-center rounded-full size-16 active:scale-75 transition-all duration-75 ease-linear p-[5px]"
            @click="
              togglePlaylist;
              playSound(index);
            "
          >
            <PlayIcon
              class="size-8 transition duration-150 ease-in-out"
              v-show="!isPlaying"
            />
            <PauseIcon
              class="size-8 transition duration-150 ease-in-out"
              v-show="isPlaying"
            />
          </button>
          <button
            class="size-8 disabled:text-accent/95 control"
            :disabled="index == totalBooks - 1"
            @click="nextSong"
          >
            <ForwardIcon />
          </button>
        </div>
      </div>
    </div>
  </template>
</template>

<script lang="ts" setup>
import ProcessingState from "./components/ProcessingState.vue";
import EmptyState from "./components/EmptyState.vue";
import Progressbar from "./components/ProgressBar.vue"
import {
  Bars3Icon,
  PlayIcon,
  PauseIcon,
  ForwardIcon,
  BackwardIcon,
  ViewColumnsIcon,
  PlusCircleIcon,
} from "@heroicons/vue/24/solid";
import { ref, onMounted, onBeforeUnmount, watch, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { type AudioLibrary } from "../src-tauri/bindings/AudioLibrary";
import { open } from "@tauri-apps/plugin-dialog";
import useHotkey, { HotKey, RemoveHandler } from "vue3-hotkey";
import { listen } from "@tauri-apps/api/event";
import { useSound } from "@vueuse/sound";

// State refs
const isPlaying = ref(false);
const audio = ref<HTMLAudioElement>();
const currentlyPlaying = ref(false);
const currentlyStopped = ref(false);
const currentTime = ref(0);
const processingPdf = ref(false);
const trackDuration = ref(0);
const currentSong = ref(0);
const displayLibrary = ref(false);
const audioLibrary = ref<AudioLibrary | null>(null);

// Derived reactive refs
const audioBooks = computed(() => audioLibrary.value?.audioBooks ?? []);
const totalBooks = computed(() => audioBooks.value.length);

onMounted(() => {
  invoke("read_library").then((res) => {
    audioLibrary.value = res as AudioLibrary;
  });

  if (audioBooks.value.length > 0) {
    changeSong(0); // Only call after books are loaded
  }
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

function loadLibrary() {
  invoke("read_library").then((res) => {
    audioLibrary.value = res as AudioLibrary;
  });
}

function nextSong() {
  if (currentSong.value < audioBooks.value.length - 1)
    changeSong(currentSong.value + 1);
}

function prevSong() {
  if (currentSong.value > 0) changeSong(currentSong.value - 1);
}

function changeSong(index: number) {
  const wasPlaying = currentlyPlaying.value;

  //TODO:
  // stopAudio();

  currentSong.value = index;
  const source = audioBooks.value[index]?.audioSrc;
  if (!source) return;

  audio.value = new Audio(source);

  audio.value.addEventListener("loadedmetadata", () => {
    trackDuration.value = Math.round(audio.value?.duration || 0);
  });

  // audio.value.addEventListener("ended", handleEnded);

  //TODO:
  // if (wasPlaying) {
  //   playAudio();
  // }
}

function isCurrentSong(index: number) {
  return currentSong.value === index;
}
// function playSound(index: number) {
//   const source = audioBooks.value[0]?.audioSrc;
//   const { play } = useSound(source);
//   play();
//   console.log({ source });
// }

function playSound(index: number) {
  const source = audioBooks.value[index]?.audioSrc;
  if (!source) return;

  audio.value.play();
  // if (!audio.value) {
  //   audio.value = new Audio(source);
  //   audio.value.addEventListener("ended", () => {
  //     isPlaying.value = false;
  //   });
  // }

  // if (isPlaying.value) {
  //   audio.value.pause();
  //   isPlaying.value = false;
  // } else {
  //   audio.value.play();
  //   isPlaying.value = true;
  // }
}

watch(currentTime, (val) => {
  currentTime.value = Math.round(val);
});
</script>

<style scoped>
@reference "./assets/styles.css";

#wrapper {
  @apply hidden;
}
#wrapper > div {
  @apply px-8 py-3;
}

#wrapper.isActive {
  @apply block;
}

.control {
  @apply cursor-pointer hover:opacity-75 hover:scale-95  transition-all duration-75 ease-linear;
}
</style>

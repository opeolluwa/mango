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
        <!-- <Bars3Icon
          class="size-7 cursor-pointer hover:opacity-65 hover:scale-95"
          @click="previewLibrary"
        /> -->
        <Bars3Icon
          class="size-7 cursor-pointer hover:opacity-65 hover:scale-95"
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
        <audio
          src="source.mp3"
          controls
          ref="audioPlayer"
          class="w-full"
        ></audio>
        <div id="info" class="text-center">
          <p class="leading-loose">You are listening to...</p>
          <h2 class="text-xl my-2 font-medium overflow-hidden text-ellipsis">
            {{ audio.fileName }}
          </h2>
        </div>

        <audio :src="audioBooks[index]?.audioSrc"></audio>
        <div id="progress" class="flex flex-col px-12">
          <div class="flex justify-between mb-1 prose-sm">
            <small> 0.00 </small>
            <small>6.25</small>
          </div>
          <Progressbar :progress="100" />
        </div>

        <div id="controls" class="flex items-center justify-evenly">
          <button
            @click="gotoPreviousSong"
            class="size-8 disabled:text-accent/95 control"
            :disabled="index == 0"
          >
            <BackwardIcon />
          </button>

          <button
            class="bg-accent-secondary text-accent flex justify-center items-center rounded-full size-16 active:scale-75 transition-all duration-75 ease-linear p-[5px]"
            @click="playTestfile"
          >
            <!-- <button
            class="bg-accent-secondary text-accent flex justify-center items-center rounded-full size-16 active:scale-75 transition-all duration-75 ease-linear p-[5px]"
            @click="changeSong(index)"
          > -->
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
            @click="gotoNextSong"
          >
            <ForwardIcon />
          </button>
        </div>
      </div>
    </div>
  </template>
</template>

<script lang="ts" setup>
import {
  BackwardIcon,
  Bars3Icon,
  ForwardIcon,
  PauseIcon,
  PlayIcon,
  PlusCircleIcon,
} from "@heroicons/vue/24/solid";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { computed, onMounted, ref, useTemplateRef, watch } from "vue";
import { type AudioLibrary } from "../src-tauri/bindings/AudioLibrary";
import EmptyState from "./components/EmptyState.vue";
import ProcessingState from "./components/ProcessingState.vue";
import Progressbar from "./components/ProgressBar.vue";

const source = "source.mp3";

const audioPlayer = ref();
const playTestfile = () => {
  audioPlayer.value?.play()
};

// State refs
const isPlaying = ref(false);
const audio = ref();
const currentTime = ref(0);
const processingPdf = ref(false);
const trackDuration = ref(0);
const currentSong = ref(0);
const audioLibrary = ref<AudioLibrary | null>(null);

const audsrc = "./source.mp3";

// Derived reactive refs
const audioBooks = computed(() => audioLibrary.value?.audioBooks ?? []);
const totalBooks = computed(() => audioBooks.value.length);

// helpers
const gotoPreviousSong = () =>
  currentSong.value > 0 && changeSong(currentSong.value - 1);
const gotoNextSong = () =>
  currentSong.value < audioBooks.value.length - 1 &&
  changeSong(currentSong.value + 1);

// const togglePlaySound = () => (isPlaying.value = !isPlaying.value);
const isCurrentSong = (index: number) => currentSong.value === index;
const loadLibrary = () =>
  invoke("read_library").then((res) => {
    audioLibrary.value = res as AudioLibrary;
  });

//hooks
onMounted(() => {
  loadLibrary();
  if (audioBooks.value.length > 0) {
    changeSong(0);
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

async function changeSong(index: number) {
  const source = audioBooks.value[index]?.audioSrc;
  if (!source) return;

  audio.value = new Audio(audsrc);

  console.log({ src: audio.value.src });
  if (audio.value) {
    audio.value.pause();
    audio.value.currentTime = 0;
  }

  // // const { play } = useSound(source);
  // // play();

  // const newAudio = new Audio(source);
  // audio.value = newAudio;
  // currentSong.value = index;

  audio.value.addEventListener("loadedmetadata", () => {
    trackDuration.value = Math.round(audio.value!.duration);
    audio.value.play().catch(console.error);
  });

  audio.value.addEventListener("timeupdate", () => {
    currentTime.value = audio.value.currentTime;
  });

  audio.value.addEventListener("ended", () => {
    isPlaying.value = false;
  });

  if (isPlaying.value) {
    await audio.value.play().catch(console.error);
  }
}

async function togglePlaySound() {
  if (!audio.value) return;

  if (isPlaying.value) {
    audio.value.pause();
  } else {
    await audio.value
      .play()
      .catch((err: any) => console.log("failed to play due to ", err));
    // .catch("failed to play sound duue to", console.error);
  }

  isPlaying.value = !isPlaying.value;
}
</script>

<style scoped>
audio::-webkit-media-controls-enclosure {
  border-radius: 5px;
  background-color: green;
}

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

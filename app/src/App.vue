<template>
  <template v-if="totalBooks == 0">
    <div
      class="h-screen overflow-hidden bg-secondary bg-[url(cover.jpg)] bg-center-center bg-blend-multiply bg-cover bg-no-repeat bg-accent/60 text-accent-secondary relative flex flex-col justify-center items-center"
    >
      <ViewColumnsIcon class="size-40 mb-5" />

      <h2 class="text-xl font-medium leading-10">
        No audio book has been added
      </h2>
      <small class="leading-1 mb-2"
        >Your audio books will appear as soon as you begin to add them</small
      >
      <button
        class="bg-accent-secondary text-accent inline-flex gap-x-2 rounded items-center px-6 py-1 mt-2 cursor-pointer shadow-md transition-colors duration-200 ease-linear hover:opacity-95 hover:scale-95 control"
        @click="createNewBook"
      >
        <PlusCircleIcon class="size-7" @click="invoke('synthesize_audio')" />
        Create
      </button>
    </div>
  </template>

  <template v-else>
    <div
      class="modal absolute bg-black/60 flex flex-col text-accent-secondary items-center justify-center w-screen h-screen z-50"
      v-show="processingPdf"
    >
      <Loader />
      <small>Processing... </small>
    </div>

    <div
      class="modal absolute bg-black/60 flex flex-col text-accent-secondary w-screen h-screen z-50 px-8"
      v-show="displayLibrary"
    >
      <form action="" class="text-accent-secondary">
        <div>
          <label for="price" class="block text-sm/6 font-medium text-gray-900"
            >Audio books</label
          >
          <input
            type="search"
            name="price"
            id="price"
            class="block min-w-0 grow py-1.5 pr-3 pl-1 text-base text-gray-900 placeholder:text-gray-400 focus:outline-none sm:text-sm/6 border bg-gray-500 w-full rounded-lg py-2 px-6 text-accent-secondary placeholder:px-4 placeholder:text-accent-secondary border-accent-secondary cursor-pointer"
            placeholder="search collection"
          />

          <ul class="mt-2">
            <li
              v-for="(audio, index) in audioBooks"
              :key="index"
              class="text-sm font-medium overflow-hidden text-ellipsis cursor-pointer"
            >
              {{ audio.fileName }}
            </li>
          </ul>
        </div>
      </form>
    </div>
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
            <small>{{ currentTime | +fancyTimeFormat }}</small>

            <small>{{ trackDuration | +fancyTimeFormat }}</small>
          </div>

          <progress
            value="32"
            max="100"
            class="h-1 flex rounded w-full"
          ></progress>
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
import {
  Bars3Icon,
  PlusCircleIcon,
  PlayIcon,
  PauseIcon,
  ForwardIcon,
  BackwardIcon,
  ViewColumnsIcon,
} from "@heroicons/vue/24/solid";
import { ref, onMounted, onBeforeUnmount, watch, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import Loader from "./Loader.vue";
import { type AudioLibrary } from "../src-tauri/bindings/AudioLibrary";
import { open } from "@tauri-apps/plugin-dialog";
import useHotkey, { HotKey, RemoveHandler } from "vue3-hotkey";

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

const hotkeys = ref<HotKey[]>([
  {
    keys: ["esc"],
    preventDefault: true,
    handler() {
      if (displayLibrary.value == true) {
        displayLibrary.value = false;
      }
    },
  },
 
]);
const stopArr = useHotkey(hotkeys.value);

// 取消监听快捷键
// const removeHotKeys = (hk: HotKey) => {
//   stopArr.foreach((item: RemoveHandler) => item());
// };

// Lifecycle: onMounted
onMounted(async () => {
  const res = await invoke("read_library");
  audioLibrary.value = res as AudioLibrary;

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

function previewLibrary() {
  displayLibrary.value = true;
  loadLibrary();
  // listen to escap and click event
}

// Playback control
function togglePlaylist() {
  isPlaying.value = !isPlaying.value;
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

  stopAudio();

  currentSong.value = index;
  const source = audioBooks.value[index]?.audioSrc;
  if (!source) return;

  audio.value = new Audio(source);

  audio.value.addEventListener("loadedmetadata", () => {
    trackDuration.value = Math.round(audio.value?.duration || 0);
  });

  audio.value.addEventListener("ended", handleEnded);

  if (wasPlaying) {
    playAudio();
  }
}

function playAudio() {
  if (
    currentlyStopped.value &&
    currentSong.value + 1 === audioBooks.value.length
  ) {
    currentSong.value = 0;
    changeSong(0);
  }

  if (!currentlyPlaying.value) {
    currentlyPlaying.value = true;
    audio.value?.play();
  } else {
    stopAudio();
  }

  currentlyStopped.value = false;
}

function stopAudio() {
  audio.value?.pause();
  currentlyPlaying.value = false;
}

function handleEnded() {
  if (currentSong.value + 1 === audioBooks.value.length) {
    stopAudio();
    currentlyStopped.value = true;
  } else {
    currentSong.value++;
    changeSong(currentSong.value);
    playAudio();
  }
}

function isCurrentSong(index: number) {
  return currentSong.value === index;
}

function playSound(index: number) {
  const source = audioBooks.value[index]?.audioSrc;
  if (!source) return;

  if (!audio.value) {
    audio.value = new Audio(source);
    audio.value.addEventListener("ended", () => {
      isPlaying.value = false;
    });
  }

  if (isPlaying.value) {
    audio.value.pause();
    isPlaying.value = false;
  } else {
    audio.value.play();
    isPlaying.value = true;
  }
}

// Cleanup
onBeforeUnmount(() => {
  if (audio.value) {
    audio.value.removeEventListener("ended", handleEnded);
  }
  stopArr.forEach((item: RemoveHandler) => item());
});

// Optional watch
watch(currentTime, (val) => {
  currentTime.value = Math.round(val);
});

// Time format
function fancyTimeFormat(s: number) {
  return `${Math.floor(s / 60)}:${s % 60 < 10 ? "0" : ""}${s % 60}`;
}
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

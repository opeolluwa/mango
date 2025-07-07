<template>
  <footer
    class="fixed bg-app-dark border-t-gray-100/10 w-screen min-h-12 bottom-0 parent-element py-3 text-small items-center gap-y-1 justify-between z-5000 flex flex-col"
  >
    <div class="gap-x-4 hidden lg:flex">
      <img
        src="../assets/cover.jpg"
        class="w-10 h-10 rounded col-span-1"
        alt=""
      />
      <div class="col-span-1">
        <small class="text-gray-400"> You are listening to... </small>
        <h6
          class="text-[14px] w-[150px] overflow-ellipsis truncate leading-3.5"
        >
          {{ fileName }}
        </h6>
      </div>
    </div>

    <div class="flex gap-x-4 items-center">
      <Icon
        icon="fluent:previous-48-filled"
        @click="playThePreviousBook"
        :class="[
          isFirstBookIndex
            ? 'text-gray-400/50 cursor-not-allowed hover:text-gray-400/50 size-5'
            : 'icon text-white/90',
        ]"
      />

      <div
        class="w-8 h-8 rounded-lg bg-app-orange/90 shadow shadow-bg-app-orange/60 border-gray-600 flex items-center justify-center text-white/60"
      >
        <Icon
          icon="fluent:play-48-filled"
          class="icon"
          v-show="!isPlaying"
          @click="togglePlaying"
        />
        <Icon
          icon="fluent:pause-48-filled"
          class="icon p-[2px]"
          v-show="isPlaying"
          @click="togglePlaying"
        />
      </div>
      <Icon
        icon="fluent:next-48-filled"
        @click="playTheNextBook"
        :class="[
          isLastBookIndex
            ? 'text-gray-400/50 cursor-not-allowed hover:text-gray-400/50 size-5'
            : 'icon',
        ]"
      />
      <Icon
        icon="gravity-ui:heart-fill"
        class="icon text-app-red hidden"
        v-show="isLoved"
        @click="toggledIsLoved"
      />
      <Icon
        icon="gravity-ui:heart"
        class="icon text-gray-500 hidden"
        v-show="!isLoved"
        @click="toggledIsLoved"
      />
    </div>

    <div class="flex w-full items-center gap-x-2">
      <div class="text-[12px] text-gray-400">
        {{ formatTime(currentTime) }}
      </div>
      <SliderRoot
        v-model="sliderValue"
        class="relative flex items-center select-none touch-none w-full h-5"
        :max="100"
        :step="1"
      >
        <SliderTrack class="bg-app-gray relative grow rounded-full h-1">
          <SliderRange class="absolute bg-grass8 rounded-full h-full" />
        </SliderTrack>
        <SliderThumb
          class="block w-4 h-4 p-1 bg-app-orange rounded-full hover:bg-app-orange shadow-sm focus:outline-none  border-none outline-none"
          aria-label="Volume"
        />
      </SliderRoot>

      <div class="text-[12px] text-gray-400">
        {{ formatTime(duration) }}
      </div>
    </div>

    <div class=" w-[10%] items-center gap-x-2 hidden">
      <Icon icon="fluent-mdl2:volume-3" class="icon" />
      <ProgressBar class="w-4/5" :progress="volume * 100 || 0" />
    </div>
    <div class="gap-x-2 items-center hidden">
      <Icon icon="iconamoon:playlist-shuffle-fill" class="size-5" />
      <Icon icon="iconamoon:playlist-repeat-list-light" class="size-5" />
      <Icon icon="solar:playlist-outline" class="size-5" />
    </div>
  </footer>
</template>

<script lang="ts" setup>
import { Icon } from "@iconify/vue";
import ProgressBar from "./ProgressBar.vue";
import { computed, ref } from "vue";
import { useCurrentBook } from "../stores/book.ts";
import { useBookProcesses } from "../stores/process.ts";
import { SliderRange, SliderRoot, SliderThumb, SliderTrack } from "reka-ui";
import {
  pauseAudioBook,
  playTheNextBook,
  playThePreviousBook,
  resumeAudioBook,
} from "../hooks/book.ts";

const store = useCurrentBook();
const processes = useBookProcesses();

const fileName = computed(() => store.currentBook?.title);
const isPlaying = computed(() => processes.isPlayingBook);
const isLastBookIndex = computed(() => processes.isPlayingLastBook);
const isFirstBookIndex = computed(() => processes.isPlayingFirstBook);

const isLoved = ref(false);
const currentTime = ref(0);
const duration = ref(0);
const volume = ref(0.25);

const sliderValue = ref([(currentTime.value / duration.value) * 100 || 0]);

// audioRef.value?.volume = volume.value;
const toggledIsLoved = () => {
  isLoved.value = !isLoved.value;
};

const togglePlaying = async () => {
  if (processes.isPlayingBook) {
    await pauseAudioBook();
  } else {
    await resumeAudioBook();
    // playAudioBook(String(store.currentBook?.fileName));
  }

  processes.isPlayingBook = !processes.isPlayingBook;
};

// const updateCurrentTime = () => {
//   if (audioRef.value) {
//     currentTime.value = audioRef.value.currentTime;
//   }
// };
//
// const updateDuration = () => {
//   if (audioRef.value) {
//     duration.value = audioRef.value.duration;
//   }
// };

const formatTime = (time: number): string => {
  const minutes = Math.floor(time / 60);
  const seconds = Math.floor(time % 60);
  return `${String(minutes).padStart(2, "0")}:${String(seconds).padStart(
    2,
    "0"
  )}`;
};
</script>

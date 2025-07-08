<template>
  <footer
    class="fixed bg-app-dark border-t-gray-100/10 w-screen min-h-12 bottom-0 parent-element py-3 text-small items-center gap-y-1 justify-between z-5000 flex flex-col"
  >
    <h6 class="text-4xl overflow-ellipsis truncate leading-3.5">
      Barking up the wront tree
    </h6>

    <div class="flex gap-x-4 items-center justify-between">
      <Icon
        icon="mingcute:rewind-backward-10-line"
        @click="playThePreviousBook"
        :class="['icon size-5 text-white/90']"
      />
      <!-- <Icon
        icon="fluent:previous-48-filled"
        @click="playThePreviousBook"
        :class="[
          isFirstBookIndex
            ? 'text-gray-400/50 cursor-not-allowed hover:text-gray-400/50 size-5'
            : 'icon text-white/90',
        ]"
      /> -->

      <div
        class="size-12 rounded-full bg-app-orange/90 shadow shadow-bg-app-orange/60 border-gray-600 flex items-center justify-center text-white"
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
      <!-- <Icon
        icon="fluent:next-48-filled"
        @click="playTheNextBook"
        :class="[
          isLastBookIndex
            ? 'text-gray-400/50 cursor-not-allowed hover:text-gray-400/50 size-5'
            : 'icon',
        ]"
      /> -->
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

      <!-- <Icon icon="fluent-mdl2:volume-3" class="icon" /> -->
      <Icon icon="mingcute:rewind-forward-10-line" class="icon" />
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
          class="block w-4 h-4 p-1 bg-app-orange rounded-full hover:bg-app-orange shadow-sm focus:outline-none border-none outline-none"
          aria-label="Volume"
        />
      </SliderRoot>

      <div class="text-[12px] text-gray-400">
        {{ formatTime(duration) }}
      </div>
    </div>

    <div class="w-[10%] items-center gap-x-2 hidden">
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
import { SliderRange, SliderRoot, SliderThumb, SliderTrack } from "reka-ui";
import { computed, ref } from "vue";
import {
  playThePreviousBook
} from "../hooks/book.ts";
import { useCurrentBook } from "../stores/book.ts";
import { useBookProcesses } from "../stores/process.ts";
import ProgressBar from "./ProgressBar.vue";

const songs = ref([
  {
    title: "some title",
    audioSrc:
      "https://ik.imagekit.io/nethbooks/tes.edited-v2.pdf_Yanwb0U8U.mp3?updatedAt=1751924145616",
  },
  {
    title: "adeoye",
    audioSrc:
      "https://ik.imagekit.io/nethbooks/Adeoye%20Testimony%20Toluwanimi%20-%20Cover%20Pages.pdf_Titdm2bgI.mp3?updatedAt=1751924085915",
  },
]);
const player = ref(new Audio());
const store = useCurrentBook();
const processes = useBookProcesses();

const fileName = computed(() => store.currentBook?.title);
const isPlaying = ref(false);
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
  // if (player.value.paused){

  // }
  if (isPlaying.value) {
    pause();
  } else {
    play();
  }

  isPlaying.value = !isPlaying.value;
  // processes.isPlayingBook = !processes.isPlayingBook;
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

const play = () => {
  player.value.src =
    "https://ik.imagekit.io/nethbooks/tes.edited-v2.pdf_Yanwb0U8U.mp3?updatedAt=1751924145616";

    player.value.playbackRate = 0.95

  player.value.addEventListener("loadedmetadata", () => {
    player.value.play();
  });
};

const pause = () => {
  player.value.addEventListener("loadedmetadata", () => {
    player.value.pause();
  });
};
</script>

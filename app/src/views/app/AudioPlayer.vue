<template>
  <div class="">
    <div
      class="w-screen z-101 absolute top-2 left-0 bg-app-dark/80 h-[100vh]"
      @click="toggleSideNav"
      v-show="showSideNav"
    >
      <Transition name="fade">
        <AppNavigation class="w-full absolute bg-app-dark" />
      </Transition>
    </div>

    <div class="flex justify-between mb-10">
      <Icon
        icon="fluent:chevron-left-32-filled"
        @click="playThePreviousBook"
        :class="['icon size-5 text-white/90']"
      />
      <Icon
        icon="tabler:dots"
       @click="toggleSideNav"
        :class="['icon size-5 text-white/90']"
      />
    </div>
    <img
      src="@/assets/test.jpg"
      class="contain mb-12 h-[200px] mx-auto rounded"
      alt=""
    />

    <div class="mb-12 text-center">
      <h6 class="text-5xl text-center text-white/80">
        Barking up the wrong tree
      </h6>
      <p class="small text-gray-400 mt-2">Eric humming bird</p>
    </div>

    <div class="flex gap-x-4 items-center justify-center">
      <Icon
        icon="mingcute:rewind-backward-10-line"
        @click="playThePreviousBook"
        :class="['icon size-5 text-white/90']"
      />

      <div
        class="size-16 rounded-full bg-app-orange/90 shadow shadow-bg-app-orange/60 border-gray-600 flex items-center justify-center text-white"
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

      <!-- <Icon icon="fluent-mdl2:volume-3" class="icon" /> -->
      <Icon icon="mingcute:rewind-forward-10-line" class="icon" />
    </div>

    <div class="flex w-full items-center gap-x-2 mt-12">
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
      <!-- <ProgressBar class="w-4/5" :progress="volume * 100 || 0" /> -->
    </div>
    <div class="gap-x-2 items-center hidden">
      <Icon icon="iconamoon:playlist-shuffle-fill" class="size-5" />
      <Icon icon="iconamoon:playlist-repeat-list-light" class="size-5" />
      <Icon icon="solar:playlist-outline" class="size-5" />
    </div>
  </div>
</template>

<script lang="ts" setup>
import { Icon } from "@iconify/vue";
import { SliderRange, SliderRoot, SliderThumb, SliderTrack } from "reka-ui";
import { ref } from "vue";
import { playThePreviousBook } from "../../hooks/book.ts";
import AppNavigation from "../../components/uiBlocks/AppNavigation.vue";

const player = ref(new Audio());
const isPlaying = ref(false);

const currentTime = ref(0);
const duration = ref(0);

const sliderValue = ref([(currentTime.value / duration.value) * 100 || 0]);

const showSideNav = ref(false);
const toggleSideNav = () => (showSideNav.value = !showSideNav.value);

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

  player.value.playbackRate = 0.95;

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

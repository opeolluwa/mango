<template>
  <div class="">
    <Transition>
      <AppNavigation
        v-if="show"
        class="w-full absolute top-0 left-0 h-screen z-110 bg-app-dark/95"
        @click="show = !show"
      />
    </Transition>

    <div class="flex justify-between mb-10 pb-3">
      <Icon
        icon="fluent:chevron-left-32-filled"
        :class="['icon size-5 dark:text-white/90']"
        @click="useGoBack"
      />

      <Icon
        icon="tabler:dots"
        :class="['icon size-5 dark:text-white/90']"
        @click="show = !show"
      />
    </div>
    <img
      src="@/assets/test.jpg"
      class="contain mb-12 h-[250px] mx-auto rounded"
      alt=""
    />

    <div class="mb-20 text-center">
      <h6 class="text-3xl text-center text-gray-500">Half of a yellow sun</h6>
      <p class="small text-gray-400 mt-2">Chimamanda Nogozi</p>
    </div>

    <div class="flex gap-x-6 items-center justify-center">
      <Icon
        icon="iconoir:rewind-solid"
        :class="['icon size-5 dark:text-white/90']"
        @click="playThePreviousBook"
      />

      <Icon
        icon="mingcute:rewind-backward-10-line"
        :class="['icon size-5 dark:text-white/90']"
        @click="playThePreviousBook"
      />

      <div
        class="size-16 rounded-full bg-app-orange/90 shadow shadow-bg-app-orange/60 border-gray-600 flex items-center justify-center text-white"
        @click="togglePlaying"
      >
        <Icon v-show="!isPlaying" icon="fluent:play-48-filled" class="icon" />
        <Icon
          v-show="isPlaying"
          icon="fluent:pause-48-filled"
          class="icon p-[2px]"
        />
      </div>

      <Icon icon="mingcute:rewind-forward-10-line" class="icon" />
      <Icon
        icon="iconoir:forward-solid"
        :class="['icon size-5 dark:text-white/90']"
        @click="playThePreviousBook"
      />
    </div>

    <div class="flex w-full items-center justify-between gap-x-2 mt-12">
      <small class="text-[12px] text-gray-400">
        {{ formatTime(currentTime) }}
      </small>
      <small class="text-[12px] text-gray-400">
        {{ formatTime(duration) }}
      </small>
    </div>
    <div>
      <SliderRoot
        v-model="sliderValue"
        class="relative flex items-center select-none touch-none w-full h-5"
        :max="100"
        :step="1"
      >
        <SliderTrack class="bg-gray-500/40 relative grow rounded-full h-1">
          <SliderRange class="absolute bg-grass8 rounded-full h-full" />
        </SliderTrack>
        <SliderThumb
          class="block w-4 h-4 p-1 bg-app-orange rounded-full hover:bg-app-orange shadow-sm focus:outline-none border-none outline-none"
          aria-label="Volume"
        />
      </SliderRoot>
    </div>
  </div>
  <footer
    class="fixed left-0 w-full py-3 text-small items-center gap-y-1 justify-between gap-x-2 z-100 flex min-h-12 tems-center bottom-0 px-5 shadow-lg border-t-white/50"
  >
    <RouterLink
      v-for="(item, index) in routes"
      :key="index"
      :to="{ name: 'Player' }"
      class="flex gap-y-1 flex-col items-center justify-center capitalize text-stone-500"
    >
      <Icon :icon="item.default" class="size-5" />
    </RouterLink>
  </footer>
</template>

<script lang="ts" setup>
import { Icon } from "@iconify/vue";
import { SliderRange, SliderRoot, SliderThumb, SliderTrack } from "reka-ui";
import { ref } from "vue";
import { playThePreviousBook } from "../../composibles/book.ts";
import { useGoBack } from "../../composibles/useRouter.ts";
const player = ref(new Audio());
const isPlaying = ref(false);

const currentTime = ref(0);
const duration = ref(0);
const sliderValue = ref([(currentTime.value / duration.value) * 100 || 0]);
const show = ref(false);

const togglePlaying = async () => {
  if (isPlaying.value) {
    pause();
  } else {
    play();
  }

  isPlaying.value = !isPlaying.value;
  // processes.isPlayingBook = !processes.isPlayingBook;
};

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
  player.value.pause();
};

const routes: Array<{ default: string; active: string; route: string }> = [
  {
    default: "material-symbols:speed-2x-rounded",
    active: "material-symbols:speed-2x-rounded",
    route: "Home",
  },
  {
    default: "fluent:search-12-regular",
    active: "fluent:search-16-filled",
    route: "Explore",
  },
  {
    default: "solar:playlist-bold",
    active: "solar:playlist-bold",
    route: "favorites",
  },
  {
    default: "fluent:bookmark-20-regular",
    active: "fluent:bookmark-20-filled",
    route: "bookmark",
  },
  {
    active: "fluent-mdl2:volume-2",
    default: "fluent-mdl2:volume-disabled",
    route: "player",
  },
];
</script>

<style scoped>
/* we will explain what these classes do next! */
.v-enter-active,
.v-leave-active {
  transition: opacity 0.5s ease;
}

.v-enter-from,
.v-leave-to {
  opacity: 0;
}
</style>

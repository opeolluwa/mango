<template>
  <footer
      :class="{'hidden': !fileName}"
      class="fixed bg-app-dark border-t border-t-gray-50/10 w-screen min-h-12 bottom-0 parent-element py-3 text-small flex items-center justify-between z-5000"
  >
    <audio
        ref="audioRef"
        src="audioSrc"
        preload="auto"
        class="hidden"
        @timeupdate="updateCurrentTime"
        @loadedmetadata="updateDuration"
    />

    <div class="flex gap-x-4 xs:hidden">
      <img
          src="../assets/cover.jpg"
          class="w-10 h-10 rounded col-span-1"
          alt=""
      />
      <div class="col-span-1">
        <small class="text-gray-400"> You are listening to... </small>
        <h6 class="text-[14px] leading-3.5">{{ fileName }}</h6>
      </div>
    </div>
    <div class="flex gap-x-4 items-center">
      <Icon icon="fluent:previous-48-filled" class="icon"/>

      <div
          class="w-8 h-8 border rounded-full border-gray-600 flex items-center justify-center text-white/80"
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
      <Icon icon="fluent:next-48-filled" class="icon"/>
      <Icon
          icon="gravity-ui:heart-fill"
          class="icon text-app-red"
          v-show="isLoved"
          @click="toggledIsLoved"
      />
      <Icon
          icon="gravity-ui:heart"
          class="icon text-gray-500"
          v-show="!isLoved"
          @click="toggledIsLoved"
      />
    </div>
    <div class="flex w-[40%] items-center gap-x-2">
      <div class="text-[12px] text-gray-400">
        {{ formatTime(currentTime) }}
      </div>
      <ProgressBar
          class="w-4/5"
          :progress="(currentTime / duration) * 100 || 0"
      />
      <div class="text-[12px] text-gray-400">
        {{ formatTime(duration) }}
      </div>
    </div>

    <div class="flex w-[10%] items-center gap-x-2">
      <Icon icon="fluent-mdl2:volume-3" class="icon"/>
      <!-- <input type="range" class="bg-app-orange" /> -->
      <ProgressBar class="w-4/5" :progress="volume * 100 || 0"/>
    </div>
    <div class="gap-x-2 items-center hidden">
      <Icon icon="iconamoon:playlist-shuffle-fill" class="size-5"/>
      <Icon icon="iconamoon:playlist-repeat-list-light" class="size-5"/>
      <Icon icon="solar:playlist-outline" class="size-5"/>
    </div>
  </footer>
</template>

<script lang="ts" setup>
import {Icon} from "@iconify/vue";
import ProgressBar from "./ProgressBar.vue";
import {computed, ref} from "vue";
import {useCurrentBook} from "../stores/book.ts";
import {useBookProcesses} from "../stores/actions.ts";
import {pauseAudioBook, playAudioBook} from "../hooks/book.ts";

const store = useCurrentBook();
const processes = useBookProcesses();

const fileName = computed(() => store.currentBook?.fileName);
const isPlaying = computed(() => processes.isPlayingBook);

const isLoved = ref(false);
const audioRef = ref<HTMLAudioElement | null>(null);
const currentTime = ref(0);
const duration = ref(0);
const volume = ref(0.25);

// audioRef.value?.volume = volume.value;
const toggledIsLoved = () => {
  isLoved.value = !isLoved.value;
};

const togglePlaying = () => {
  if (isPlaying) {
    pauseAudioBook();
  } else {
    playAudioBook(String(store.currentBook?.fileName));
  }
};

const updateCurrentTime = () => {
  if (audioRef.value) {
    currentTime.value = audioRef.value.currentTime;
  }
};

const updateDuration = () => {
  if (audioRef.value) {
    duration.value = audioRef.value.duration;
  }
};

const formatTime = (time: number): string => {
  const minutes = Math.floor(time / 60);
  const seconds = Math.floor(time % 60);
  return `${String(minutes).padStart(2, "0")}:${String(seconds).padStart(
      2,
      "0"
  )}`;
};
</script>

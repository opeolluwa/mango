<template>
  <div id="wrapper">
    <div
      class="flex flex-col items-center gap-2 justify-evenly absolute left-0 right-0 bottom-5 mb-4 w-full"
    >
      <audio
        ref="audioRef"
        :src="audioSrc"
        preload="auto"
        class="hidden"
        @timeupdate="updateCurrentTime"
        @loadedmetadata="updateDuration"
      />

      <div id="info" class="text-center mb-24  top-[50%]">
        <p class="leading-loos">You are listening to...</p>
        <h2 class="text-xl font-medium overflow-hidden text-ellipsis">
          {{ fileName }}
        </h2>
      </div>

      <div id="controls" class="flex items-center justify-evenly w-2/4">
        <button
          class="size-6 disabled:text-accent/95 control"
          aria-label="Previous"
        >
          <BackwardIcon />
        </button>

        <button
          class="bg-accent-secondary text-accent flex justify-center items-center rounded-full size-12 active:scale-75 transition-all duration-75 ease-linear p-[5px]"
        >
          <PlayIcon
            class="size-6 transition duration-150 ease-in-out"
            v-show="!isPlaying"
            @click="playAudio"
          />
          <PauseIcon
            class="size-6 transition duration-150 ease-in-out"
            v-show="isPlaying"
            @click="pauseAudio"
          />
        </button>

        <button
          class="size-6 disabled:text-accent/95 control"
          aria-label="Next"
        >
          <ForwardIcon />
        </button>
      </div>

      <div class="flex justify-between gap-x-2 items-center w-full">
        <small> {{ formatTime(currentTime) }} </small>
        <ProgressBar
          :progress="(currentTime / duration) * 100 || 0"
          class="mb-[1.2px]"
        />
        <small>{{ formatTime(duration) }}</small>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import {
  BackwardIcon,
  ForwardIcon,
  PauseIcon,
  PlayIcon,
} from "@heroicons/vue/24/solid";
import ProgressBar from "./ProgressBar.vue";

const props = defineProps<{
  fileName: string;
  audioSrc: string;
}>();

const { fileName, audioSrc } = props;
const isPlaying = ref(false);
const audioRef = ref<HTMLAudioElement | null>(null);
const currentTime = ref(0);
const duration = ref(0);
const playAudio = () => {
  audioRef.value?.play().catch((err) => {
    console.error("Failed to play audio:", err);
  });
  isPlaying.value = !isPlaying.value;
};

const pauseAudio = () => {
  audioRef.value?.pause();
  isPlaying.value = !isPlaying.value;
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

<style scoped>
@reference "../assets/styles.css";

#wrapper > div {
  @apply px-4 py-3;
}

#wrapper.isActive {
  @apply block;
}

.control {
  @apply cursor-pointer hover:opacity-75 hover:scale-95 transition-all duration-75 ease-linear;
}
</style>

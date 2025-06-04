<template>
  <template v-if="!empty">
    <div
      class="overflow-hidden bg-secondary bg-center-center bg-blend-multiply bg-cover bg-no-repeat relative flex flex-col justify-center items-center px-4 h-[90%] text-gray-400"
    >
      <h2 class="text-xl font-bold leading-10 prose-xl">
        No audio book has been created
      </h2>
      <small class="text-center mb-2"
        >Your audio books will appear as soon as you begin to add them</small
      >
      <button
        class="bg-app-orange text-white btn-lg inline-flex gap-x-2 rounded items-center px-8 py-2 mt-2 cursor-pointer shadow-md transition-colors duration-200 ease-linear hover:opacity-95 hover:scale-95 control"
        @click="useCreateNewBook"
      >
        <Icon icon="ic:round-plus" class="size-8" />
        Create
      </button>
    </div>
  </template>
  <template>
    <ColumnLayout>
      <div class="grid grid-cols-12 h-64 mt-16 gap-12">
        <div
          class="rounded-lg shadow shadow-app-gray py-6 bg-linear-50 from-app-orange/90 from-20% via-app-gray to-app-dark col-span-4 h-full px-4"
        >
          <small class="text-[12px] text-gray-400"
            >69 tracks . 4 hours 37 minutes</small
          >
          <h2
            class="text-leadning-loose prose-h1:first-letter:capitalize text-white/90"
          >
            Playlist for the day
          </h2>
        </div>

        <div
          class="rounded-lg col-span-8 h-full px-4 shadow-app-gray py-6 bg-blend-multiply bg-[url(../assets/cover.jpg)] bg-center bg-cover bg-app-gray/90 bg-no-repeat relative"
        >
          <div class="flex justify-between">
            <small class="text-[12px] text-gray-400"
              >69 tracks . 4 hours 37 minutes</small
            >
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
          <h2>Bands of sacrifice</h2>

          <div
            class="size-16 absolute top-[calc(50%-1rem)] left-[calc(50%-var(--spacing)*16)] rounded-full flex items-center justify-center bg-white/10"
          >
            <Icon icon="fluent:play-48-filled" class="size-6" />
          </div>
          <ProgressBar :progress="60" class="absolute bottom-1.5 w-[80%]" />
        </div>
      </div>

      <div>
        <ul class="flex items-center gap-x-6 w-full">
          <li
            v-for="(tab, index) in tabs"
            :key="index"
            class="first-letter:capitalize text-gray-400/50 border-t-2 border-t-transparent hover:border-t-app-orange hover:text-app-orange pt-4 cursor-pointer"
          >
            {{ tab.label }}
          </li>
        </ul>

        <div class="flex flex-col gap-y-4 mt-4">
          <AudioBook
            class=""
            v-for="(book, index) in audioBooks"
            :key="index"
            :file-name="book.fileName"
            :duration="book.playBackDuration.toString()"
            :date-last-played="new Date().toLocaleDateString()"
          />
        </div>
      </div> </ColumnLayout
  ></template>
</template>

<script lang="ts" setup>
import { Icon } from "@iconify/vue";
import { ref, watch } from "vue";
import AudioBook from "../components/AudioBook.vue";
import ColumnLayout from "../components/layouts/ColumnLayout.vue";
import ProgressBar from "../components/ProgressBar.vue";
import { useMusicLibary } from "../stores/library.ts";
import { type AudioBook as AudioBookInterface } from "../../src-tauri/bindings/AudioBook.ts";
import { useCreateNewBook } from "../hooks/createBook.ts";
const empty = ref(false);
const musicStore = useMusicLibary();
const audioBooks = ref<Array<AudioBookInterface>>();

watch(audioBooks, async () => {
  audioBooks.value = musicStore.audioBooks;
});

const isLoved = ref(false);
const toggledIsLoved = () => {
  isLoved.value = !isLoved.value;
};
const tabs: { route: string; label: string }[] = [
  {
    route: "",
    label: "Playlist",
  },
  {
    route: "",
    label: "artist",
  },
  {
    route: "",
    label: "albulm",
  },
  {
    route: "",
    label: "friend's playlist",
  },
];
</script>

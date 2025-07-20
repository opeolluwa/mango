<template>
  <template v-if="!emptyLibrary">
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
        @click="createNewBook"
      >
        <Icon icon="ic:round-plus" class="size-8" />
        Create
      </button>
    </div>
  </template>
  <template v-else>
    <div class="flex flex-col overflow-hidden">
      <div class="flex justify-between">
        <div>
          <h2 class="text-4xl text-app-dark/90 dark:text-gray-400">
            Olatunde!
          </h2>
          <VueGreetings class="text-sm text-gray-400 dark:text-gray-400/60" />
        </div>

        <AvatarRoot
          class="bg-blackA3 inline-flex size-[35px] shadow select-none items-center justify-center overflow-hidden rounded-full align-middle border-app-orange/50 border-2"
        >
          <AvatarImage
            class="h-full w-full rounded-[inherit] object-cover"
            src="https://images.unsplash.com/photo-1492633423870-43d1cd2775eb?&w=128&h=128&dpr=2&q=80"
            alt="Colm Tuite"
          />
          <AvatarFallback
            class="text-grass11 dark:text-stone-300 leading-1 flex h-full w-full items-center justify-center bg-white dark:bg-stone-800 text-sm font-medium"
            :delay-ms="600"
          >
            CT
          </AvatarFallback>
        </AvatarRoot>
      </div>

      <div class="mt-12 mb-10 flex gap-4 w-screen overflow-x-scroll">
        <!-- <small class="text-gray-600">Continue listening</small> -->
        <img
          v-for="(image, index) in images"
          :key="index"
          :src="image.src"
          :alt="image.alt"
          class="contain w-2/5 h-[150px] rounded"
        />
      </div>

      <div class="mb-10 hidden">
        <small class="text-gray-400">Recent books</small>
        <div class="flex flex-col gap-y-4 mt-4">
          <AudioBook
            class=""
            v-for="(book, index) in musicPlaylist"
            :key="index"
            :file-name="String(book.duration)"
            duration="0"
            :date-last-played="new Date().toLocaleDateString()"
          />
        </div>
      </div>
    </div>
  </template>
</template>

<script lang="ts" setup>
import { Icon } from "@iconify/vue";
import { AvatarFallback, AvatarImage, AvatarRoot } from "reka-ui";
import { computed, ref } from "vue";
import AudioBook from "../../components/AudioBook.vue";
import VueGreetings from "../../components/uiBlocks/VueGreetings.vue";
import { createNewBook } from "../../composibles/book.ts";
import { useAudioBookLibrary } from "../../stores/library.ts";

const musicPlaylist = ref([
  {
    fileName: "Bohemian_Rhapsody_Queen.mp3",
    duration: "5:55",
    dateLastPlayed: "2025-07-19T14:32:15.000Z",
  },
  {
    fileName: "Shape_of_You_Ed_Sheeran.mp3",
    duration: "3:53",
    dateLastPlayed: "2025-07-18T09:45:22.000Z",
  },
  {
    fileName: "Blinding_Lights_The_Weeknd.mp3",
    duration: "3:20",
    dateLastPlayed: "2025-07-20T16:18:44.000Z",
  },
  {
    fileName: "Watermelon_Sugar_Harry_Styles.mp3",
    duration: "2:54",
    dateLastPlayed: "2025-07-17T20:12:08.000Z",
  },
  {
    fileName: "Good_4_U_Olivia_Rodrigo.mp3",
    duration: "2:58",
    dateLastPlayed: "2025-07-20T11:27:33.000Z",
  },
]);

const images = ref([
  { src: "/01.jpg", alt: "" },
  { src: "/02.jpg", alt: "" },
  { src: "/03.jpg", alt: "" },
  { src: "/04.jpg", alt: "" },
]);
const library = useAudioBookLibrary();

const emptyLibrary = computed(() => library.audioBooks.length == 0);
</script>

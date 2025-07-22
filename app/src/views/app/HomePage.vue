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
    <div class="flex flex-col overflow-scroll pb-48">
      <div class="flex justify-between">
        <div>
          <h2 class="text-2xl font-black text-app-dark/90 dark:text-gray-400">
            Hey, Olatunde! 👋
          </h2>
          <VueGreetings class="leading-5 text-gray-600 dark:text-gray-400/60" />
        </div>
      </div>

      <dic class="mt-12 hidden">
        <h3 class="text-stone-400 text mb-2 dark:text-gray-400/60">
          Continue listening
        </h3>
        <div
          class="mb-10 flex gap-x-4 w-screen overflow-x-scroll pr-24"
          style="scroll-behavior: smooth"
        >
          <img
            v-for="(image, index) in images"
            :key="index"
            :src="image.src"
            :alt="image.alt"
            class="contain h-[200px] shadow-md rounded-lg hover:scale-95 transition-all duration-300"
          />
        </div>
      </dic>

      <section class="hidde\n mt-12">
        <h3 class="text-stone-400 text mb-2 dark:text-gray-400/60">
          Your books
        </h3>

        <div class="hiden">
          <AudioBook
            v-for="book in audiobooks"
            :key="book.id"
            :title="book.title"
            :author="book.author"
            :cover="book.cover"
            class="hover:shadow-md transition-shadow cursor-pointer"
          />
        </div>
      </section>
    </div>
  </template>
</template>

<script lang="ts" setup>
import { Icon } from "@iconify/vue";
import { computed, ref } from "vue";
import AudioBook from "../../components/AudioBook.vue";
import VueGreetings from "../../components/uiBlocks/VueGreetings.vue";
import { createNewBook } from "../../composibles/book.ts";
import { useAudioBookLibrary } from "../../stores/library.ts";
const audiobooks = ref([
  {
    id: 1,
    title: "The Great Gatsby",
    author: "F. Scott Fitzgerald",
    cover: "/02.jpg",
  },
  {
    id: 2,
    title: "To Kill a Mockingbird",
    author: "Harper Lee",
    cover: "/03.jpg",
  },

  {
    id: 4,
    title: "The Hobbit",
    author: "J.R.R. Tolkien",
    cover: "/04.jpg",
  },

  {
    id: 4,
    title: "The Hobbit",
    author: "J.R.R. Tolkien",
    cover: "/05.jpg",
  },

  {
    id: 4,
    title: "The Hobbit",
    author: "J.R.R. Tolkien",
    cover: "/06.jpg",
  },
]);

const images = ref([
  { src: "/02.jpg", alt: "" },
  { src: "/03.jpg", alt: "" },
  { src: "/04.jpg", alt: "" },
  { src: "/05.jpg", alt: "" },
  { src: "/06.jpg", alt: "" },
  { src: "/07.jpg", alt: "" },
]);
const library = useAudioBookLibrary();

const emptyLibrary = computed(() => library.audioBooks.length == 0);
</script>

<style scoped></style>

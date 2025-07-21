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

      <section>
        <h3 class="mb-2 font-mediumtext-gray-400 dark:text-gray-400/60">
          Continue listening
        </h3>
        <div
          class="mb-10 flex gap-4 w-screen overflow-x-scroll pr-24"
          style="scroll-behavior: smooth"
        >
          <img
            v-for="(image, index) in images"
            :key="index"
            :src="image.src"
            :alt="image.alt"
            class="contain h-[180px] rounded hover:resize"
          />
        </div>
      </section>

      <section>
        <h3 class="mb-2 font-mediumtext-gray-400 dark:text-gray-400/60">
          Recently added
        </h3>

        <div class="space-y-4">
          <AudioBook
            v-for="book in audiobooks"
            :key="book.id"
            :title="book.title"
            :author="book.author"
            :cover="book.cover"
            class="bg-white hover:shadow-md transition-shadow cursor-pointer"
          />
        </div>

        <!-- <small class="text-gray-400">Recent books</small>
        <div class="flex flex-col gap-y-4 mt-4">
          <AudioBook
            class=""
            v-for="(book, index) in musicPlaylist"
            :key="index"
            :file-name="String(book.duration)"
            duration="0"
            :date-last-played="new Date().toLocaleDateString()"
          />
        </div> -->
      </section>
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
const audiobooks = ref([
  {
    id: 1,
    title: "The Great Gatsby",
    author: "F. Scott Fitzgerald",
    cover:
      "https://images.unsplash.com/photo-1544947950-fa07a98d237f?w=64&h=64&fit=crop&crop=center",
  },
  {
    id: 2,
    title: "To Kill a Mockingbird",
    author: "Harper Lee",
    cover:
      "https://images.unsplash.com/photo-1512820790803-83ca734da794?w=64&h=64&fit=crop&crop=center",
  },
  {
    id: 3,
    title: "1984",
    author: "George Orwell",
    cover:
      "https://images.unsplash.com/photo-1481627834876-b7833e8f5570?w=64&h=64&fit=crop&crop=center",
  },
  // {
  //   id: 4,
  //   title: "Pride and Prejudice",
  //   author: "Jane Austen",
  //   cover:
  //     "https://images.unsplash.com/photo-1516979187457-637abb4f9353?w=64&h=64&fit=crop&crop=center",
  // },
  // {
  //   id: 5,
  //   title: "The Hobbit",
  //   author: "J.R.R. Tolkien",
  //   cover:
  //     "https://images.unsplash.com/photo-1507003211169-0a1dd7228f2d?w=64&h=64&fit=crop&crop=center",
  // },
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

<style scoped>
section {
  margin-top: 3.5rem;
}
</style>

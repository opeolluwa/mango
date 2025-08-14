<template>
  <template v-if="emptyLibrary">
    <div
      class="overflow-hidden bg-cover bg-no-repeat relative flex flex-col justify-center items-center px-4 h-[90%] text-gray-400"
    >
      <h2 class="text-xl font-bold leading-10 prose-xl">
        No audio book has been created
      </h2>
      <small class="text-center mb-2"
        >Your audio books will appear as soon as you begin to add them</small
      >
      <button
        class="bg-app-orange text-white btn-lg inline-flex gap-x-2 items-center px-8 py-2 mt-2 cursor-pointer shadow-md transition-colors duration-200 ease-linear hover:opacity-95 hover:scale-95 control rounded"
        @click="createNewBook"
      >
        <Icon icon="ic:round-plus" class="size-8" />
        Create
      </button>
    </div>
  </template>
  <template v-else>
    <div class="flex flex-col overflow-x-hidden pb-48">
      <div class="flex justify-between">
        <div>
          <h2 class="text-2xl font-black text-app-dark/90 dark:text-gray-200">
            Hey, Olatunde! 👋
          </h2>
          <VueGreetings class="leading-5 text-gray-600 dark:text-gray-400" />
        </div>

        <div class="col-span-4 flex gap-x-3 items-center">
          <AvatarRoot
            class="bg-blackA3 inline-flex size-8 select-none items-center justify-center overflow-hidden rounded-full align-middle"
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
      </div>

      <div class="mt-12">
        <h3 class="font-3xl text-gray-400 font-medium mb-3">
          Continue listening
        </h3>
        <div
          class="mb-10 flex gap-x-4 w-screen overflow-x-scroll pr-24"
          style="scroll-behavior: smooth"
        ></div>
      </div>

      <section class="mt-8">
        <h3 class="font-3xl text-gray-400 font-medium mb-2">Recently added</h3>

        <div class="hiden">
          <!-- <AudioBook
            v-for="book in audiobooks"
            :key="book.id"
            :title="book.title"
            :author="book.author"
            :cover="book.cover"
            class="hover:shadow-md transition-shadow cursor-pointer"
          /> -->
        </div>
      </section>
    </div>
  </template>
</template>

<script lang="ts" setup>
import { Icon } from "@iconify/vue";
import { AvatarFallback, AvatarImage, AvatarRoot } from "reka-ui";
// import { ref } from "vue";
// import AudioBook from "../../components/AudioBook.vue";
import VueGreetings from "../../components/uiBlocks/VueGreetings.vue";
import { createNewBook } from "../../composibles/book.ts";

// const audiobooks = ref([]);

// const images = ref([]);

const emptyLibrary = true;
// const emptyLibrary = computed(() => library.audioBooks.length == 0);
</script>

<style scoped></style>

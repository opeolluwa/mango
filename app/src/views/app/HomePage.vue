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
    <div class="flex flex-col">
      <div class="flex justify-between">
        <div>
          <h1 class="text-4xl text-gray-400 dark:text-gray-400/60">Hello,</h1>
          <h2 class="text-4xl text-app-dark/90 dark:text-gray-400">
            Olatunde!
          </h2>
          <VueGreetings class="text-sm text-app-dark/95 dark:text-gray-400/95 mt-[3px]" />
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

      <div class="mt-12 mb-10 hidden">
        <small class="text-gray-400">continue listening</small>
        <div
          class="card flex rounded-xl gap-3 mt-4 bg-gray-200 dark:bg-app-gray/70 py-4 px-2"
        >
          <img
            src="@/assets/test.jpg"
            class="contain w-[120px] h-[150px]"
            alt=""
          />
          <div class="text-gray-400">
            <h1 class="text-3xl">Half of a yellow sun</h1>
            <p>Chimamanda Ngozi</p>

            <small class="text-gray-700">33% complete</small>
          </div>
        </div>
      </div>

      <div class="mb-10 hidden">
        <small class="text-gray-400">Recent books</small>
        <div class="flex flex-col gap-y-4 mt-4">
          <!-- <AudioBook
          class=""
          v-for="(book, index) in audioBooks"
          :key="index"
          :file-name="String(book.title)"
          duration="0"
          :date-last-played="new Date().toLocaleDateString()"
        /> -->
        </div>
      </div>
    </div>
  </template>
</template>

<script lang="ts" setup>
import { Icon } from "@iconify/vue";
import { AvatarFallback, AvatarImage, AvatarRoot } from "reka-ui";
import { computed } from "vue";
import { createNewBook } from "../../composibles/book.ts";
import { useAudioBookLibrary } from "../../stores/library.ts";
import VueGreetings from "../../components/uiBlocks/VueGreetings.vue";

const library = useAudioBookLibrary();

const emptyLibrary = computed(() => library.audioBooks.length == 0);
</script>

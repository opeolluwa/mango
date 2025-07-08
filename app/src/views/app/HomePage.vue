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
          <h1 class="text-4xl text-gray-400/60">Hello,</h1>
          <h2 class="text-4xl text-gray-400">Emmanuel!</h2>
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

      <div class="mt-12 mb-10">
        <small class="text-gray-400">continue listening</small>
        <div class="card flex rounded-xl gap-3 mt-4 bg-app-gray/70 py-4 px-2">
          <img
            src="@/assets/test.jpg"
            class="contain w-[120px] h-[150px]"
            alt=""
          />
          <div class="text-gray-400">
            <h1 class="text-3xl">Barking up the wrong tree</h1>
            <p>Erick mamardor</p>

            <small class="text-gray-700">33% complete</small>
          </div>
        </div>
      </div>

      <div class="mb-10">
        <small class="text-gray-400">Recent books</small>
      </div>
    </div>

    <!-- 



    <div
      class="rounded-lg shadow w-full shadow-app-gray py-6 bg-linear-60 from-app-orange/90 from-20% via-app-dark to-app-orange h-48 px-4 mt-6"
    >
      <small class="text-[12px] text-gray-400"
        >69 tracks . 4 hours 37 minutes</small
      >
      <h2
        class="text-leading-loose prose-h1:first-letter:capitalize text-white/90"
      >
        Playlist for the day
      </h2>
    </div>

    <div
      class="rounded-lg h-48 px-4 hidden shadow-app-gray py-6 bg-blend-multiply bg-[url(../assets/cover.jpg)] bg-center bg-cover bg-app-gray/90 bg-no-repeat relative"
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

    <div>
      <ul class="flex items-center gap-x-6 w-full">
        <li
          v-for="(tab, index) in tabs"
          :key="index"
          class="first-letter:capitalize text-gray-400/50 border-b-2 border-b-transparent hover:border-b-app-orange hover:text-app-orange pt-4 cursor-pointer"
        >
          {{ tab.label }}
        </li>
      </ul>

      <div class="flex flex-col gap-y-4 mt-4">
        <AudioBook
          class=""
          v-for="(book, index) in audioBooks"
          :key="index"
          :file-name="String(book.title)"
          duration="0"
          :date-last-played="new Date().toLocaleDateString()"
        />
      </div> -->
    <!-- </div> -->
  </template>
</template>

<script lang="ts" setup>
import { Icon } from "@iconify/vue";
import { AvatarFallback, AvatarImage, AvatarRoot } from "reka-ui";
import { computed } from "vue";
import { createNewBook } from "../../hooks/book.ts";
import { useAudioBookLibrary } from "../../stores/library.ts";

const library = useAudioBookLibrary();

const emptyLibrary = computed(() => library.audioBooks.length == 0);
</script>

<template>
  <header class="flex gap-x-4 items-center justify-between">
    <div class="gap-x-3 items-center hidden lg:flex">
      <Icon
        icon="mingcute:left-fill"
        class="text-gray-400/50 size-5 flex items-center h-full icon"
        @click="goBack()"
      />
      <Icon
        icon="mingcute:right-fill"
        class="text-gray-400/50 size-5 flex items-center h-full icon"
        @click="goForward()"
      />
    </div>

      <Icon
        icon="material-symbols:menu-rounded"
        class="text-gray-400/50 size-6 flex items-center h-full "

      />

    <form
      action=""
      class="bg-gray-50/10 flex gap-x-2 items-center rounded-xl h-10 relative pl-4 w-3/5"
    >
      <Icon
        icon="fluent:search-16-filled"
        class="text-gray-400/50 size-5 flex items-center h-full"
      />
      <input
        type="search"
        class="w-full h-10 outline-none border-none pr-6 placeholder:text-gray-50/10 placeholder:text-sm placeholder:leading-loose"
        placeholder="Search by artist, song or albulm"
      />
    </form>
    <div class="col-span-4 flex gap-x-3 items-center">
      <!-- <Icon
      icon="mdi:bell"
      class="text-gray-400/50 size-5 flex items-center h-full"
    /> -->

      <Icon
        icon="mdi:book-plus-outline"
        class="text-gray-400/50 size-5 flex items-center h-full cursor-pointer"
        @click="createNewBook"
      />
      <RouterLink to="/notification">
        <Icon
          icon="mdi:bell-outline"
          class="text-gray-400/50 size-5 flex items-center h-full cursor-pointer"
        />
      </RouterLink>
    </div>
  </header>
</template>

<script lang="ts" setup>
import { Icon } from "@iconify/vue";
import { goBack, goForward } from "../../hooks/router";
import { createNewBook } from "../../hooks/book";
</script>

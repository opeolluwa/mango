<template>
  <heading class="flex gap-x-4 items-center justify-between">
    <div class="flex gap-x-3 items-center">
      <Icon
        icon="fic:outline-chevron-right"
        class="text-gray-400/50 size-5 flex items-center h-full"
      />
      <Icon
        icon="ic:outline-chevron-right"
        class="text-gray-400/50 size-5 flex items-center h-full"
      />
    </div>
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
    <div class="col-span-4 flex gap-x-3">
      <!-- <Icon
      icon="mdi:bell"
      class="text-gray-400/50 size-5 flex items-center h-full"
    /> -->

      <Icon
        icon="mdi:book-plus-outline"
        class="text-gray-400/50 size-6 flex items-center h-full cursor-pointer"
      />
      <Icon
        icon="mdi:bell-outline"
        class="text-gray-400/50 size-6 flex items-center h-full cursor-pointer"
      />
    </div>
  </heading>
</template>

<script lang="ts" setup>
import { Icon } from "@iconify/vue";
</script>

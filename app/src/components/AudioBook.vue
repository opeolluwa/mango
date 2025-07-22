<template>
  <div class="flex gap-y-4 items-center justify-between w-full py-2">
    <div class="flex items-center gap-x-4">
      <img :src="cover" class="size-8 rounded" alt="" />
      <div class="flex flex-col gap-y-[2px]">
        <h2 class="font-medium">
          {{ title }}
        </h2>
        <div class="text-gray-400 text-sm">
          {{ author }}
        </div>
      </div>
    </div>

    <Icon icon="fe:elipsis-v" class="size-5 text-gray-400 icon" />
  </div>
</template>

<style scoped></style>

<script lang="ts" setup>
import { Icon } from "@iconify/vue";
// import { playAudioBook } from "../composibles/book";

const props = defineProps<{
  title: string;
  author: string;
  cover: string;
}>();

const { cover, author, title } = props;
</script>

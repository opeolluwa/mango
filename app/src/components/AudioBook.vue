<template>
  <div class="flex gap-y-4 items-center justify-between w-full">
    <div class="flex items-center gap-x-4">
      <!-- <img src="../assets/book.png" class="size-8 rounded" alt="" /> -->
      <!-- <Gravatar email="somebody@somewhere.com" /> -->
      <h2 class="">
        {{ fileName }}
      </h2>
    </div>

    <div class="text-[12px] text-gray-400 ">
      {{ duration }}
    </div>

    <div class="text-[12px] text-gray-400 ">
      {{ dateLastPlayed }}
    </div>

    <div class="flex gap-x-3 justify-end text-white/70">
      <Icon
        icon="fluent:play-12-filled"
        class="size-5 icon"
        @click="playAudioBook(fileName)"
      />
      <Icon icon="fe:elipsis-v" class="size-5 text-gray-400 icon" />
    </div>
  </div>
</template>

<style scoped></style>

<script lang="ts" setup>
import { Icon } from "@iconify/vue";
import { playAudioBook } from "../composibles/book";

const props = defineProps<{
  fileName: string;
  duration: string;
  dateLastPlayed: string;
}>();

const { fileName } = props;
</script>

<template>
  <div class="flex gap-y-4 items-center justify-between w-full ">
    <div class=" flex items-center gap-x-4">
      <img src="../assets/cover.jpg" class="size-8 rounded" alt="" />
      <h2 class="text-white/80">
        {{ fileName }}
      </h2>
    </div>

    <div class="text-[12px] text-gray-400 ">
      {{ duration }}
    </div>

    <div class="text-[12px] text-gray-400 ">
      {{ dateLastPlayed }}
    </div>
    
    <div class=" flex gap-x-3 justify-end  text-white/70">
      <Icon icon="fluent:play-12-filled" class="size-5" />
      <Icon icon="fe:elipsis-v" class="size-5 text-gray-400" />
    </div>

    
  </div>
</template>

<style scoped></style>

<script lang="ts" setup>
import { Icon } from "@iconify/vue";

const props = defineProps<{
  fileName: string;
  duration: string;
  dateLastPlayed: string;
}>();

const { fileName, duration, dateLastPlayed } = props;
</script>

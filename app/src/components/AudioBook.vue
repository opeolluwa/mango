<template>
  <RouterLink
    :to="{ name: 'Player', params: { src, title: name } }"
    class="flex gap-y-4 items-center justify-between w-full py-2"
  >
    <div class="grid grid-cols-12 items-center gap-x-4">
      <div
        class="col-span-8 flex flex-col -gap-y-[4px] text-app-dark dark:text-gray-200 truncate w-[85%] text-clip"
      >
        <h2 class="flex">
          <Icon icon="fe:elipsis-v" class="size-5 text-gray-400 icon" />
          {{ name.replaceAll("-", " ") }}
        </h2>
        <div class="text-gray-400 text-sm hidden">
          {{ author }}
        </div>
      </div>
    </div>

    <did class="flex gap-x-4 items-center col-span-4">
      <small>{{ "3:50" }}</small>
      <Icon icon="fe:elipsis-v" class="size-5 text-gray-400 icon" />
    </did>
  </RouterLink>
</template>

<script lang="ts" setup>
import { Icon } from "@iconify/vue";
const props = defineProps<{
  name: string;
  author: string;
  duration: string;
  src: string;
}>();

const { name, author, src } = props;
</script>

<style scoped></style>

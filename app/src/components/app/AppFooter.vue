<template>
  <footer
    class="fixed bg-app-dark border-t border-t-gray-50/10 w-screen min-h-12 bottom-0 parent-element py-3 text-small flex items-center justify-between z-5000"
  >
    <div class="flex gap-x-2">
      <img
        src="./../../assets/cover.jpg"
        class="w-10 h-10 rounded col-span-1"
        alt=""
      />
      <div class="col-span-1">
        <small class="text-gray-400"> You are listening to... </small>
        <h6 class="leading-">Welcome to horrowwook</h6>
      </div>
    </div>
    <div class="flex gap-x-4 items-center">
      <Icon icon="fluent:previous-48-filled" class="icon" />

      <div
        class="w-8 h-8 border rounded-full border-gray-600 flex items-center justify-center"
      >
        <!-- <Icon icon="fluent:play-48-filled" class="icon" /> -->
        <Icon icon="fluent:pause-48-filled" class="icon p-[2px]" />
      </div>
      <Icon icon="fluent:next-48-filled" class="icon" />
      <Icon icon="gravity-ui:heart-fill" class="icon text-app-red" />
      <!-- <Icon icon="gravity-ui:heart" class="icon text-gray-500" /> -->
    </div>
    <div class="flex w-[40%] items-center gap-x-2">
      <ProgressBar class="w-4/5" progress="40" />
      <div class="text-[12px] text-gray-400">1:55/3:47</div>
    </div>

    <div class="flex w-[10%] items-center gap-x-2">
      <Icon icon="fluent-mdl2:volume-3" class="icon" />
      <ProgressBar class="w-4/5" progress="40" />
    </div>
    <div class="flex gap-x-2 items-center">
      <Icon icon="iconamoon:playlist-shuffle-fill" class="size-6" />
      <Icon icon="iconamoon:playlist-repeat-list-light" class="size-6" />
      <Icon icon="solar:playlist-outline" class="size-5" />
    </div>
  </footer>
</template>

<script lang="ts" setup>
import { Icon } from "@iconify/vue";
import ProgressBar from "../../components/ProgressBar.vue";
</script>

<template>
  <footer
    class="fixed border-t-gray-100/10 w-full py-3 text-small items-center gap-y-1 justify-between gap-x-2 z-5000 flex bg-app-dark/70 shadow-app-orange/60 border-app-orange/10 border shadow min-h-12 tems-center px-4 bottom-0"
  >
    <div
      v-for="(item, index) in routes"
      class="flex gap-y-1 flex-col items-center justify-center capitalize text-stone-500 text-sm"
    >
      <template v-if="index == 2">
        <div class="bg-app-orange rounded-full shadow shadow-app-dark">
             <Icon :icon="item.active" :key="index" class="size-10" />
        </div> 

      </template>
      <template v-else>
        <Icon :icon="item.default" :key="index" class="size-5" />
      </template>
        <!-- <Label class="text-small text-stone-600">{{ item.label }}</Label> -->

    </div>
  </footer>
</template>

<script lang="ts" setup>
import { Icon } from "@iconify/vue";
const routes: Array<{ default: string; active: string; label: string }> = [
  {
    default: "mage:home",
    active: "mage:home-fill",
    label: "Home",
  },
  {
    default: "fluent:search-12-regular",
    active: "fluent:search-16-filled",
    label: "Explore",
  },

  {
    default: "ic:baseline-plus",
    active: "ic:baseline-plus",
    label: "favorites",
  },
  {
    default: "mingcute:heart-line",
    active: "mingcute:heart-fill",
    label: "favorites",
  },
  {
    active: "solar:library-bold-duotone",
    default: "solar:library-linear",
    label: "library",
  },
];
</script>

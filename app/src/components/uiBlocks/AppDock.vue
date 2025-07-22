<template>
  <footer
    class="fixed border-gray-100/50 dark:border-none w-full py-3 items-center gap-y-1 justify-between gap-x-2 z-4000 flex border min-h-12 tems-center px-4 bottom-0    shadow-inner dark:border-gray-100  dark:border-t-8 bg-white"
  >
    <RouterLink
      :to="{ name: item.route }"
      v-for="(item, index) in routes"
      class="flex gap-y-1 flex-col items-center justify-center capitalize text-gray-500 text-sm"
      @click="updateTab(index)"
    >
      <template v-if="index == currentTab">
        <Icon :icon="item.active" :key="index" class="size-5 text-app-orange" />
        <Label class="text-app-orange"> {{ item.route }}</Label>
      </template>
      <template v-else>
        <Icon :icon="item.default" :key="index" class="size-5 icon" />
        <Label class="capitalize"> {{ item.route }}</Label>
      </template>
    </RouterLink>
  </footer>
</template>

<script lang="ts" setup>
import { Icon } from "@iconify/vue";
import { ref } from "vue";

const currentTab = ref(0);
const updateTab = (index: number) => (currentTab.value = index);

const routes: Array<{ default: string; active: string; route: string }> = [
  {
    default: "mage:home",
    active: "mage:home-fill",
    route: "Home",
  },
  {
    default: "fluent:clock-16-regular",
    active: "fluent:clock-16-filled",
    route: "Recent",
  },

  {
    default: "mage:notification-bell",
    active: "mage:notification-bell-fill",
    route: "Notification",
  },
  {
    active: "mage:settings-fill",
    default: "mage:settings",
    route: "Settings",
  },
];
</script>

<template>
  <AppNavigation
    v-if="showSideNav"
    class="fixed left-0 bottom-16 w-[70vw] z-[6000] dark:bg-app-dark bg-white"
  />

  <div
    class="flex gap-4 min-h-12 items-center justify-between absolute top-0 w-screen py-3 pr-12 bg-white z-500 dark:bg-app-dark"
  >
    <template v-if="route.meta.isHome">
      <div class="flex gap-x-2 align-center">
        <Icon
          icon="material-symbols:menu-rounded"
          :class="['icon size-5 dark:text-white/90']"
          @click="showSideNav != showSideNav"
        />
        <UserCard :editable="false" :avatar-size="12" :show-text="false" />
      </div>
    </template>

    <template v-else>
      <div class="flex gap-x-2 align-center">
        <Icon
          icon="fluent:chevron-left-32-filled"
          :class="['icon size-5 dark:text-white/90']"
          @click="useGoBack"
        />
        <span class="font-medium"> {{ label }}</span>
      </div>
    </template>
    <slot name="headerIcon" />
  </div>

  <div class="mt-12 min-h-screen">
    <slot />
  </div>
</template>

<script lang="ts" setup>
import { ref, watch } from "vue";
import { useGoBack } from "../../composibles/useRouter";
import { Icon } from "@iconify/vue";
import { useRoute } from "vue-router";
import UserCard from "../../components/settings/UserCard.vue";
import AppNavigation from "../../components/uiBlocks/AppNavigation.vue";

const route = useRoute();
const label = ref<string>("") || "";
const showSideNav = ref(false);

watch(
  () => route.meta.label,
  (newLabel) => {
    label.value = String(newLabel ?? "");
  },
  { immediate: true }
);
</script>

<template>
  <AppNavigation
    v-if="showSideNav"
    class="fixed left-0 bottom-16 w-[70vw] z-[6000] dark:bg-app-dark bg-white"
  />
  <div class="flex justify-between min-h-20 absolute top-0 w-screen py-3 pr-12">
    <Icon
      icon="fluent:chevron-left-32-filled"
      :class="['icon size-5 dark:text-white/90']"
      @click="useGoBack"
    />

    <Icon icon="tabler:dots" :class="['icon size-5 dark:text-white/90']" @click="showSideNav!=showSideNav" />
  </div>

  <div class="mt-12">
    <slot />
  </div>
</template>

<script lang="ts" setup>
import { ref } from "vue";
import { useGoBack } from "../../composibles/router";
import { Icon } from "@iconify/vue";
const showSideNav = ref(false);
</script>

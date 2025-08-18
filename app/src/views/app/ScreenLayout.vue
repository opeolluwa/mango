<template>

  <div class="flex justify-between min-h-20 absolute top-0 w-screen py-3 pr-12">
    <Icon
      icon="fluent:chevron-left-32-filled"
      :class="['icon size-5 dark:text-white/90']"
      @click="useGoBack"
    />
    {{ label }}
    <Icon
      icon="tabler:dots"
      :class="['icon size-5 dark:text-white/90']"
      @click="showSideNav != showSideNav"
    />
  </div>

  <div class="mt-12">
    <slot />
  </div>
</template>

<script lang="ts" setup>
import { ref, watch } from "vue";
import { useGoBack } from "../../composibles/router";
import { Icon } from "@iconify/vue";
import { useRoute } from "vue-router";

const route = useRoute();
const label = ref<string>("") || "";

watch(
  () => route.meta.label,
  (newLabel) => {
    label.value = String(newLabel ?? "");
  },
  { immediate: true }
);
const showSideNav = ref(false);
</script>

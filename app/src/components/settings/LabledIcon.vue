<template>
  <RouterLink
    :to="{ name: route }"
    class="grid gap-x-4 grid-cols-12 items-center justify-between w-full"
  >
    <Icon :icon="icon" :class="['size-5 dark:text-white/90 col-span-2']" />
    <div class="col-span-8">
      <h2 class="font-3xl font-medium capitalize">
        {{ title }}
      </h2>
      <small class="text-gray-400 first-letter:capitalize">{{
        description
      }}</small>
    </div>
    <Icon
      v-if="route"
      icon="fluent:chevron-right-32-filled"
      :class="['icon size-5 dark:text-white/90 col-span-2']"
    />
  </RouterLink>
</template>

<script lang="ts" setup>
import { Icon } from "@iconify/vue";

defineProps<{
  icon: string;
  title: string;
  description?: string;
  route?: string;
}>();
</script>

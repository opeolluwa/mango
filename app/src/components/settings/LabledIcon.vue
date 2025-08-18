<template>
  <div class="grid gap-x-4 grid-cols-12 items-center" @click="logOut">
    <Icon :icon="icon" :class="['size-6 dark:text-white/90 col-span-2']" />
    <div class="col-span-8">
      <h2 class="font-3xl font-medium capitalize">
        {{ title }}
      </h2>
      <small class="text-gray-400 first-letter:capitalize">{{
        description
      }}</small>
    </div>
    <Icon
      icon="fluent:chevron-right-32-filled"
      :class="['icon size-5 dark:text-white/90 col-span-2']"
    />
  </div>
</template>

<script lang="ts" setup>
defineProps({
  icon: String,
  description: String,
  title: String,
});
</script>

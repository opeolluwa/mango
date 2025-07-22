<script setup lang="ts">
import { RouterLink } from "vue-router";
</script>

<template>
  <RouterLink
    :to="{ name: 'Player' }"
    class="bg-gray-200 dark:bg-app-gray min-h-12 w-full absolute left-0 bottom-16 py-2 rounded px-4"
  >
    hey
  </RouterLink>
</template>

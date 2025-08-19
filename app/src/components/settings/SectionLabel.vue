<template>
  <h2 class="text-gray-400 font-medium  mb-2">
    {{ label }}
  </h2>
</template>

<script lang="ts" setup>
defineProps<{
  label: string;
}>();
</script>

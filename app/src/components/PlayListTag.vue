<template>
  <div class="flex text-sm gap-x-6 parent-element items-center">
    <span
      :style="{ backgroundColor: color, fontSize: 12 }"
      class="w-2 h-2 rounded-full"
    ></span>
    {{ name }}
  </div>
</template>

<script setup lang="ts">
const props = defineProps<{
  color: string;
  name: string;
}>();

const { name, color } = props;
</script>

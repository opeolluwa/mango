<template>
  <div
    class="flex items-center gap-x-4 border-l-3 border-l-transparent cursor-pointer hover:text-app-orange hover:border-l-app-orange hover:border-l-3 px-6 text-sm"
  >
    <Icon :icon="icon" class="size-5" /> {{ label }}
  </div>
</template>

<script lang="ts" setup>
import { Icon } from "@iconify/vue";
const props = defineProps<{
  icon: string;
  label: string;
}>();

const { icon, label } = props;
</script>

<style scoped></style>

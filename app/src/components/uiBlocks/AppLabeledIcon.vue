<template>
  <template v-if="route">
    <RouterLink
      :to="route"
      class="flex items-center gap-x-4 border-l-3 border-l-transparent cursor-pointer hover:text-app-orange hover:border-l-app-orange hover:border-l-3 px-6 text-sm hover:border-l-linear-to-r from-app-orange from-20% to-app-dark"
    >
      <Icon :icon="icon" class="size-5" /> {{ label }}
    </RouterLink>
  </template>
  <template v-else>
    <div
      class="flex items-center gap-x-4 border-l-3 border-l-transparent cursor-pointer hover:text-app-orange hover:border-l-app-orange hover:border-l-3 px-6 text-sm hover:border-l-linear-to-r from-app-orange from-20% to-app-dark"
    >
      <Icon :icon="icon" class="size-5" /> {{ label }}
    </div>
  </template>
</template>

<script lang="ts" setup>
import { Icon } from "@iconify/vue";
const props = defineProps<{
  icon: string;
  label: string;
  route?: string;
}>();

const { icon, label } = props;
</script>

<style scoped></style>

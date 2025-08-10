<template>
  <RouterLink
    :to="href"
    class="py-4 px-8 hover:scale-90 transition delay-75 duration-200 ease-initial block"
  >
    <slot />
  </RouterLink>
</template>

<script lang="ts" setup>
import { RouterLink } from "vue-router";

defineProps<{ href: string }>();
</script>

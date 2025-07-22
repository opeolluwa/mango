<template>
  <RouterView v-slot="{ Component }">
    <Transition name="fade" mode="out-in">
      <component :is="Component" />
    </Transition>
  </RouterView>
  <SensationalTint class="absolute inset-x-2 bottom-1 -z-10 transform-gpu overflow-hidden blur-3xl opacity-30 dark:hidden" />
</template>

<script lang="ts" setup>
import SensationalTint from "@/components/uiBlocks/SensationalTint.vue";
</script>

<style>
h1 {
  font-weight: 400;
  line-height: 45px;
}

h1 + p {
  margin-top: 10px;
}


h1.text-7xl {
  line-height: 1;
}
</style>

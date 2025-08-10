<template>
  <div class="overflow-x-hidden">
    <main class="layout pt-3 parent-element w-screen dark:bg-app-dark">
      <RouterView v-slot="{ Component }">
        <Transition name="fade">
          <Component :is="Component" />
        </Transition>
      </RouterView>
    </main>
    <SensationalTint
      class="absolute inset-x-2 top-24 -z-10 transform-gpu overflow-hidden blur-3xl opacity-70"
    />
  </div>
</template>

<script lang="ts" setup>
import SensationalTint from "@/components/uiBlocks/SensationalTint.vue";
</script>

<style scoped>
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.5s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>

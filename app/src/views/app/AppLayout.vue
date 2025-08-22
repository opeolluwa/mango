<template>
  <div class="overflow-x-hidden">

    <main class="layout parent-element w-screen dark:bg-app-dark mt-6">
      <RouterView v-slot="{ Component }">
        <Transition name="fade">
          <Component :is="Component" />
        </Transition>
      </RouterView>
    </main>
    <AppDock />
  </div>
</template>

<script lang="ts" setup>
import AppDock from "../../components/uiBlocks/AppDock.vue";


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

<template>
  <div
    id="container"
    class="bg-app-dark h-screen w-screen text-white grid grid-cols-12 min-w-[600px]"
  >
    <AppNavigation />
    <main class="col-span-6 layout parent-element">
      <RouterView />
    </main>
    <aside class="col-span-3"></aside>
    <AppFooter />
  </div>
</template>

<script lang="ts" setup>
import AppFooter from "./components/app/AppFooter.vue";
import AppNavigation from "./components/app/AppNavigation.vue";
</script>
<style scoped>
@reference "./assets/styles.css";
</style>

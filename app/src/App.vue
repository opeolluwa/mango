<template>
  <div
    id="container"
    class="bg-app-dark h-screen w-screen text-white grid grid-cols-12 min-w-[600px]"
  >
    <AppNavigation />
    <main class="col-span-6 layout parent-element">
      <AppHeading />
      <RouterView />
    </main>
    <aside class="col-span-3"></aside>
    <AppFooter
      :file-name="fileName"
      :audio-src="audioSource"
    />
  </div>
</template>

<script lang="ts" setup>
import AppFooter from "./components/uiBlocks/AppFooter.vue";
import AppNavigation from "./components/uiBlocks/AppNavigation.vue";
import AppHeading from "./components/uiBlocks/AppHeading.vue";
import { ref } from "vue";
const audioSource = ref("source.mp3");
const fileName = ref("the great asb.pdf");
</script>
<style scoped>
@reference "./assets/styles.css";
</style>

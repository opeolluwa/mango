<template>
  <AppLayout>
    <AppNavigation class="col-span-3 xl:col-span-2" />
    <AppMain class="col-span-9 xl:col-span-8" />
    <aside class="hidden xl:block xl:col-span-2">hey</aside>
    <AppFooter :file-name="fileName" :audio-src="audioSource" />
  </AppLayout>
</template>

<script lang="ts" setup>
import { ref } from "vue";
import AppLayout from "./components/layouts/ApplAyout.vue";
import AppFooter from "./components/uiBlocks/AppFooter.vue";
import AppMain from "./components/uiBlocks/AppMain.vue";
import AppNavigation from "./components/uiBlocks/AppNavigation.vue";
const audioSource = ref("source.mp3");
const fileName = ref("the great asb.pdf");
</script>
<style scoped>
@reference "./assets/styles.css";
</style>

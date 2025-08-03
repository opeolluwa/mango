<template>
  <div
    class="px-8 flex flex-col h-screen pb-[15vh] justify-end w-screen relative layout"
  >
    <!-- <img src="/screen-two.png" alt="" /> -->
     
        
    <h1
      class="text-4xl mt-6 capitalize leading-loose dark:text-white/70 w-[90%]"
      style="line-height: 50px"
    >
      <span class="font-bold">Elevate</span>
      your reading with <span class="font-bold">quick insight</span>
    </h1>

    <SensationalTint />

    <RouterLink
      :to="{ name: 'SignUp' }"
      class="relative -bottom-14 text-white font-medium bg-app-orange rounded-lg py-4 text-center hover:animate-pulse mt-8"
      @click="initializeAppSettings"
    >
      Get started
    </RouterLink>
  </div>
</template>
<script lang="ts" setup>
import SensationalTint from "@/components/uiBlocks/SensationalTint.vue";
import { invoke } from "@tauri-apps/api/core";
import { RouterLink } from "vue-router";

const initializeAppSettings = async () => {
  try {
    await invoke("initalize_app_settings");
  } catch (error) {
    console.error("Failed to initialize app settings:", error);
  }
};
</script>

<template>
  <div
    class="px-8 py-2 relative flex flex-col h-screen justify-center items-center w-screen layout"
  >
    <img src="../../assets/illustrations/share.svg" alt="" class="h-[50vh]" />
    <h1
      class="text-2xl mt-4 font-extrabold leading-loose text-center dark:text-white/70 w-[90%]"
      style="line-height: 20px; font-weight: 800"
    >
      Share and collaborate
    </h1>
    <p class="text-center text-150">
      Share audio books with friends, socialize and interract
    </p>

  <SensationalTint class="absolute inset-x-2 bottom-1 -z-10 transform-gpu overflow-hidden blur-3xl opacity-30 " />

    <RouterLink
      :to="{ name: 'SignUp' }"
      class="text-white font-medium bg-app-orange rounded-lg py-4 text-center hover:animate-pulse mt-10 w-full bottom-3.5"
      @click="initializeAppSettings"
    >
      Create account
    </RouterLink>
    <RouterLink
      :to="{ name: 'Login' }"
      class="text-app-dark border-app-dark border-2 dark:border-gray-400/50 dark:text-gray-400 font-medium bg-transparent rounded-lg py-4 text-center hover:animate-pulse mt-8 w-full"
      @click="initializeAppSettings"
    >
      Login
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

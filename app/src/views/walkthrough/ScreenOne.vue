<template>
  <div class="relative flex flex-col h-screen max-w-screen walkthrough_layout">
    <img src="../../assets/illustrations/upload.svg" alt="" class="h-[50vh]" />
    <h1
      class="text-2xl mt-2 font-extrabold leading-loose text-center dark:text-white/70 w-[90%]"
      style="line-height: 20px; font-weight: 800"
    >
      Upload document
    </h1>
    <p class="text-center text-150 text-gray-400">
      Upload a new PDF document to our high frequency servers
    </p>

    <SensationalTint
      class="absolute inset-x-2 bottom-1 -z-10 transform-gpu overflow-hidden blur-3xl opacity-30"
    />

    <div class="flex justify-between w-full mt-24 absolute px-8 bottom-7">
      <RouterLink
        :to="{ name: 'Login' }"
        class="text-gray-400 dark:text-gray-400 font-medium bg-transparent rounded-lg py-4 text-center hover:animate-pulse h-fit"
      >
        Skip
      </RouterLink>
      <RouterLink
        :to="{ name: 'ScreenTwo' }"
        class="text-white bg-app-orange h-fit py-2 px-6 rounded-lg"
      >
        Next
      </RouterLink>
    </div>
  </div>
</template>
<script lang="ts" setup>
import SensationalTint from "@/components/uiBlocks/SensationalTint.vue";
import { invoke } from "@tauri-apps/api/core";
import { onMounted, ref } from "vue";
import { RouterLink, useRouter } from "vue-router";
import { AppSettings } from "../../types/appSettings";

const settings = ref<AppSettings>();
const router = useRouter();

const fetchAppSettings = async () => {
  try {
    const result: AppSettings = await invoke("fetch_app_settings");
    settings.value = result;
  } catch (error) {
    console.error("Failed to fetch app settings:", error);
  }
};
onMounted(async () => {
  await fetchAppSettings();
  if (settings.value?.appInitialized) {
    router.push({ name: "ExistingUserLogin" });
  }
});
</script>

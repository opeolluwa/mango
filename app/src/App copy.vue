<template>
  <template v-if="fetchingResource">
    <div
      class="flex flex-col bg-red-500 w-screen h-screen relative justify-center items-center dark:bg-app-dark"
    >
      <div class="loader"></div>
      <p class="absolute bottom-6 label">Geckko labs</p>
    </div>
  </template>

  <template v-else>
    <div class="w-full">
      <UApp>
        <router-view v-slot="{ Component }">
          <transition name="fade">
            <component :is="Component" />
          </transition>
        </router-view>
      </UApp>
    </div>
  </template>
</template>

<script lang="ts" setup>
import { onMounted, ref } from "vue";
import { useRouter } from "vue-router";
import { AppSettings } from "../src-tauri/bindings/AppSettings";
import { invoke } from "@tauri-apps/api/core";

const fetchingResource = ref(true);
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
  fetchingResource.value = true
  try {
    await fetchAppSettings();
    if (settings.value?.appInitialized) {
      console.log("App is already initialized, redirecting to home...");
      router.push({ name: "ExistingUserLogin" });
    } else {
      console.log("App is not initialized, staying on ScreenThree...");
    }
  } catch (error) {
    console.log(error);
  } finally {
    fetchingResource.value = false;
  }
});
</script>

<style scoped>
/* HTML: <div class="loader"></div> */
.loader {
  --r1: 154%;
  --r2: 68.5%;
  width: 60px;
  aspect-ratio: 1;
  border-radius: 50%;
  background: radial-gradient(
      var(--r1) var(--r2) at top,
      #0000 79.5%,
      var(--color-app-orange) 80%
    ),
    radial-gradient(
      var(--r1) var(--r2) at bottom,
      var(--color-app-orange) 79.5%,
      #0000 80%
    ),
    radial-gradient(
      var(--r1) var(--r2) at top,
      #0000 79.5%,
      var(--color-app-orange) 80%
    ),
    #f5f2f2;
  background-size: 50.5% 220%;
  background-position: -100% 0%, 0% 0%, 100% 0%;
  background-repeat: no-repeat;
  animation: l9 2s infinite linear;
}

@keyframes l9 {
  33% {
    background-position: 0% 33%, 100% 33%, 200% 33%;
  }

  66% {
    background-position: -100% 66%, 0% 66%, 100% 66%;
  }

  100% {
    background-position: 0% 100%, 100% 100%, 200% 100%;
  }
}

@font-face {
  font-family: "Bebas Neue";
  src: url("./src/assets/BebasNeue-Regular.ttf");
}
.label {
  font-family: "Bebas Neue";
  text-transform: capitalize;
  font-size: 120%;
  font-weight: 800;
  color: #bbb;
  letter-spacing: 3.5px;
}
</style>

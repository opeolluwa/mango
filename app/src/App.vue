<template>
  <div class="w-full">
    <router-view v-slot="{ Component }">
      <transition name="fade">
        <component :is="Component" />
      </transition>
    </router-view>
  </div>
</template>

<script lang="ts" setup>
// import { onMounted } from "vue";
// import {
//   isPermissionGranted,
//   requestPermission,
//   sendNotification,
// } from "@tauri-apps/plugin-notification";

// onMounted(async () => {
//   let permissionGranted = await isPermissionGranted();

//   if (!permissionGranted) {
//     const permission = await requestPermission();
//     permissionGranted = permission === "granted";
//   }

//   if (permissionGranted) {
//     sendNotification({ title: "Tauri", body: "Tauri is awesome!" });
//   }
// });
</script>

<style scoped></style>

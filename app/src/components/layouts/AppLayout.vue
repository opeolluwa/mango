<template>
  <div class="h-screen w-screen">
    <slot />
    <AppDock />
    <AppDockPlayer />
  </div>
</template>

<script lang="ts" setup>
import AppDock from "../uiBlocks/AppDock.vue";
import AppDockPlayer from "../uiBlocks/AppDockPlayer.vue";
</script>

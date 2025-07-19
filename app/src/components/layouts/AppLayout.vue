<template>
  <div class="h-screen w-screen">
    <slot />
    <AppDock/>
  </div>
</template>

<script lang="ts" setup>
import AppDock from '../uiBlocks/AppDock.vue';

</script>

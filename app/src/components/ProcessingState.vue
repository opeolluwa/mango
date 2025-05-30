<template>
  <div
    class="modal absolute bg-black/60 flex flex-col text-accent-secondary items-center justify-center w-screen h-screen z-50"
  >
    <Loader />
    <small>Processing... </small>
  </div>
</template>


<script lang="ts" setup>
import Loader from "./AppLoader.vue";

const props = defineProps(['processingPdf'])
</script>
<style></style>
<template>
  <div class="bg-white/40 h-1 w-full rounded-full flex flex-col justify-center">
    <div
      class="bg-accent-secondary h-1 rounded-full cursor-pointer"
      :style="{ width: progress + '%' }"
    ></div>
  </div>
</template>

<script lang="ts" setup>
defineProps(["progress"]);
</script>

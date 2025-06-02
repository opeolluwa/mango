<template>
  <div class="bg-app-gray h-1   rounded-full flex flex-col justify-center">
    <div
      class="bg-app-orange h-1 rounded-full cursor-pointer"
      :style="{ width: progress + '%' }"
    ></div>
  </div>
</template>

<script lang="ts" setup>
defineProps(["progress"]);
</script>

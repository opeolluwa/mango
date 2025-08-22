<template>

  <Transition name="fade">
    <UApp class="w-full"> <RouterView /> </UApp>
  </Transition>

</template>

<script lang="ts" setup>
import { onMounted } from "vue";
import { RouterView } from "vue-router";

onMounted(() => {});
</script>

<style scoped></style>

<template>
  <UApp class="w-full">
    <RouterView v-slot="{ Component }">
      <Transition name="fade">
        <Component :is="Component" />
      </Transition>
    </RouterView>
  </UApp>
  <ModalsContainer />
</template>

<script lang="ts" setup>
import { onMounted } from "vue";
import { RouterView } from "vue-router";

onMounted(() => {
  const splash = document.getElementById("splashscreen");
  splash?.classList.add("animate__fadeOut");
  if (splash) {
    splash.style.display = "none";
  }
});
</script>

<style scoped></style>

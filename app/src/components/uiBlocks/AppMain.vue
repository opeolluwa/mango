<template>
  <main class="layout pt-3 parent-element">
    <ModalContainer
      class="flex flex-col items-center justify-center"
    ></ModalContainer>
    <Transition name="fade" mode="out-in">
      <RouterView />
    </Transition>
  </main>
</template>

<script lang="ts" setup>
import { container as ModalContainer } from "jenesius-vue-modal";
</script>

<style scoped>
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.3s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>

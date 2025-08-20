<template>
  <Transition name="fade">
    <UApp class="w-full"> <RouterView /> </UApp>
  </Transition>
  <Teleport to="#modal">
    <AppModal :title="modal.title" :kind="modal.kind" :message="modal.message" />
  </Teleport>
  <button @click="openModal">show modal</button>
</template>

<script lang="ts" setup>
import { RouterView } from "vue-router";
import AppModal from "./components/uiBlocks/AppModal.vue";
import { useModal } from "./composibles/useModal";
import { markRaw, onMounted } from "vue";

const modal = useModal({
  title: "Sign up failed",
  message: "invalid email or password",
  kind: "error",
});

const openModal = () => {
  modal.component.value = markRaw(AppModal);
  modal.showModal();
};

onMounted(()=>{
  openModal()
})
</script>

<style scoped></style>

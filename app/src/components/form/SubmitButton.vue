<script setup lang="ts">
import AppButton from "../button/AppButton.vue";
import FormLoader from "./FormLoader.vue";
defineProps({
  loading: Boolean,
});
</script>

<template>
  <div
    class="w-full h-full bg-gray-900/60 absolute left-0 bottom-0 flex justify-center items-center z-500"
    v-if="loading"
  >
    <Transition>
      <FormLoader />
    </Transition>
  </div>
  <AppButton
    type="submit"
    class="text-app-dark rounded-lg font-medium bg-app-orange/90 dark:text-white"
  >
    Continue
  </AppButton>
</template>

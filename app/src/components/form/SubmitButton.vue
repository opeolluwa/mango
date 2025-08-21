<template>
  <div
    v-if="loading"
    class="w-full h-[150vh] overflow-y-scroll bg-stone-900/60 absolute left-0 bottom-0 flex justify-center items-center z-500"
  >
    <Transition>
      <FormLoader />
    </Transition>
  </div>
  <AppButton
    type="submit"
    class="text-app-dark rounded-lg font-medium bg-app-orange/90 dark:text-white"
  >
    {{ text || "Continue" }}
  </AppButton>
</template>

<script setup lang="ts">
import AppButton from "../button/AppButton.vue";
import FormLoader from "./FormLoader.vue";
defineProps({
  loading: Boolean,
  text: {
    type: String,
    default: "Continue",
  },
});
</script>

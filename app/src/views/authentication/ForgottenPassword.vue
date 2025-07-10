<template>
  <div class="layout h-screen overflow-y-hidden">
    <ArrowLongLeftIcon
      class="size-12 text-app-orange mb-4"
      @click="router.back"
    />

    <h1 class="text-5xl w-[70%]">Forgotten Password!</h1>
    <p class="small text-gray-400">
      Let's get you back in. Provide email registered with your account
    </p>

    <form
      action=""
      @submit.prevent="submitForm"
      class="mt-8 flex flex-col gap-y-8"
    >
      <div class="flex flex-col w-full">
        <AppFormLabel text="Email" for="email" />
        <input
          id="email"
          class="hover:border hover:border-app-orange/60 bg-app-orange/10 selection:bg-app-orange app-form-input"
          type="email"
          placeholder="jane@mailer.com"
        />
      </div>

      <SubmitButton :loading="processingRequest" />
      <RouterLink to="/login" class="text-stone-500 flex justify-end -mt-4"
        >Return to login</RouterLink
      >

      <RouterLink
        to="/confirm-otp"
        class="text-stone-500 flex justify-end -mt-4"
        >confirm otp</RouterLink
      >
    </form>
  </div>
</template>

<script lang="ts" setup>
import { ArrowLongLeftIcon } from "@heroicons/vue/24/solid";
import { ref } from "vue";
import { useRouter } from "vue-router";
import AppFormLabel from "../../components/form/AppFormLabel.vue";
import SubmitButton from "../../components/form/SubmitButton.vue";

const router = useRouter();
const processingRequest = ref(false);
const submitForm = async () => {
  processingRequest.value = true;

  window.setTimeout(() => {
    processingRequest.value = false;
  }, 3000);
};
</script>

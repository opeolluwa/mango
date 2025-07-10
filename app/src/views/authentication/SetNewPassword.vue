<template>
  <div class="layout h-screen overflow-y-hidden">
    <ArrowLongLeftIcon
      class="size-12 text-app-orange mb-4"
      @click="router.back"
    />

    <h1 class="text-5xl w-[70%]">Set new Password</h1>
    <p class="small text-gray-400">
      Get crank up where you stopped your last audio book!
    </p>

    <form
      action=""
      @submit.prevent="submitForm"
      class="mt-8 flex flex-col gap-y-8"
    >
      <div class="flex flex-col w-full">
        <AppFormLabel text="New password" for="password" />
        <input
          id="password"
          class="hover:border hover:border-app-orange/60 bg-app-orange/10 selection:bg-app-orange app-form-input"
          type="text"
          placeholder="jane@mailer.com"
        />
      </div>

      <div class="flex flex-col w-full">
        <AppFormLabel text="Confirm password" for="password" />
        <input
          id="password"
          class="hover:border hover:border-app-orange/60 bg-app-orange/10 selection:bg-app-orange app-form-input"
          type="text"
          placeholder="********"
        />
      </div>
      <SubmitButton :loading="processingRequest" />
      <RouterLink
        to="/forgotten-password"
        class="text-stone-500 flex justify-end -mt-4"
        >Forgotten password?</RouterLink
      >

      <RouterLink to="/app" class="text-stone-500 flex justify-end -mt-4"
        >go to app</RouterLink
      >
    </form>
  </div>
</template>

<script lang="ts" setup>
import { ArrowLongLeftIcon } from "@heroicons/vue/24/solid";
import { ref } from "vue";
import { useRouter } from "vue-router";
import AppFormLabel from "../../components/form/AppFormLabel.vue";

const router = useRouter();
const processingRequest = ref(false);
const submitForm = async () => {
  processingRequest.value = true;
};
</script>

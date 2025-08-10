<template>
  <div class="layout h-screen overflow-y-hidden">
    <ArrowLongLeftIcon
      class="size-12 text-app-orange mb-4"
      @click="router.back"
    />

    <AuthScreenHeaderText>Login to continue!</AuthScreenHeaderText>
    <p class="small text-gray-400">
      Get crank up where you stopped your last audio book!
    </p>

    <form
      action=""
      class="mt-8 flex flex-col gap-y-8"
      @submit.prevent="submitForm"
    >
      <div class="flex flex-col w-full">
        <AppFormLabel text="Email" for="email" />
        <input
          id="email"
          class="app-form-input"
          type="text"
          placeholder="jane@mailer.com"
        />
      </div>

      <div class="flex flex-col w-full">
        <AppFormLabel text="password" for="email" />
        <input
          id="email"
          class="app-form-input"
          type="text"
          placeholder="********"
        />
      </div>
      <SubmitButton type="submit" :loading="processingRequest" />
      <RouterLink
        :to="{ name: 'ForgottenPassword' }"
        class="text-stone-500 flex justify-end -mt-4"
        >Forgotten password?</RouterLink
      >

      <RouterLink
        :to="{ name: 'Home' }"
        class="text-stone-500 flex justify-end -mt-4"
        >go to app</RouterLink
      >
    </form>
  </div>
</template>

<script lang="ts" setup>
import { ArrowLongLeftIcon } from "@heroicons/vue/24/solid";
import { onMounted, ref } from "vue";
import { useRouter } from "vue-router";
import { AppPersonalization } from "../../../src-tauri/bindings/AppPersonalization";
import AuthScreenHeaderText from "../../components/auth/AuthScreenHeaderText.vue";
import AppFormLabel from "../../components/form/AppFormLabel.vue";
import SubmitButton from "../../components/form/SubmitButton.vue";
import { invoke } from "@tauri-apps/api/core";

const appPersonalization = ref<AppPersonalization>();
const router = useRouter();

const processingRequest = ref(false);
const submitForm = async () => {
  processingRequest.value = true;
};

onMounted(async () => {
  appPersonalization.value = await invoke("fetch_app_personalization");
  console.log("appPersonalization", appPersonalization.value);
  
});
</script>

<template>
  <div class="layout h-screen overflow-y-hidden">
    <ArrowLongLeftIcon
      class="size-12 text-app-orange mb-4"
      @click="router.back"
    />

    <AuthScreenHeaderText>Welcome back, Olatunde!</AuthScreenHeaderText>
    <p class="small text-gray-400">Login with your password to continue</p>

    <form
      action=""
      @submit.prevent="submitForm"
      class="mt-8 flex flex-col gap-y-8"
    >
      <div class="flex flex-col w-full">
        <AppFormLabel text="Password" for="password" />
        <input
          id="password"
          class="app-form-input"
          type="password"
          placeholder="********"
        />
      </div>
      <SubmitButton type="submit" :loading="processingRequest" />
      <RouterLink :to="{ name: 'Login' }" class="text-stone-500 flex justify-end -mt-4">
        Not Olatunde?
      </RouterLink>
    </form>
  </div>
</template>

<script lang="ts" setup>
import AppFormLabel from "../../components/form/AppFormLabel.vue";
import { ArrowLongLeftIcon } from "@heroicons/vue/24/solid";
import { useRouter } from "vue-router";
import SubmitButton from "../../components/form/SubmitButton.vue";
import { ref } from "vue";
import AuthScreenHeaderText from "../../components/auth/AuthScreenHeaderText.vue";

const router = useRouter();

const processingRequest = ref(false);
const submitForm = async () => {
  processingRequest.value = true;
};
</script>

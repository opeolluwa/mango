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
      @submit.prevent="sendForm"
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

      <AppButton
        type="submit"
        class="bg-linear-210 rounded-lg font-medium text-[18px] from-app-orange/20 via-15% via-app-dark to-app-orange w-full"
      >
        Continue
      </AppButton>
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
import AppButton from "../../components/button/AppButton.vue";
import AppFormLabel from "../../components/form/AppFormLabel.vue";
import { ArrowLongLeftIcon } from "@heroicons/vue/24/solid";
import { useRouter } from "vue-router";

const router = useRouter();
const sendForm = async () => {};
</script>



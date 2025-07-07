<template>
  <div class="layout h-screen overflow-y-hidden">
    <ArrowLongLeftIcon
      class="size-12 text-app-orange mb-4"
      @click="router.back"
    />

    <h1 class="text-5xl">Login to continue!</h1>
    <p class="small text-gray-400">
      Get crank up where you stopped your last audio book!
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
          type="text"
        />
      </div>

      <div class="flex flex-col w-full">
        <AppFormLabel text="password" for="email" />
        <input
          id="email"
          class="hover:border hover:border-app-orange/60 bg-app-orange/10 selection:bg-app-orange app-form-input"
          type="text"
        />
      </div>
      <AppButton
        type="submit"
        class="bg-linear-210 rounded-lg font-medium text-[18px] from-app-orange/20 via-15% via-app-dark to bg-app-orange w-full"
      >
        Continue
      </AppButton>
      <RouterLink
        to="/forgotten-password"
        class="text-stone-500 flex justify-end"
        >Forgotten password?</RouterLink
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

<style scoped>
*::-webkit-scrollbar-thumb {
  background-color: transparent;
  opacity: 0.75;
  border-radius: 6px;
  border: 3px solid transparent;
}
</style>

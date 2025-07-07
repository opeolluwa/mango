<template>
  <div class="layout pt-6">
    <h1 class="text-5xl">Create a free account</h1>
    <p class="small text-gray-400">Register a free account</p>

    <form
      action=""
      @submit.prevent="sendForm"
      class="mt-4 flex flex-col gap-y-8"
    >
      <div></div>

      <div class="flex flex-col w-full">
        <Label class="font-semibold leading-[35px]" for="firstName">
          First name
        </Label>

        <input
          id="firstName"
          class="hover:border hover:border-app-orange/60 hover:bg-app-orange/10 selection:bg-app-orange app-form-input "
          type="text"
          placeholder="enter your first name"
        />
      </div>

      <div class="flex flex-col w-full">
        <Label class="font-semibold leading-[35px]" for="firstName">
          Last name
        </Label>

        <input
          id="firstName"
          class="border-app-orange/10 border hover:border hover:border-app-orange/60 hover:bg-app-orange/10 selection:bg-app-orange app-form-input "
          type="text"
          placeholder="enter your first name"
        />
      </div>

      <div class="flex flex-col w-full">
        <Label class="font-semibold leading-[35px]" for="firstName">
          Email
        </Label>

        <input
          id="email"
          class="border-app-orange/10 border hover:border hover:border-app-orange/60 hover:bg-app-orange/10 selection:bg-app-orange app-form-input "
          type="text"
          placeholder="enter your email"
        />
      </div>

      <div class="flex flex-col w-full">
        <Label class="font-semibold leading-[35px]" for="password">
          Password
        </Label>

        <input
          id="password"
          class="border-app-orange/10 border hover:border hover:border-app-orange/60 hover:bg-app-orange/10 selection:bg-app-orange app-form-input "
          type="text"
          placeholder="provide a string password"
        />
      </div>
      <AppButton
        type="submit"
        class="bg-linear-210 rounded-lg font-medium text-[18px] from-app-orange/20 via-15% via-app-dark to bg-app-orange w-full"
      >
        Continue
      </AppButton>
    </form>
  </div>
</template>

<script lang="ts" setup>
import AppButton from "../../components/button/appButton.vue";
import { Label } from "reka-ui";
const sendForm = async () => {};
</script>

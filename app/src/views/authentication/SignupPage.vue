<template>
  <div class="layout h-screen overflow-y-hidden">
    <ArrowLongLeftIcon
      class="size-12 text-app-orange mb-4"
      @click="router.back"
    />

    <h1 class="text-5xl">Create a free account</h1>
    <p class="small text-gray-400">Register a free account to get started!</p>

    <form
      action=""
      @submit.prevent="sendForm"
      class="mt-8 flex flex-col gap-y-8"
    >
      <!-- <div class="flex flex-col w-full">
        <AppFormLabel text="First name" for="firstName" />
        <input
          id="firstName"
          class="hover:border hover:border-app-orange/60 bg-app-orange/10 selection:bg-app-orange app-form-input"
          type="text"
          placeholder="jane"
        />
      </div> 

      <div class="flex flex-col w-full">
        <AppFormLabel text="Last name" for="" />
        <input
          id="firstName"
          class="hover:border hover:border-app-orange/60 bg-app-orange/10 selection:bg-app-orange app-form-input"
          type="text"
          placeholder="doe"
        />
      </div> -->

      <div class="flex flex-col w-full">
        <AppFormLabel text="Email" for="email" />
        <input
          id="email"
          class="hover:border hover:border-app-orange/60 bg-app-orange/10 selection:bg-app-orange app-form-input"
          type="email"
          placeholder="jane@mailer.com"
        />
      </div>

      <div class="flex flex-col w-full">
        <AppFormLabel for="password" text="Password" />

        <input
          id="password"
          class="hover:border hover:border-app-orange/60 bg-app-orange/10 selection:bg-app-orange app-form-input"
          type="password"
          placeholder="********"
        />
      </div>
      <div class="flex flex-col gap-2.5">
        <label
          class="flex flex-row gap-4 items-center [&>.checkbox]:hover:bg-neutral-100"
        >
          <CheckboxRoot
            v-model="checkboxOne"
            class="hover:bg-app-orange flex h-5 w-5 appearance-none items-center justify-center rounded-md bg-app-orange shadow-sm border-none outline-none "
          >
            <CheckboxIndicator
              class="bg-app-orange h-full w-full rounded flex items-center justify-center"
            >
              <Icon icon="radix-icons:check" class="h-5 w-5 text-app-white" />
            </CheckboxIndicator>
          </CheckboxRoot>
          <span class="select-none text-gray-400 text-sm"
            >Accept terms and conditions.</span
          >
        </label>
      </div>
      <AppButton
        type="submit"
        class="bg-linear-210 rounded-lg font-medium text-[18px] from-app-orange/20 via-15% via-app-dark to bg-app-orange w-full"
      >
        Continue
      </AppButton>
      <RouterLink to="/login" class="text-stone-500 flex justify-end -mt-4"
        >Already have an account? Login</RouterLink
      >
    </form>
  </div>
</template>

<script lang="ts" setup>
import AppButton from "../../components/button/AppButton.vue";
import AppFormLabel from "../../components/form/AppFormLabel.vue";
import { ArrowLongLeftIcon } from "@heroicons/vue/24/solid";
import { useRouter } from "vue-router";
import { CheckboxIndicator, CheckboxRoot } from "reka-ui";
import { ref } from "vue";
const router = useRouter();
const checkboxOne = ref(true);
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

<template>
  <div class="layout">
    <ArrowLongLeftIcon
      class="size-12 text-app-orange mb-4"
      @click="useGoBack"
    />

    <AuthScreenHeaderText>Create a free account</AuthScreenHeaderText>
    <p class="small text-gray-400">Register a free account to get started!</p>

    <form
      action=""
      class="mt-8 flex flex-col gap-y-8"
      @submit.prevent="submitForm"
    >
      <ErrorOutlet v-if="formSubmitError">
        {{ formSubmitError }}
      </ErrorOutlet>
      <div class="flex flex-col w-full">
        <AppFormLabel text="Email" for="email" />
        <input
          id="email"
          v-model="email"
          class="app-form-input"
          type="email"
          placeholder="jane@mailer.com"
          v-bind="emailProps"
        />
        <ErrorOutlet v-if="errors.email" class="mt-2">
          {{ errors.email }}
        </ErrorOutlet>
      </div>

      <div class="flex flex-col w-full">
        <AppFormLabel for="password" text="Password" />

        <input
          id="password"
          v-model="password"
          class="app-form-input"
          type="password"
          placeholder="********"
          v-bind="passwordProps"
        />
        <ErrorOutlet v-if="errors.password" class="mt-2">
          {{ errors.password }}
        </ErrorOutlet>
      </div>
      <div class="flex flex-col gap-2.5">
        <label
          class="flex flex-row gap-4 items-center [&>.checkbox]:hover:bg-neutral-100"
        >
          <UCheckbox v-model="acceptTerms" color="neutral" size="md" />
          <span class="select-none text-stone-700 text-sm dark:text-white"
            >Accept terms and conditions.</span
          >
        </label>
      </div>
      <SubmitButton :loading="processingRequest" class="text-white" />
      <RouterLink :to="{ name: 'Login' }" class="text-stone-500 text-center"
        >Already have an account?
        <span class="text-app-orange font-medium">Login</span></RouterLink
      >
    </form>
  </div>
</template>

<script lang="ts" setup>
import { ArrowLongLeftIcon } from "@heroicons/vue/24/solid";
import { message } from "@tauri-apps/plugin-dialog";
import { useForm } from "vee-validate";
import { ref } from "vue";
import { useRouter } from "vue-router";
import * as yup from "yup";
import axios from "../../axios.config.ts";
import AuthScreenHeaderText from "../../components/auth/AuthScreenHeaderText.vue";
import AppFormLabel from "../../components/form/AppFormLabel.vue";
import ErrorOutlet from "../../components/form/ErrorOutlet.vue";
import SubmitButton from "../../components/form/SubmitButton.vue";
import { useGoBack } from "../../composibles/router.ts";

const validationSchema = yup.object({
  email: yup.string().required().email(),
  password: yup.string().required().min(6),
});

const { defineField, errors, handleSubmit } = useForm({
  validationSchema,
});

const [email, emailProps] = defineField("email");
const [password, passwordProps] = defineField("password");

const acceptTerms = ref(true);
const processingRequest = ref(false);
const formSubmitError = ref("");
const router = useRouter();

const submitForm = handleSubmit(async (values) => {
  processingRequest.value = true;

  try {
    if (!acceptTerms.value) {
      await message("Please accept the terms of service to continue", {
        title: "Action required!",
        kind: "info",
      });
      processingRequest.value = false;
      return;
    }
    const response = await axios.post("/auth/signup", {
      email: values.email,
      password: values.password,
    });

    if (response.status === 201) {
      const token = response.data.data.token;

      router.push({ name: "ConfirmOtp", query: { token } });
    } else {
      formSubmitError.value = response.data.message || "Failed to create user";
    }
  } catch (error) {
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    // await message((error as any).response.data.message, {
    //   title: "Failed to create user!",
    //   kind: "error",
    // });
    console.log(error);
  } finally {
    processingRequest.value = false;
  }
});
</script>

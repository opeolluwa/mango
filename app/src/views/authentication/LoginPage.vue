<template>
  <div class="layout h-screen overflow-y-hidden relative">
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
      <ErrorOutlet v-if="formSubmitError">{{ formSubmitError }}</ErrorOutlet>
      <div class="flex flex-col w-full">
        <AppFormLabel text="Email" for="email" />
        <input
          id="email"
          v-model="email"
          class="app-form-input"
          type="text"
          placeholder="jane@mailer.com"
          v-bind="emailAttr"
        />
        <ErrorOutlet v-if="errors.email">{{ errors.email }}</ErrorOutlet>
      </div>

      <div class="flex flex-col w-full">
        <AppFormLabel text="password" for="password" />
        <input
          id="password"
          v-model="password"
          class="app-form-input"
          type="password"
          placeholder="********"
          v-bind="passwordAttr"
        />
        <ErrorOutlet v-if="errors.password">{{ errors.password }}</ErrorOutlet>
      </div>
      <SubmitButton type="submit" :loading="processingRequest" />
      <RouterLink
        :to="{ name: 'ForgottenPassword' }"
        class="text-stone-500 flex justify-end -mt-4"
        >Forgotten password?</RouterLink
      >
    </form>
    <div class="w-full text-center px-8 mt-28">
      <RouterLink
        :to="{ name: 'SignUp' }"
        class="text-stone-500 mt-20 text-center"
        >Don&apos;t have an account
        <span class="accent font-medium">Sign up</span>
      </RouterLink>
    </div>
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
import { useForm } from "vee-validate";
import * as yup from "yup";
import ErrorOutlet from "../../components/form/ErrorOutlet.vue";
import axios from "axios";

const loginSchema = yup.object({
  email: yup.string().required().email(),
  password: yup.string().required(),
});

const { defineField, errors, handleSubmit } = useForm({
  validationSchema: loginSchema,
});

const router = useRouter();

const [email, emailAttr] = defineField("email");
const [password, passwordAttr] = defineField("password");

const appPersonalization = ref<AppPersonalization>();
const processingRequest = ref(false);
const formSubmitError = ref<string | null>(null);

const submitForm = handleSubmit(async (values) => {
  try {
    const { email, password } = values;
    processingRequest.value = true;
    const response = await axios.post(
      "/auth/login",
      { email, password },
      {
        headers: {
          "Content-Type": "application/json",
        },
      }
    );

    console.log(response);
    if (response.status === 200) {
      router.replace({ name: "Home" });
    } else {
      throw new Error("Login failed");
    }
  } catch (error) {
    console.log(error);
    if (error) {
      formSubmitError.value = "Invalid credentials";
    }
  } finally {
    processingRequest.value = false;
  }
});

onMounted(async () => {
  appPersonalization.value = await invoke("fetch_app_personalization");
  console.log("appPersonalization", appPersonalization.value);
});
</script>

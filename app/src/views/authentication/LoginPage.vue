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
      <div class="flex flex-col w-full">
        <AppFormLabel text="Email" for="email" />
        <input
          id="email"
          v-model="email"
          class="app-form-input"
          type="text"
          placeholder="jane@mailer.com"
          v-bind="emailProps"
        />
        <ErrorOutlet v-if="errors.email" class="mt-2">
          {{ errors.email }}
        </ErrorOutlet>
      </div>

      <div class="flex flex-col w-full">
        <AppFormLabel text="password" for="email" />
        <input
          id="password"
          v-model="password"
          class="app-form-input"
          type="text"
          placeholder="********"
          v-bind="passwordProps"
        />
        <ErrorOutlet v-if="errors.password" class="mt-2">
          {{ errors.password }}
        </ErrorOutlet>
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
        >Don&apos;t have an account <span class="accent font-medium">Sign up</span>
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
// import { load } from "@tauri-apps/plugin-store";
import axios from "../../axios.config";
import ErrorOutlet from "../../components/form/ErrorOutlet.vue";
import * as yup from "yup";
import { useForm } from "vee-validate";

// const store = await load("token.json", { autoSave: true });
const appPersonalization = ref<AppPersonalization>();
const router = useRouter();
const formSubmitError = ref("");
const processingRequest = ref(false);

const validationSchema = yup.object({
  email: yup.string().required().email(),
  password: yup.string().required().min(6),
});

const { defineField, errors, handleSubmit } = useForm({
  validationSchema,
});

const [email, emailProps] = defineField("email");
const [password, passwordProps] = defineField("password");

const submitForm = handleSubmit(async (values) => {
  processingRequest.value = true;

  console.log("Form submitted with values:", values);
  try {
    const response = await axios.post("/auth/login", {
      email: values.email,
      password: values.password,
    });
    if (response.status === 200) {
      router.push({ name: "Home" });
      // const { data } = response.data;
      // await store.set("token", {
      //   ...data,
      // });
    } else {
      console.error("Failed to login user due to", response.data);
      formSubmitError.value = response.data.message || "Failed to create user";
    }
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
  } catch (error: any) {
    console.log(error.response.data);
    formSubmitError.value = error.response.data.message;
  } finally {
    processingRequest.value = false;
  }
});

onMounted(async () => {
  appPersonalization.value = await invoke("fetch_app_personalization");
  console.log("appPersonalization", appPersonalization.value);
});
</script>

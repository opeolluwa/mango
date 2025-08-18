<template>
  <div class="layout h-screen overflow-y-hidden relative">
    <ArrowLongLeftIcon
      class="size-12 text-app-orange mb-4"
      @click="router.back"
    />
    <AuthScreenHeaderText>Login to continue!</AuthScreenHeaderText>
    <p class="small text-gray-400">
      Let&apos; cranked up where you stopped your last audio book!
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
    </form>
    <RouterLink
      :to="{ name: 'ForgottenPassword' }"
      class="text-stone-500 flex justify-end mt-2"
      >Forgotten password?</RouterLink
    >
    <div class="w-full text-center px-8 mt-8">
      <RouterLink
        :to="{ name: 'Home' }"
        class="text-stone-500 mt-20 text-center"
        >Don&apos;t have an account
        <span class="accent font-medium">Sign up</span>
      </RouterLink>
    </div>
  </div>
</template>
<script lang="ts" setup>
import axios from "axios";
import { useForm } from "vee-validate";
import { ref } from "vue";
import { useRouter } from "vue-router";
import * as yup from "yup";

import { ArrowLongLeftIcon } from "@heroicons/vue/24/solid";

import AuthScreenHeaderText from "../../components/auth/AuthScreenHeaderText.vue";
import AppFormLabel from "../../components/form/AppFormLabel.vue";
import ErrorOutlet from "../../components/form/ErrorOutlet.vue";
import SubmitButton from "../../components/form/SubmitButton.vue";

import { useCachedUserStore } from "../../stores/cachedUser";
import { useUserInformation } from "../../stores/user";

const loginSchema = yup.object({
  email: yup.string().required().email(),
  password: yup.string().required(),
});

const { defineField, errors, handleSubmit } = useForm({
  validationSchema: loginSchema,
});

const [email, emailAttr] = defineField("email");
const [password, passwordAttr] = defineField("password");

const router = useRouter();

const cachedUserStore = useCachedUserStore();
const userStore = useUserInformation();

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

    if (response.status !== 200) {
      formSubmitError.value = "Failed to login. Please try again";
      return;
    }
    const token = response.data.data.accessToken;
    const userInformation = await userStore.fetchUserInformation(token);

    await cachedUserStore.cacheUserData({
      email: userInformation.email,
      firstName: userInformation.firstName,
      lastName: userInformation.lastName,
      avatarUrl: userInformation.profilePicture,
    });
    router.push({ name: "Home" });
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
  } catch (error: any) {
    formSubmitError.value = error.response.data.message;
  } finally {
    processingRequest.value = false;
  }
});

// onBeforeMount(() => {
//   if (userExists.value) {
//     router.push({ name: "ExistingUserLogin" });
//   }
// });
</script>

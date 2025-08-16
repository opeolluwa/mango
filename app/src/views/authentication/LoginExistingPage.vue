<template>
  <div class="layout h-screen overflow-y-hidden">
    <ArrowLongLeftIcon
      class="size-12 text-app-orange mb-4"
      @click="router.back"
    />

    <AuthScreenHeaderText
      >Welcome back, {{ cachedUser?.firstName }}!</AuthScreenHeaderText
    >
    <p class="small text-gray-400">Login with your password to continue</p>
    <ErrorOutlet v-if="formSubmitError">{{ formSubmitError }}</ErrorOutlet>
    <form
      action=""
      class="mt-8 flex flex-col gap-y-8"
      @submit.prevent="submitForm"
    >
      <div class="flex flex-col w-full">
        <AppFormLabel text="Password" for="password" />
        <input
          id="password"
          v-model="password"
          class="app-form-input"
          type="password"
          placeholder="********"
          v-bind="passwordProps"
        />
        <ErrorOutlet v-if="errors.password"> {{ errors.password }}</ErrorOutlet>
      </div>
      <SubmitButton type="submit" :loading="processingRequest" />
      <RouterLink
        :to="{ name: 'Login' }"
        class="text-stone-500 flex justify-end -mt-4"
      >
        Not {{ cachedUser?.firstName }}?
      </RouterLink>
    </form>
  </div>
</template>

<script lang="ts" setup>
import AppFormLabel from "../../components/form/AppFormLabel.vue";
import { ArrowLongLeftIcon } from "@heroicons/vue/24/solid";
import { useRouter } from "vue-router";
import SubmitButton from "../../components/form/SubmitButton.vue";
import { onBeforeMount, ref } from "vue";
import AuthScreenHeaderText from "../../components/auth/AuthScreenHeaderText.vue";
import { useCachedUserStore } from "../../stores/cachedUser";
import { CachedUser } from "../../../src-tauri/bindings/CachedUser";
import { useUserInformation } from "../../stores/user";
import axios from "axios";
import * as yup from "yup";
import { useForm } from "vee-validate";
import ErrorOutlet from "../../components/form/ErrorOutlet.vue";

const router = useRouter();

const loginSchema = yup.object({
  password: yup.string().required(),
});

const { defineField, errors, handleSubmit } = useForm({
  validationSchema: loginSchema,
});

const cachedUserStore = useCachedUserStore();
const userStore = useUserInformation();

const cachedUser = ref<CachedUser>();

const processingRequest = ref(false);
const [password, passwordProps] = defineField("password");
const formSubmitError = ref<string | null>(null);

const submitForm = handleSubmit(async (values) => {
  try {
    const { password } = values;
    processingRequest.value = true;
    console.log({ email: cachedUser.value?.email, password });
    const response = await axios.post(
      "/auth/login",
      { email: cachedUser.value?.email, password },
      {
        headers: {
          "Content-Type": "application/json",
        },
      }
    );

    if (response.status !== 200) {
      formSubmitError.value = "Failed to login. Please try again";
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

onBeforeMount(async () => {
  const user = await cachedUserStore.fetchCachedUser();
  if (!user) {
    router.push({ name: "Login" });
  } else {
    cachedUser.value = user;
  }
});
</script>

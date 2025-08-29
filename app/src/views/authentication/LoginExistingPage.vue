<template>
  <div class="layout h-screen overflow-y-hidden">
    <ArrowLongLeftIcon
      class="size-12 text-app-orange mb-4"
      @click="router.back"
    />

    <AuthScreenHeaderText
      >Welcome back, {{ cachedUser?.firstName?.trim() }}!</AuthScreenHeaderText
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
import { ArrowLongLeftIcon } from "@heroicons/vue/24/solid";
import { useForm } from "vee-validate";
import { onBeforeMount, ref } from "vue";
import { useRouter } from "vue-router";
import * as yup from "yup";
import AuthScreenHeaderText from "../../components/auth/AuthScreenHeaderText.vue";
import AppFormLabel from "../../components/form/AppFormLabel.vue";
import ErrorOutlet from "../../components/form/ErrorOutlet.vue";
import SubmitButton from "../../components/form/SubmitButton.vue";
import { useLogin } from "../../composibles/useLogin";
import { useCachedUserStore } from "../../stores/cachedUser";
import { CachedUser } from "../../types/cachedUser";

const router = useRouter();

const loginSchema = yup.object({
  password: yup.string().required(),
});

const { defineField, errors, handleSubmit } = useForm({
  validationSchema: loginSchema,
});

const cachedUserStore = useCachedUserStore();
const cachedUser = ref<CachedUser>();

const processingRequest = ref(false);
const [password, passwordProps] = defineField("password");
const formSubmitError = ref<string | null>(null);
const submitForm = handleSubmit(async (values) => {
  formSubmitError.value = null;
  processingRequest.value = true;

  await useLogin({
    email: String(cachedUser.value?.email),
    password: values.password,
  })
    .then(async () => {
      await router.push({ name: "Home" });
      processingRequest.value = false;
    })
    .finally(() => {
      processingRequest.value = false;
    });
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

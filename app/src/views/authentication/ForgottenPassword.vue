<template>
  <div class="layout h-screen overflow-y-hidden">
    <ArrowLongLeftIcon
      class="size-12 text-app-orange mb-4"
      @click="router.back"
    />

    <AuthScreenHeaderText>Forgotten Password!</AuthScreenHeaderText>
    <p class="small text-gray-400">
      Let's get you back in. Provide email registered with your account
    </p>

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
          v-bind="emailProps"
          placeholder="jane@mailer.com"
        />
        <ErrorOutlet v-if="errors.email" class="mt-2">
          {{ errors.email }}
        </ErrorOutlet>
      </div>

      <SubmitButton :loading="processingRequest" />
      <RouterLink
        :to="{ name: 'Login' }"
        class="text-stone-500 flex justify-end -mt-4"
      >
        Return to login
      </RouterLink>
    </form>
  </div>
</template>

<script lang="ts" setup>
import { ArrowLongLeftIcon } from "@heroicons/vue/24/solid";
import { useForm } from "vee-validate";
import { ref } from "vue";
import { useRouter } from "vue-router";
import * as yup from "yup";
import axios from "../../plugins/axios";
import AuthScreenHeaderText from "../../components/auth/AuthScreenHeaderText.vue";
import AppFormLabel from "../../components/form/AppFormLabel.vue";
import ErrorOutlet from "../../components/form/ErrorOutlet.vue";
import SubmitButton from "../../components/form/SubmitButton.vue";

const validationSchema = yup.object({
  email: yup.string().required().email(),
});

const { defineField, errors, handleSubmit } = useForm({
  validationSchema,
});

const router = useRouter();

const [email, emailProps] = defineField("email");
const formSubmitError = ref("");
const processingRequest = ref(false);

const submitForm = handleSubmit(async (values) => {
  processingRequest.value = true;

  try {
    const response = await axios.post("/auth/forgotten-password", {
      email: values.email,
      password: values.password,
    });

    if (response.status === 200) {
      const token = response.data.data.token;
      router.push({ name: "VerifyAccountRecovery", query: { token } });
    } else {
      formSubmitError.value = response.data.message || "Failed to create user";
    }
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
  } catch (error: any) {
    console.log(error);
    // formSubmitError.value = error.response.data.message;
  } finally {
    processingRequest.value = false;
  }
});
</script>

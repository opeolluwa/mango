<template>
  <div class="layout h-screen overflow-y-hidden">
    <ArrowLongLeftIcon
      class="size-12 text-app-orange mb-4"
      @click="router.back"
    />


    <AuthScreenHeaderText>Set new Password</AuthScreenHeaderText>
    <p class="small text-gray-400">There you go! Set new password</p>

    <form action="" class="mt-8 flex flex-col gap-y-8" @submit="submitForm">
      <ErrorOutlet v-if="formSubmitError">{{ formSubmitError }}</ErrorOutlet>

      <div class="flex flex-col w-full">
        <AppFormLabel text="New password" for="password" />
        <input
          id="password"
          v-model="password"
          class="app-form-input"
          type="password"
          placeholder="********"
          v-bind="passwordProps"
        />
        <ErrorOutlet v-if="errors.password">{{ errors.password }}</ErrorOutlet>
      </div>

      <div class="flex flex-col w-full">
        <AppFormLabel text="Confirm password" for="password" />
        <input
          id="password"
          v-model="confirmPassword"
          class="app-form-input"
          type="password"
          placeholder="********"
          v-bind="confirmPasswordProps"
        />
        <ErrorOutlet v-if="errors.confirmPassword">{{
          errors.confirmpassword
        }}</ErrorOutlet>
      </div>
      <SubmitButton :loading="processingRequest" class="text-white" />
    </form>
  </div>
</template>

<script lang="ts" setup>
import { ArrowLongLeftIcon } from "@heroicons/vue/24/solid";
import { ref } from "vue";
import { useRoute, useRouter } from "vue-router";
import AppFormLabel from "../../components/form/AppFormLabel.vue";
import AuthScreenHeaderText from "../../components/auth/AuthScreenHeaderText.vue";
import * as yup from "yup";
import { useForm } from "vee-validate";
import SubmitButton from "../../components/form/SubmitButton.vue";
import ErrorOutlet from "../../components/form/ErrorOutlet.vue";
import axios from "axios";

const validationSchema = yup.object({
  confirmPassword: yup.string().required(),
  password: yup.string().required(),
});

const { defineField, errors, handleSubmit } = useForm({
  validationSchema: validationSchema,
});

const [password, passwordProps] = defineField("password");
const [confirmPassword, confirmPasswordProps] = defineField("confirmPassword");

const router = useRouter();
const processingRequest = ref(false);
const formSubmitError = ref<string | null>(null);
const route = useRoute();

const submitForm = handleSubmit(async (values) => {
  processingRequest.value = true;
  try {
    const { password, confirmPassword } = values;
    processingRequest.value = true;
    const response = await axios.post(
      "/auth/reset-password",
      { confirmPassword, password },
      {
        headers: {
          "Content-Type": "application/json",
          Authorization: `Bearer ${route.query.token}`,
        },
      }
    );

    console.log(response);
    if (response.status === 200) {
      router.replace({ name: "Login" });
    } else {
      throw new Error("Operation failed");
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
</script>

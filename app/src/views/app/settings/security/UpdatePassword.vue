<template>
  <h2 class="font-3xl text-gray-400 font-medium">Update password</h2>
  <form action="" class="mt-8 flex flex-col gap-y-8" @submit="submitForm">
    <ErrorOutlet v-if="formSubmitError">{{ formSubmitError }}</ErrorOutlet>

    <div class="flex flex-col w-full">
      <AppFormLabel text="Current Password" for="password" />
      <input
        id="password"
        v-model="currentPassword"
        class="app-form-input"
        type="password"
        placeholder="********"
        v-bind="currentPasswordProps"
      />
      <ErrorOutlet v-if="errors.currentPassword">{{
        errors.currentPassword
      }}</ErrorOutlet>
    </div>

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

    <SubmitButton
      :loading="formSubmitted"
      :diabaled="dataUnchanged"
      class="text-white"
      text="Update profile"
    />
  </form>
</template>

<script lang="ts" setup>
import axios from "axios";
import { useForm } from "vee-validate";
import { computed, ref } from "vue";
import * as yup from "yup";
import AppFormLabel from "../../../../components/form/AppFormLabel.vue";
import ErrorOutlet from "../../../../components/form/ErrorOutlet.vue";
import SubmitButton from "../../../../components/form/SubmitButton.vue";
import { useUpdatePassword } from "../../../../composibles/useUpdatePassword";

const validationSchema = yup.object({
  confirmPassword: yup.string().required(),
  password: yup.string().required(),
});

const { defineField, errors, handleSubmit } = useForm({
  validationSchema: validationSchema,
});

const [password, passwordProps] = defineField("password");
const [confirmPassword, confirmPasswordProps] = defineField("confirmPassword");
const [currentPassword, currentPasswordProps] = defineField("currentPassword");

const formSubmitError = ref<string | null>(null);

const formSubmitted = ref(false);
const dataUnchanged = computed(() => {
  return (
    password.value === "" &&
    confirmPassword.value === "" &&
    currentPassword.value === ""
  );
});
const submitForm = handleSubmit(async () => {
  formSubmitError.value = null;
  try {
    const response = await useUpdatePassword({
      currentPassword: currentPassword.value,
      newPassword: password.value,
      confirmPassword: confirmPassword.value,
    });

    if (!response.success) {
      formSubmitError.value =
        response.data.message || "An error occurred. Please try again.";
      return;
    }

    // eslint-disable-next-line @typescript-eslint/no-explicit-any
  } catch (error: any) {
    if (axios.isAxiosError(error)) {
      formSubmitError.value =
        error.response?.data?.message || "An error occurred. Please try again.";
    } else {
      formSubmitError.value = "An error occurred. Please try again.";
    }
  } finally {
    formSubmitted.value = false;
  }
});
</script>

<template>
  <div class="layout">
    <AuthScreenHeaderText>Complete your profile</AuthScreenHeaderText>
    <p class="small text-gray-400">Just a few details to get you started</p>

    <form class="mt-8 flex flex-col gap-y-8" @submit="onSubmit">
      <ErrorOutlet v-if="formSubmitError">{{ formSubmitError }}</ErrorOutlet>

      <div class="flex flex-col w-full">
        <AppFormLabel for="firstname" text="Firstname" />
        <input
          id="firstname"
          v-model="firstname"
          class="app-form-input"
          type="text"
          placeholder="firstname"
          v-bind="firstnameAttr"
        />
        <ErrorOutlet v-if="errors.firstname">{{
          errors.firstname
        }}</ErrorOutlet>
      </div>

      <div class="flex flex-col w-full">
        <AppFormLabel for="lastname" text="lastname" />
        <input
          id="lastname"
          v-model="lastname"
          class="app-form-input"
          type="text"
          placeholder="lastname"
          v-bind="lastnameAttr"
        />
        <ErrorOutlet v-if="errors.firstname">{{ errors.lastname }}</ErrorOutlet>
      </div>

      <SubmitButton :loading="processingRequest" />
    </form>
  </div>
</template>

<script setup lang="ts">
import { useForm } from "vee-validate";
import AuthScreenHeaderText from "../../components/auth/AuthScreenHeaderText.vue";
import AppFormLabel from "../../components/form/AppFormLabel.vue";
import SubmitButton from "../../components/form/SubmitButton.vue";

import * as yup from "yup";
import ErrorOutlet from "../../components/form/ErrorOutlet.vue";

import axios from "axios";
import { useRoute, useRouter } from "vue-router";
import { ref } from "vue";

const formSchema = yup.object({
  firstname: yup.string().required(),
  lastname: yup.string().required(),
});

const { defineField, errors, handleSubmit } = useForm({
  validationSchema: formSchema,
});

const route = useRoute();
const router = useRouter();

const [firstname, firstnameAttr] = defineField("firstname");
const [lastname, lastnameAttr] = defineField("lastname");

const processingRequest = ref(false);
const formSubmitError = ref("");

const onSubmit = handleSubmit(async (values) => {
  processingRequest.value = true;
  try {
    const { firstname: firstName, lastname: lastName } = values;

    const response = await axios.post(
      "/auth/onboard",
      { firstName, lastName },
      {
        headers: {
          "Content-Type": "application/json",
          Authorization: `Bearer ${route.query.token}`,
        },
      }
    );

    if (response.status === 200) {
      router.replace({ name: "Home" });
    } else {
      formSubmitError.value = response.data.error || "Failed";
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

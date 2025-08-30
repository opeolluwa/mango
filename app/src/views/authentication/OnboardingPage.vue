<template>
  <div class="layout">
    <AuthScreenHeaderText>Complete your profile</AuthScreenHeaderText>
    <p class="small text-gray-400">Just a few details to get you started</p>

    <form class="mt-8 flex flex-col gap-y-8" @submit="onSubmit">
      <ErrorOutlet v-if="formSubmitError">{{ formSubmitError }}</ErrorOutlet>

      <div class="flex flex-col w-full">
        <AppFormLabel for="firstname" text="First name" />
        <input
          id="firstname"
          v-model="firstname"
          class="app-form-input"
          type="text"
          placeholder="firstname"
          v-bind="firstnameProps"
        />
        <ErrorOutlet v-if="errors.firstname">{{
          errors.firstname
        }}</ErrorOutlet>
      </div>

      <div class="flex flex-col w-full">
        <AppFormLabel for="lastname" text="Last name" />
        <input
          id="lastname"
          v-model="lastname"
          class="app-form-input"
          type="text"
          placeholder="lastname"
          v-bind="lastnameProps"
        />
        <ErrorOutlet v-if="errors.firstname">{{ errors.lastname }}</ErrorOutlet>
      </div>

      <div class="flex flex-col w-full">
        <AppFormLabel for="username" text="Username" />
        <input
          id="username"
          v-model="username"
          class="app-form-input"
          type="text"
          placeholder="username"
          v-bind="usernameProps"
        />
        <ErrorOutlet v-if="errors.username">{{ errors.username }}</ErrorOutlet>
      </div>

      <SubmitButton :loading="processingRequest" />
    </form>
  </div>
</template>

<script setup lang="ts">
import axios from "axios";
import { useForm } from "vee-validate";
import { ref } from "vue";
import { useRoute, useRouter } from "vue-router";
import * as yup from "yup";
import AuthScreenHeaderText from "../../components/auth/AuthScreenHeaderText.vue";
import AppFormLabel from "../../components/form/AppFormLabel.vue";
import ErrorOutlet from "../../components/form/ErrorOutlet.vue";
import SubmitButton from "../../components/form/SubmitButton.vue";
import { useCachedUserStore } from "../../stores/cachedUser";

import { useUserInformationStore } from "../../stores/user";
import { UserProfile } from "../../types/userProfile";
import { message } from "@tauri-apps/plugin-dialog";
import { useErrorHandler } from "../../utils/handleError";

const formSchema = yup.object({
  firstname: yup.string().required(),
  lastname: yup.string().required(),
  username: yup.string().required(),
});

const { defineField, errors, handleSubmit } = useForm({
  validationSchema: formSchema,
});

const router = useRouter();
const route = useRoute();

const cachedUserStore = useCachedUserStore();
const userStore = useUserInformationStore();

const [firstname, firstnameProps] = defineField("firstname");
const [lastname, lastnameProps] = defineField("lastname");
const [username, usernameProps] = defineField("username");

const processingRequest = ref(false);
const formSubmitError = ref("");

const refreshData = (profile: UserProfile) => {
  firstname.value = profile.firstName;
  lastname.value = profile.lastName;
  username.value = profile.username;
};

const onSubmit = handleSubmit(async (values) => {
  processingRequest.value = true;
  try {
    const { firstname: firstName, lastname: lastName, username } = values;
    const token = route.query["token"];
    const response = await axios.post(
      "/auth/onboard",
      { firstName, lastName, username },
      {
        headers: {
          "Content-Type": "application/json",
          Authorization: `Bearer ${token}`,
        },
      }
    );

    if (response.status !== 200) {
      await message(response.data.data?.message, {
        kind: "error",
        title: "Request failed",
      });
    }

    const userProfile: UserProfile = response.data.data;

    userStore.updateProfile(userProfile);
    refreshData(userProfile);

    cachedUserStore.cacheUserData({
      firstName: String(userProfile.firstName),
      lastName: lastName.lastname,
      email: userProfile.email,
      avatarUrl: String(userProfile.profilePicture),
    });
    router.replace({ name: "Home" });
  } catch (error) {
    await useErrorHandler(error);
  } finally {
    processingRequest.value = false;
  }
});
</script>

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
          v-bind="firstnameAttr"
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
          v-bind="lastnameAttr"
        />
        <ErrorOutlet v-if="errors.firstname">{{ errors.lastname }}</ErrorOutlet>
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
import { UserInformation } from "../../types/userProfile";
import { useUserInformation } from "../../stores/user";

const formSchema = yup.object({
  firstname: yup.string().required(),
  lastname: yup.string().required(),
});

const { defineField, errors, handleSubmit } = useForm({
  validationSchema: formSchema,
});

const router = useRouter();
const route = useRoute();

const cachedUserStore = useCachedUserStore();
const userStore = useUserInformation();

const [firstname, firstnameAttr] = defineField("firstname");
const [lastname, lastnameAttr] = defineField("lastname");

const processingRequest = ref(false);
const formSubmitError = ref("");

const onSubmit = handleSubmit(async (values) => {
  processingRequest.value = true;
  try {
    const { firstname: firstName, lastname: lastName } = values;
    const token = route.query["token"];
    const response = await axios.post(
      "/auth/onboard",
      { firstName, lastName },
      {
        headers: {
          "Content-Type": "application/json",
          Authorization: `Bearer ${token}`,
        },
      }
    );

    if (response.status === 200) {
      const userProfile: UserInformation = await userStore.fetchUserInformation(
        token as string
      );
      cachedUserStore.cacheUserData({
        firstName: userProfile.firstName,
        lastName: lastName.lastname,
        email: userProfile.email,
        avatarUrl: userProfile.profilePicture,
        // identifier: userProfile.identifier //TODO:
      });
      router.replace({ name: "Home" });
    } else {
      formSubmitError.value = response.data.error || "Failed to onboard user";
    }
  } catch (error) {
    console.log(error);
  } finally {
    processingRequest.value = false;
  }
});
</script>

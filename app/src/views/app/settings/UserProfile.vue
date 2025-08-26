<template>
  <div class="flex flex-col gap-y-3.5 justify-center items-center">
    <UserCard :editable="true" :show-text="false" />
    <h2 class="font-medium font-xl capitalize">{{ fullName }}</h2>
  </div>

  <form class="mt-6 flex flex-col gap-8" @submit.prevent="submitForm">
    <div class="flex flex-col w-full">
      <AppFormLabel for="firstname" text="First name" />
      <input v-model="firstName" class="app-form-input" type="text" />
    </div>
    <div class="flex flex-col w-full">
      <AppFormLabel for="firstname" text="last name" />
      <input v-model="lastName" class="app-form-input" type="text" />
    </div>

    <div class="flex flex-col w-full">
      <AppFormLabel for="firstname" text="Email" />
      <input v-model="email" class="app-form-input" type="email" />
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
import { computed, ref } from "vue";
import SubmitButton from "../../../components/form/SubmitButton.vue";
import UserCard from "../../../components/settings/UserCard.vue";
import { useUserInformationStore } from "../../../stores/user";
import { useCachedUserStore } from "../../../stores/cachedUser";
import { useUpdateUserProfile } from "../../../composibles/useUpdateUserProfile";

const userInformationStore = useUserInformationStore();
const fullName = computed(() => userInformationStore.fullName);
const email = ref(userInformationStore.email);
const firstName = ref(userInformationStore.firstName);
const lastName = ref(userInformationStore.lastName);

const formSubmitted = ref(false);
const dataUnchanged = computed(() => {
  return (
    email.value === userInformationStore.email &&
    firstName.value === userInformationStore.firstName &&
    lastName.value === userInformationStore.lastName
  );
});

const submitForm = async () => {
  formSubmitted.value = true;
  try {
    const updatedProfile = await useUpdateUserProfile({
      email: email.value,
      firstName: firstName.value,
      lastName: lastName.value,
    });

    useUserInformationStore().$patch({
      email: updatedProfile.email,
      firstName: updatedProfile.firstName,
      lastName: updatedProfile.lastName,
    });

    useCachedUserStore().$patch({
      email: updatedProfile.email,
      firstName: updatedProfile.firstName,
      lastName: updatedProfile.lastName,
    });
    
  } catch (error) {
    console.error("Error updating profile:", error);
  } finally {
    formSubmitted.value = false;
  }
};
</script>

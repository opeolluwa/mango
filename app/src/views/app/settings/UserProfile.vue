<template>
  <div class="flex flex-col gap-y-3.5 justify-center items-center">
    <UserCard :editable="true" :show-text="false" :avatar-size="200" />
    <h2 class="font-medium font-xl capitalize">{{ fullName }}</h2>
  </div>

  <form class="mt-6 flex flex-col gap-8" @submit.prevent="">
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
      <input v-model="email" class="app-form-input" type="text" />
    </div>

    <SubmitButton :loading="false" class="text-white" text="Update profile" />
  </form>
</template>

<script lang="ts" setup>
import { computed, ref } from "vue";
import SubmitButton from "../../../components/form/SubmitButton.vue";
import UserCard from "../../../components/settings/UserCard.vue";
import { useUserInformationStore } from "../../../stores/user";

const userInformationStore = useUserInformationStore();
const fullName = computed(() => userInformationStore.fullName);
const email = ref(userInformationStore.email);
const firstName = ref(userInformationStore.firstName);
const lastName = ref(userInformationStore.lastName);
</script>

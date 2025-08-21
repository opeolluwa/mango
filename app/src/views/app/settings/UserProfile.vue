<template>
  <div class="flex flex-col gap-y-3.5 justify-center items-center">
    <UserCard :editable="true" :show-text="false" :avatar-size="200" />
    <h2 class="font-medium font-xl capitalize">{{ fullName }}</h2>
  </div>

  <form class="mt-6 flex flex-col gap-8">
    <div class="flex flex-col w-full">
      <AppFormLabel for="firstname" text="First name" />
      <input v-model="firstName" class="app-form-input" type="text" />
    </div>
    <div class="flex flex-col w-full">
      <AppFormLabel for="firstname" text="last name" />
      <input class="app-form-input" type="text" v-model="lastName" />
    </div>

    <div class="flex flex-col w-full">
      <AppFormLabel for="firstname" text="Email" />
      <input class="app-form-input" type="text" v-model="email" />
    </div>

    <SubmitButton :loading="false" class="text-white" text="Update profile" />
  </form>
</template>

<script lang="ts" setup>
import { computed, ref } from "vue";
import SubmitButton from "../../../components/form/SubmitButton.vue";
import UserCard from "../../../components/settings/UserCard.vue";
import { useCachedUserStore } from "../../../stores/cachedUser";
const store = useCachedUserStore();
const fullName = computed(() => store.fullName);

const email = ref(store.email);
const firstName = ref(store.firstName);
const lastName = ref(store.lastName);
</script>

<template>
  <div class="flex flex-col gap-y-3.5 justify-center items-center">
    <UserCard :editable="true" :show-text="false" :avatar-size="200" />
    <h2 class="font-medium font-xl">Maximus Charles</h2>
  </div>

  <form class="mt-6 flex flex-col gap-8">
    <div class="flex flex-col w-full">
      <AppFormLabel for="firstname" text="First name" />
      <input value="Maximum" class="app-form-input" type="text" />
    </div>
    <div class="flex flex-col w-full">
      <AppFormLabel for="firstname" text="last name" />
      <input class="app-form-input" type="text" value="charles" />
    </div>

    <div class="flex flex-col w-full">
      <AppFormLabel for="firstname" text="Email" />
      <input class="app-form-input" type="text" value="test@mailer.com" />
    </div>

    <SubmitButton :loading="false" class="text-white" text="Update profile" />
  </form>
</template>

<script lang="ts" setup>
import SubmitButton from "../../../components/form/SubmitButton.vue";
import UserCard from "../../../components/settings/UserCard.vue";
</script>

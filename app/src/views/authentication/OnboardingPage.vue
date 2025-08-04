<template>
  <div class="layout">
    <AuthScreenHeaderText>Onboard to get started</AuthScreenHeaderText>
    <p class="small text-gray-400">Register a free account to get started!</p>

    <form @submit="onSubmit" class="mt-8 flex flex-col gap-y-8">
      <div class="flex flex-col w-full">
        <AppFormLabel for="firstname" text="Firstname" />
        <input
          id="firstname"
          class="app-form-input"
          type="text"
          placeholder="firstname"
          v-model="firstname"
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
          class="app-form-input"
          type="text"
          placeholder="lastname"
          v-model="lastname"
          v-bind="lastnameAttr"
        />
        <ErrorOutlet v-if="errors.firstname">{{ errors.lastname }}</ErrorOutlet>
      </div>

      <SubmitButton />
    </form>
  </div>
</template>

<script setup lang="ts">
  import { useForm } from 'vee-validate';
  import AuthScreenHeaderText from '../../components/auth/AuthScreenHeaderText.vue';
  import AppFormLabel from '../../components/form/AppFormLabel.vue';
  import SubmitButton from '../../components/form/SubmitButton.vue';

  import * as yup from 'yup';
  import ErrorOutlet from '../../components/form/ErrorOutlet.vue';

  const formSchema = yup.object({
    firstname: yup.string().required(),
    lastname: yup.string().required(),
  });

  const { defineField, errors, handleSubmit } = useForm({
    validationSchema: formSchema,
  });

  const onSubmit = handleSubmit((values) => {
    console.log(values);
  });

  const [firstname, firstnameAttr] = defineField('firstname');
  const [lastname, lastnameAttr] = defineField('lastname');
</script>

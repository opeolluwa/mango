<template>
  <div class="layout">
    <AuthScreenHeaderText>Complete your profile</AuthScreenHeaderText>
    <p class="small text-gray-400">Just a few details to get you started</p>

    <form @submit="onSubmit" class="mt-8 flex flex-col gap-y-8">
      <ErrorOutlet v-if="formSubmitError">{{ formSubmitError }}</ErrorOutlet>

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

      <SubmitButton :loading="processingRequest" />
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
  import axios from '../../axios.config';
  import { useRoute, useRouter } from 'vue-router';
  import { ref } from 'vue';

  const route = useRoute();
  const router = useRouter();

  const formSchema = yup.object({
    firstname: yup.string().required(),
    lastname: yup.string().required(),
  });

  const { defineField, errors, handleSubmit } = useForm({
    validationSchema: formSchema,
  });

  const [firstname, firstnameAttr] = defineField('firstname');
  const [lastname, lastnameAttr] = defineField('lastname');
  const formSubmitError = ref('');
  const processingRequest = ref(false);

  const onSubmit = handleSubmit(async (values) => {
    processingRequest.value = true;
    try {
      const { firstname, lastname } = values;
      const { token } = route.params;

      const response = await axios.post(
        '/auth/onnboard',
        { firstname, lastname },
        {
          headers: {
            Authorization: `Bearer ${token}`,
          },
        }
      );

      if (response.status === 200) {
        router.push({ name: 'Home' });
      } else {
        formSubmitError.value = response.data.error || 'Failed';
      }
    } catch (error: any) {
      formSubmitError.value = error.response.data.message;
    } finally {
      processingRequest.value = false;
    }
  });
</script>

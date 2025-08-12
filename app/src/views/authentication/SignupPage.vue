<template>
  <ToastProvider>
    <div class="layout">
      <ArrowLongLeftIcon
        class="size-12 text-app-orange mb-4"
        @click="router.back"
      />

      <AuthScreenHeaderText>Create a free account</AuthScreenHeaderText>
      <p class="small text-gray-400">Register a free account to get started!</p>

      <form
        action=""
        class="mt-8 flex flex-col gap-y-8"
        @submit.prevent="submitForm"
      >
        <ErrorOutlet v-if="formSubmitError">
          {{ formSubmitError }}
        </ErrorOutlet>
        <div class="flex flex-col w-full">
          <AppFormLabel text="Email" for="email" />
          <input
            id="email"
            v-model="email"
            class="app-form-input"
            type="email"
            placeholder="jane@mailer.com"
            v-bind="emailProps"
          />
          <ErrorOutlet v-if="errors.email" class="mt-2">
            {{ errors.email }}
          </ErrorOutlet>
        </div>

        <div class="flex flex-col w-full">
          <AppFormLabel for="password" text="Password" />

          <input
            id="password"
            v-model="password"
            class="app-form-input"
            type="password"
            placeholder="********"
            v-bind="passwordProps"
          />
          <ErrorOutlet v-if="errors.password" class="mt-2">
            {{ errors.password }}
          </ErrorOutlet>
        </div>
        <div class="flex flex-col gap-2.5">
          <label
            class="flex flex-row gap-4 items-center [&>.checkbox]:hover:bg-neutral-100"
          >
            <div class="inline-flex items-center">
              <label class="flex items-center cursor-pointer relative">
                <input
                  id="check"
                  type="checkbox"
                  :checked="checkboxOne"
                  class="peer h-5 w-5 cursor-pointer transition-all appearance-none rounded shadow hover:shadow-md border border-slate-300 checked:bg-slate-800 checked:border-slate-800"
                />
                <span
                  class="absolute text-white opacity-0 peer-checked:opacity-100 top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 pointer-events-none"
                >
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    class="h-3.5 w-3.5"
                    viewBox="0 0 20 20"
                    fill="currentColor"
                    stroke="currentColor"
                    stroke-width="1"
                  >
                    <path
                      fill-rule="evenodd"
                      d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z"
                      clip-rule="evenodd"
                    ></path>
                  </svg>
                </span>
              </label>
            </div>
            <span class="select-none text-stone-700 text-sm dark:text-white"
              >Accept terms and conditions.</span
            >
          </label>
        </div>
        <SubmitButton :loading="processingRequest" />
        <RouterLink
          :to="{ name: 'Login' }"
          class="text-stone-500 flex justify-end -mt-4"
          >Already have an account?
          <span class="text-app-orange pl-1">Login</span></RouterLink
        >
      </form>
    </div>
    <ToastRoot v-model:open="processingRequest">
      <ToastTitle class="toast-title">Account Created</ToastTitle>
      <ToastDescription>
        Your account has been successfully created.
      </ToastDescription>
      <ToastViewport
        class="[--viewport-padding:_25px] fixed bottom-0 right-0 flex flex-col p-[var(--viewport-padding)] gap-[10px] w-[390px] max-w-[100vw] m-0 list-none z-[2147483647] outline-none"
      />

      <ToastViewport />
    </ToastRoot>
  </ToastProvider>
</template>

<script lang="ts" setup>
  import { ArrowLongLeftIcon } from '@heroicons/vue/24/solid';
  import {
    ToastDescription,
    ToastProvider,
    ToastRoot,
    ToastTitle,
    ToastViewport,
  } from 'reka-ui';
  import { useForm } from 'vee-validate';
  import { ref } from 'vue';
  import { useRouter } from 'vue-router';
  import * as yup from 'yup';
  import axios from '../../axios.config.ts';
  import AuthScreenHeaderText from '../../components/auth/AuthScreenHeaderText.vue';
  import AppFormLabel from '../../components/form/AppFormLabel.vue';
  import ErrorOutlet from '../../components/form/ErrorOutlet.vue';
  import SubmitButton from '../../components/form/SubmitButton.vue';
  import useToken from '../../composibles/useToken.ts';

  const validationSchema = yup.object({
    email: yup.string().required().email(),
    password: yup.string().required().min(6),
  });

  const { defineField, errors, handleSubmit } = useForm({
    validationSchema,
  });

  const router = useRouter();

  const [email, emailProps] = defineField('email');
  const [password, passwordProps] = defineField('password');

  const checkboxOne = ref(true);
  const processingRequest = ref(false);
  const formSubmitError = ref('');

  const submitForm = handleSubmit(async (values) => {
    processingRequest.value = true;

    console.log('Form submitted with values:', values);
    try {
      const response = await axios.post('/auth/signup', {
        email: values.email,
        password: values.password,
      });

      if (response.status === 201) {
        const token = response.data.data.token;
        useToken('ConfirmOtp', token, router);
      } else {
        console.error('Failed to create user:', response.data);
        formSubmitError.value =
          response.data.message || 'Failed to create user';
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

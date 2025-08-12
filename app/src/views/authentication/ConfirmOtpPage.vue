<template>
  <div
    class="w-full h-full bg-gray-900/60 absolute left-0 bottom-0 flex justify-center items-center z-500"
    v-if="processingRequest"
  >
    <Transition>
      <FormLoader />
    </Transition>
  </div>

  <div class="layout h-screen overflow-y-hidden">
    <ArrowLongLeftIcon
      class="size-12 text-app-orange mb-4"
      @click="router.back"
    />

    <AuthScreenHeaderText>Confirm OTP</AuthScreenHeaderText>
    <p class="small text-gray-400">Return the reset OTP back to us</p>

    <ErrorOutlet v-if="formSubmitError">
      {{ formSubmitError }}
    </ErrorOutlet>
    <form
      action=""
      @submit.prevent="submitForm"
      class="mt-8 flex flex-col gap-y-8"
    >
      <div class="flex flex-col w-full">
        <PinInputRoot
          :otp="true"
          id="otp"
          type="number"
          v-model="value"
          class="flex gap-x-[5px] items-center justify-center mt-1"
          @complete="handleComplete"
        >
          <PinInputInput
            v-for="(id, index) in 6"
            :key="id"
            :index="index"
            class="size-12 border-stone-600 dark:border-stone-500 hover:border-app-orange rounded-lg text-center shadow-sm border text-green10 placeholder:text-mauve8 focus:shadow-[0_0_0_2px] focus:shadow-stone-800 outline-none"
          />
        </PinInputRoot>
      </div>
      <div class="text-stone-500 justify-end flex items-center">
        <span>Did&apos;t get an OTP? Request new after</span>
        <span class="text-app-orange pl-1">{{ remaining }}s</span>
      </div>
    </form>
  </div>
</template>

<script lang="ts" setup>
  import { ArrowLongLeftIcon } from '@heroicons/vue/24/solid';
  import { useRoute, useRouter } from 'vue-router';
  import { PinInputInput, PinInputRoot } from 'reka-ui';
  import { onMounted, ref } from 'vue';
  import FormLoader from '../../components/form/FormLoader.vue';
  import AuthScreenHeaderText from '../../components/auth/AuthScreenHeaderText.vue';
  import { useCountdown } from '@vueuse/core';
  // import axios from '../../axios.config';
  import useToken from '../../composibles/useToken';
  import ErrorOutlet from '../../components/form/ErrorOutlet.vue';
  import axios from 'axios';

  const countdownSecs = 30;

  const { remaining, start } = useCountdown(countdownSecs, {
    onComplete() {},
    onTick() {},
  });

  const value = ref<number[]>([]);
  function handleComplete(otp: number[]) {
    console.log(otp);
    submitForm();
  }

  const router = useRouter();
  const route = useRoute();

  const processingRequest = ref(false);
  const formSubmitError = ref('');

  const submitForm = async () => {
    processingRequest.value = true;

    const otp = value.value.join('');

    try {
      console.log(otp);
      const response = await axios.post(
        '/auth/verify-account',
        { otp },
        {
          headers: {
            'Content-Type': 'application/json',
            Authorization: `Bearer ${route.query.token}`,
          },
        }
      );
      console.log(response);
      if (response.status === 200) {
        const { token } = response.data.data;
        useToken('Onboarding', token, router);
      } else {
        console.error('Failed to create user:', response.data);
        formSubmitError.value =
          response.data.message || 'Failed to create user';
      }
    } catch (error: any) {
      console.log(error);
      formSubmitError.value = error.response.data.message;
    } finally {
      processingRequest.value = false;
    }
  };

  onMounted(() => {
    start();
  });
</script>

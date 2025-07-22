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

    <AuthScreenHeaderText>Confirm reset OTP</AuthScreenHeaderText>
    <p class="small text-gray-400">Return the reset otp back to us</p>

    <form
      action=""
      @submit.prevent="submitForm"
      class="mt-8 flex flex-col gap-y-8"
    >
      <div class="flex flex-col w-full">
        <PinInputRoot
          id="otp"
          v-model="value"
          class="flex gap-2 items-center justify-evenly mt-1"
          @complete="handleComplete"
        >
          <PinInputInput
            v-for="(id, index) in 5"
            :key="id"
            :index="index"
            class="w-12 h-12 border-stone-600 dark:border-stone-500 hover:border-app-orange rounded-lg text-center shadow-sm border text-green10 placeholder:text-mauve8 focus:shadow-[0_0_0_2px] focus:shadow-stone-800 outline-none"
          />
        </PinInputRoot>
      </div>
      <div class="text-stone-500 justify-end flex items-center text-sm">
        <span>Did&apos;t get an OTP? Request new after</span>
        <span class="text-app-orange pl-1">86s</span>
      </div>

      <RouterLink
        :to="{ name: 'SetNewPassword' }"
        class="text-stone-500 flex justify-end -mt-4"
        >new pswd</RouterLink
      >
    </form>
  </div>
</template>

<script lang="ts" setup>
import { ArrowLongLeftIcon } from "@heroicons/vue/24/solid";
import { useRouter } from "vue-router";
import { PinInputInput, PinInputRoot } from "reka-ui";
import { ref } from "vue";
import FormLoader from "../../components/form/FormLoader.vue";
import AuthScreenHeaderText from "../../components/auth/AuthScreenHeaderText.vue";

const value = ref<string[]>([]);
function handleComplete(otp: string[]) {
  console.log(otp);
  submitForm();
}

const router = useRouter();
const processingRequest = ref(false);
const submitForm = async () => {
  processingRequest.value = true;
};
</script>

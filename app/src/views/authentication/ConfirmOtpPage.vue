<template>
  <div class="layout h-screen overflow-y-hidden">
    <ArrowLongLeftIcon
      class="size-12 text-app-orange mb-4"
      @click="router.back"
    />

    <h1 class="text-5xl">Confirm reset OTP</h1>
    <p class="small text-gray-400">Return the reset otp back to us</p>

    <form
      action=""
      @submit.prevent="sendForm"
      class="mt-8 flex flex-col gap-y-8"
    >
      <div class="flex flex-col w-full">
        <AppFormLabel text="Otp" for="email" />
        <PinInputRoot
          id="otp"
          v-model="value"
          placeholder="○"
          class="flex gap-2 items-center justify-evenly mt-1"
          @complete="handleComplete"
        >
          <PinInputInput
            v-for="(id, index) in 5"
            :key="id"
            :index="index"
            class="w-12 h-12 bg-app-orange/50 border-app-gray hover:border-app-orange rounded-lg text-center shadow-sm border text-green10 placeholder:text-mauve8 focus:shadow-[0_0_0_2px] focus:shadow-stone-800 outline-none"
          />
        </PinInputRoot>
      </div>
      <div
        to="/forgotten-password"
        class="text-stone-500 justify-end flex items-center"
      >
        <small>Did&apos;t get an OTP? Request new after</small>
        <small class="text-app-orange">86s</small>
      </div>
    </form>
  </div>
</template>

<script lang="ts" setup>
import AppFormLabel from "../../components/form/AppFormLabel.vue";
import { ArrowLongLeftIcon } from "@heroicons/vue/24/solid";
import { useRouter } from "vue-router";
import { PinInputInput, PinInputRoot } from "reka-ui";
import { ref } from "vue";

const value = ref<string[]>([]);
function handleComplete(e: string[]) {
  // eslint-disable-next-line no-alert
  alert(e.join(""));
}

const router = useRouter();
const sendForm = async () => {};
</script>

<style scoped>
*::-webkit-scrollbar-thumb {
  background-color: transparent;
  opacity: 0.75;
  border-radius: 6px;
  border: 3px solid transparent;
}
</style>

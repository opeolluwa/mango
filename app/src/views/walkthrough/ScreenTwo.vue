<template>
  <div
    class="relative flex flex-col h-screen max-w-screen walkthrough_layout"
  >
    <img src="../../assets/illustrations/listen.svg" alt="" class="h-[50vh]" />
    <h1
      class="text-2xl mt-2 font-extrabold leading-loose text-center dark:text-white/70 w-[90%]"
      style="line-height: 20px; font-weight: 800"
    >
      Listen offline or online
    </h1>
    <p class="text-center text-gray-400 text-150">
      Listen to audio equivalent of your documents offline or online
    </p>

     <SensationalTint class="absolute inset-x-2 bottom-1 -z-10 transform-gpu overflow-hidden blur-3xl opacity-30 " />

    <div class="flex justify-between w-full mt-8 absolute px-8 bottom-7">
      <RouterLink
        :to="{ name: 'ScreenOne' }"
        class=" dark:text-gray-400 font-medium bg-transparent rounded-lg py-4 text-center hover:animate-pulse h-fit text-gray-400"
      >
        Prev
      </RouterLink>
      <RouterLink
        :to="{ name: 'ScreenThree' }"
        class="text-white bg-app-orange h-fit py-2 px-6 rounded-lg"
      >
        Next
      </RouterLink>
    </div>
  </div>
</template>
<script lang="ts" setup>
import SensationalTint from "@/components/uiBlocks/SensationalTint.vue";
import { RouterLink } from "vue-router";
</script>



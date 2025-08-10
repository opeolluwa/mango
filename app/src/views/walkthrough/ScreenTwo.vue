<template>
  <div
    class="px-8 py-2 relative flex flex-col h-screen justify-center items-center w-screen layout"
  >
    <img src="/hero.svg" alt="" class="w-[250px]" />
    <h1
      class="text-2xl mt-8 font-extrabold leading-loose text-center dark:text-white/70 w-[90%]"
      style="line-height: 50px; font-weight: 800"
    >
      Listen offline or online
    </h1>
    <p class="text-center text-150">
      Listion to audio equivalent of your documents offline or online
    </p>

    <SensationalTint />

    <div class="flex justify-between w-full mt-8 absolute px-8 bottom-7">
      <RouterLink
        :to="{ name: 'ScreenOne' }"
        class="text-app-dark dark:text-gray-400 font-medium bg-transparent rounded-lg py-4 text-center hover:animate-pulse h-fit"
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

<template>
  <div
    class="px-8 py-2 relative flex flex-col h-screen justify-center items-center w-screen layout"
  >
    <img src="/add-file-2.svg" alt="" class="w-[250px]" />
    <h1
      class="text-2xl mt-8 font-extrabold leading-loose text-center dark:text-white/70 w-[90%]"
      style="line-height: 50px; font-weight: 800"
    >
      Upload document
    </h1>
    <p class="text-center text-150 w-[90%]">
      Upload a new PDF document to our high frequency servers
    </p>

    <SensationalTint />

    <div class="flex justify-between w-full mt-8 absolute px-8 bottom-7">
      <RouterLink
        :to="{ name: 'Login' }"
        class="text-app-dark dark:text-gray-400 font-medium bg-transparent rounded-lg py-4 text-center hover:animate-pulse h-fit"
      >
        Skip
      </RouterLink>
      <RouterLink
        :to="{ name: 'ScreenTwo' }"
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

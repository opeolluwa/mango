<template>
  <div
    class="px-8 flex flex-col h-screen pb-[15vh] justify-end w-screen relative layout"
  >
    <div class="relative -bottom-10" style="clip-path: circle(62% at 41% 41%)">
      <img src="/screen-three.png" alt="" style="" />
    </div>
    <h1
      class="text-4xl mt-16 capitalize leading-loose dark:text-white/70 w-[90%]"
      style="line-height: 50px"
    >
      <span class="font-bold">Elevate</span>
      your reading with <span class="font-bold">quick insight</span>
    </h1>
    <div class="flex justify-between items-center mt-4">
      <div
        class="size-14 flex justify-end items-center mt-4 rounded-full border-2 border-l-app-orange hover:animate-ping border-transparent"
      >
        <RouterLink
          :to="{ name: 'ScreenThree' }"
          class="size-12  rounded-full border-r border-r-app-dark-300 text-white bg-app-dark justify-center items-center hidden"
        >
          <ArrowLongLeftIcon class="size-6" />
        </RouterLink>
      </div>
      <div
        class="size-14 flex justify-end items-center mt-4 rounded-full border-2 border-b-app-orange hover:animate-ping border-transparent"
      >
        <RouterLink
          :to="{ name: 'ScreenThree' }"
          class="size-12 bg-app-orange rounded-full border-l border-l-app-dark-300 text-white flex justify-center items-center"
        >
          <ArrowLongRightIcon class="size-6" />
        </RouterLink>
      </div>
    </div>
    <SensationalTint />
    <RouterLink
      :to="{ name: 'Home' }"
      class="leading-loose flex mt-2 relative justify-end -bottom-24"
    >
    </RouterLink>
  </div>
</template>
<script lang="ts" setup>
import { ArrowLongLeftIcon, ArrowLongRightIcon } from "@heroicons/vue/24/solid";
import { RouterLink } from "vue-router";
import SensationalTint from "@/components/uiBlocks/SensationalTint.vue";
</script>

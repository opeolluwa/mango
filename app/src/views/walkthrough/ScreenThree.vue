<template>
  <div
    class="px-8 flex flex-col h-screen pb-[15vh] justify-end w-screen relative layout"
  >
    <img src="/screen-three-02.png" alt="" />
    <h1
      class="text-4xl mt-16 capitalize leading-loose dark:text-white/70"
      style="line-height: 50px"
    >
      stay
      <span class="font-bold">Motivated</span>
      and achieve <span class="font-bold">Goals</span>
    </h1>

    <div
      class="size-14 flx justify-center items-center mt-4 rounded-full border-r-2 border-r-app-orange border-app-orange-50/20 hidden"
    >
      <RouterLink
        :to="{ name: 'SignUp' }"
        class="size-12 bg-app-orange rounded-full border-r border-r-app-orange-300 text-white border-2 flex justify-center items-center"
      >
        <ArrowLongRightIcon class="size-6" />
      </RouterLink>
    </div>
    <SensationalTint />
    <RouterLink
      :to="{ name: 'SignUp' }"
      class="relative -bottom-14 border border-app-dark  bg-app-dark text-white font-medium text-app-dark rounded-lg py-4 text-center"
    >
      Get started
    </RouterLink>
  </div>
</template>
<script lang="ts" setup>
import SensationalTint from "@/components/uiBlocks/SensationalTint.vue";
import { ArrowLongRightIcon } from "@heroicons/vue/24/solid";
import { RouterLink } from "vue-router";
</script>

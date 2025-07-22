<template>
  <div
    class="px-8 flex flex-col h-screen pb-[5vh] justify-end w-screen relative layout"
  >
    <img src="/screen-three.png" alt="" />

    <h1
      class="text-4xl mt-16 capitalize leading-loose dark:text-white/70"
      style="line-height: 50px"
    >
      stay
      <span class="font-bold">Motivated</span>
      and achieve <span class="font-bold">Goals</span>
    </h1>
    <SensationalTint />
    <div
      class="flex justify-between items-center flex-row-reverse relative mt-4"
    >
      <div
        class="size-14 flex justify-center items-center rounded-full border-r-2 border-r-app-orange border-app-orange-50/20"
      >
        <RouterLink
          :to="{ name: 'SignUp' }"
          class="size-12 bg-app-orange rounded-full border-r border-r-app-orange-300 text-white flex justify-center items-center"
        >
          <ArrowLongRightIcon class="size-6" />
        </RouterLink>
      </div>
      <RouterLink
        :to="{ name: 'SignUp' }"
        class="leading-loose flex relative justify-end"
      >
        Skip
      </RouterLink>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ArrowLongRightIcon } from "@heroicons/vue/24/solid";
import { RouterLink } from "vue-router";
import SensationalTint from "@/components/uiBlocks/SensationalTint.vue";
</script>

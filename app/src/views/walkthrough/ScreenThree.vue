<template>
  <div
    class="px-8 flex flex-col h-screen pb-[5vh] justify-end w-screen relative layout"
  >
    <img src="/screen-three.png" alt="" class="rounded-xl shadow-sm" />

    <h1
      class="text-4xl mt-16 capitalize leading-loose dark:text-white/70"
      style="line-height: 50px"
    >
      stay
      <span class="font-bold">Motivated</span>
      and achieve <span class="font-bold">Goals</span>
    </h1>

    <div
      class="absolute inset-x-2 top-0 -z-10 transform-gpu overflow-hidden blur-3xl"
      aria-hidden="true"
    >
      <div
        class="relative left-[calc(50%+3rem)] aspect-1155/678 w-144.5 -translate-x-1/2 bg-linear-to-tr from-app-orange-400 to-[#9089fc] opacity-70"
        style="
          clip-path: polygon(
            74.1% 44.1%,
            100% 61.6%,
            97.5% 26.9%,
            85.5% 0.1%,
            80.7% 2%,
            72.5% 32.5%,
            60.2% 62.4%,
            52.4% 68.1%,
            47.5% 58.3%,
            45.2% 34.5%,
            27.5% 76.7%,
            0.1% 64.9%,
            17.9% 100%,
            27.6% 76.8%,
            76.1% 97.7%,
            74.1% 44.1%
          );
        "
      />
    </div>

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
</script>

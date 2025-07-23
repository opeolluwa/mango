<template>
  <div
    class="px-8 flex flex-col h-screen pb-[15vh] justify-end w-screen relative layout"
  >
    <img src="/screen-one.png" alt="" />
    <h1
      class="text-4xl capitalize leading-loose w-[90%] dark:text-white/70"
      style="line-height: 50px"
    >
      <span class="font-bold">Empower</span>
      yourself with <span class="font-bold">quick</span> knowledge
    </h1>

    <div
      class="size-14 flex justify-center items-center mt-4 rounded-full border-r-2 border-r-app-orange border-app-orange-50/20 hover:animate-ping"
    >
      <RouterLink
        :to="{ name: 'ScreenTwo' }"
        class="size-12 bg-app-orange rounded-full border-r border-r-app-orange-300 text-white flex justify-center items-center"
      >
        <ArrowLongRightIcon class="size-6" />
      </RouterLink>
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
import { ArrowLongRightIcon } from "@heroicons/vue/24/solid";
import { RouterLink } from "vue-router";
import SensationalTint from "@/components/uiBlocks/SensationalTint.vue";
</script>

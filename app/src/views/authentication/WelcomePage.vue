<template>
  <div class="px-8 flex flex-col h-screen pb-[15vh] justify-end w-screen">
    <AppLogo class="text-center" />
    <AppH1 class="text-center font-medium"
      >Convert your text document to audio</AppH1
    >
    <p class="leading-loose text-gray-400 text-center">
      Lorem ipsum dolor sit amet consectetur adipisicing elit. Perspiciatis
      laudantium deserunt deleniti,
    </p>

    <div
      class="grid grid-cols-2 items-center gap-x-4 justify-between font-medium mt-12"
    >
      <AppButton
        class="border-app-orange col-span-1 text-app-orange rounded-2xl border"
        >Login
      </AppButton>

      <AppButton
        class="border-app-orange col-span-1 text-app-dark bg-app-orange rounded-2xl border"
        >Sign up
      </AppButton>
    </div>
  </div>
</template>

<script lang="ts" setup>
import AppButton from "../../components/button/appButton.vue";
import AppH1 from "../../components/typgraphy/AppH1.vue";
import AppLogo from "../../components/uiBlocks/AppLogo.vue";
</script>

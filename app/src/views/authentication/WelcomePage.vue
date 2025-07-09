<template>
  <div
    class="px-8 flex flex-col h-screen pb-[15vh] justify-end w-screen bg-linear-210 from-app-orange/60 from-10% via-app-dark to-app-orange/20 relative"
  >
    <AppLogo
      class="text-center rounded-full border-app-orange/10 bg-app-dark/10 border-2 w-fit mx-auto px-4 py-[2px]"
    />
    <h1 class="text-7xl leading-20 text-center">Book streaming platform!</h1>
    <p class="leading-loose text-gray-400 text-center mt-4">
      Convert text documents to audio books, share, collaborate and learn
    </p>

    <div
      class="grid grid-cols-2 items-center gap-x-4 justify-between font-medium mt-12"
    >
      <AppButtonLink
        href="/login2"
        class="border-app-orange col-span-1 text-app-orange rounded-2xl border flex justify-center"
        >Login
      </AppButtonLink>

      <AppButtonLink
        href="/signup"
        class="border-app-orange col-span-1 text-app-dark bg-app-orange rounded-2xl border flex justify-center"
        >Sign up
      </AppButtonLink>
    </div>
    <a
      href="#"
      class="leading-loose text-gray-400/50 mx-auto text-center mt-4 relative -bottom-20"
    >
      Terms of use and privacy policy
    </a>
  </div>
</template>

<script lang="ts" setup>
import AppButtonLink from "../../components/button/AppButtonLink.vue";
import AppLogo from "../../components/uiBlocks/AppLogo.vue";
</script>



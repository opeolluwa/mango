<template>
  <div
    v-show="showSideNav"
    class="w-screen  z-[5000]  absolute top-2 left-0 h-[100vh] bg-app-dark/10 backdrop-blur-sm"
    @click="toggleSideNav"
  >
    <Transition name="slide-fade">
      <AppNavigation v-if="showSideNav" class="w-[70vw] absolute dark:bg-app-dark z-[5000] " />
    </Transition>
  </div>
  <header
    class="flex gap-x-4 items-center justify-between dark:border-none border-b min-h-12 py-2 px-4 border-gray-100/40 bg-linear-to-b from-white to-app-orange-50/40 dark:bg-app-dark/50 dark:from-app-dark/50 dark:to-app-dark/50 backdrop-blur-md shadow-white/20 shadow-sm hidden"
  >
    <Icon
      icon="mage:dash-menu"
      class="text-gray-400 size-6 flex items-center h-full cursor-pointer"
      @click="toggleSideNav"
    />

    <div class="col-span-4 flex gap-x-3 items-center">
      <RouterLink to="/notification">
        <Icon
          icon="mage:notification-bell"
          class="text-gray-400 size-5 flex items-center h-full cursor-pointer"
        />
      </RouterLink>
      <AvatarRoot
        class="bg-blackA3 inline-flex size-10 select-none items-center justify-center overflow-hidden rounded-full align-middle"
      >
        <AvatarImage
          class="h-full w-full rounded-[inherit] object-cover"
          src="https://images.unsplash.com/photo-1492633423870-43d1cd2775eb?&w=128&h=128&dpr=2&q=80"
          alt="Colm Tuite"
        />
        <AvatarFallback
          class="text-grass11 dark:text-stone-300 leading-1 flex h-full w-full items-center justify-center bg-white dark:bg-stone-800 text-sm font-medium"
          :delay-ms="600"
        >
          CT
        </AvatarFallback>
      </AvatarRoot>
    </div>
  </header>
</template>

<script lang="ts" setup>
import { Icon } from "@iconify/vue";
import { AvatarFallback, AvatarImage, AvatarRoot } from "reka-ui";
import { ref } from "vue";
import AppNavigation from "./AppNavigation.vue";

const showSideNav = ref(false);
const toggleSideNav = () => (showSideNav.value = !showSideNav.value);
</script>

<style>
.slide-fade-enter-active,
.slide-fade-leave-active {
  transition: opacity 0.4s cubic-bezier(0.4, 0, 0.2, 1), transform 0.4s cubic-bezier(0.4, 0, 0.2, 1);
}
.slide-fade-enter-from,
.slide-fade-leave-to {
  opacity: 0;
  transform: translateX(-40px);
}
.slide-fade-enter-to,
.slide-fade-leave-from {
  opacity: 1;
  transform: translateX(0);
}

/* we will explain what these classes do next! */
.v-enter-active,
.v-leave-active {
  transition: opacity 0.5s ease;
}

.v-enter-from,
.v-leave-to {
  opacity: 0;
}

.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.5s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>

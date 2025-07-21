<template>
  <div
    class="w-scren z-101 absolute top-2 left-0 h-[100vh]"
    @click="toggleSideNav"
    v-show="showSideNav"
  >
    <Transition name="fase">
      <AppNavigation class="w-[70vw] absolute bg-app-dark" />
    </Transition>
  </div>
  <header
    class="flex gap-x-4 items-center justify-between dark:border-none border-b min-h-12 py-2 px-4 border-gray-100/50 bg-white shadow-sm dark:border-gray-100 dark:bg-app-orange-100/10"
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
// const store = useAudioBookLibrary();
// const emptyLibrary = computed(() => store.audioBooks.length == 0);

const showSideNav = ref(false);
const toggleSideNav = () => (showSideNav.value = !showSideNav.value);
</script>

<style>
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

<template>
  <div
    class="w-screen z-101 absolute top-2 bg-app-dark/80 h-[100vh]"
    @click="toggleSideNav"
    v-show="showSideNav"
  >
    <AppNavigation class="w-[70vw] absolute bg-app-dark" />
  </div>
  <header class="flex gap-x-4 items-center justify-between">
    <div class="gap-x-3 items-center hidden lg:flex">
      <Icon
        icon="mingcute:left-fill"
        class="text-gray-400/50 size-5 flex items-center h-full icon"
        @click="goBack()"
      />
      <Icon
        icon="mingcute:right-fill"
        class="text-gray-400/50 size-5 flex items-center h-full icon"
        @click="goForward()"
      />
    </div>

    <Icon
      icon="material-symbols:menu-rounded"
      class="text-gray-400/50 size-6 flex items-center h-full cursor-pointer"
      @click="toggleSideNav"
    />

    <form
      action=""
      class="bg-gray-50/10 flex gap-x-2 items-center rounded-xl h-10 relative pl-4 w-3/5"
    >
      <Icon
        icon="fluent:search-16-filled"
        class="text-gray-400/50 size-5 flex items-center h-full"
      />
      <input
        type="search"
        class="w-full h-10 outline-none border-none pr-6 placeholder:text-gray-50/10 placeholder:text-sm placeholder:leading-loose"
        placeholder="Search by artist, song or albulm"
      />
    </form>
    <div class="col-span-4 flex gap-x-3 items-center">
      <RouterLink to="/notification">
        <Icon
          icon="mdi:bell-outline"
          class="text-gray-400/50 size-5 flex items-center h-full cursor-pointer"
        />
      </RouterLink>
      <AvatarRoot
        class="bg-blackA3 inline-flex h-[30px] w-[30px] select-none items-center justify-center overflow-hidden rounded-full align-middle"
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
import { goBack, goForward } from "../../hooks/router";
import { AvatarFallback, AvatarImage, AvatarRoot } from "reka-ui";
import { ref } from "vue";
import AppNavigation from "./AppNavigation.vue";

const showSideNav = ref(false);
const toggleSideNav = () => (showSideNav.value = !showSideNav.value);
</script>

<template>
  <div
    id="container"
    class="bg-app-dark h-screen w-screen text-white grid grid-cols-12 min-w-[600px]"
  >
    <AppNavigation />
    <main class="col-span-6 layout parent-element">
      <heading class="paent-element">
        <form
          action=""
          class="bg-gray-50/10 flex gap-x-2 items-center rounded-md h-10 relative"
        >
          <!-- <Icon
            icon="fluent:search-16-filled"
            class="text-gray-400/50 size-8 flex items-center h-full pl-2"
          /> -->
          <input
            type="text"
            class="w-full h-10 outline-none border-none pr-6 placeholder:text-gray-400/50 placeholder:text-center"
            placeholder="Search by artist, song or albulm"
          />
        </form>
      </heading>
    </main>
    <aside class="col-span-3"></aside>
    <AppFooter />
  </div>
</template>

<script lang="ts" setup>
import { Icon } from "@iconify/vue";
import AppFooter from "./components/app/AppFooter.vue";
import AppNavigation from "./components/app/AppNavigation.vue";
</script>
<style scoped>
@reference "./assets/styles.css";
</style>

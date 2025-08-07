<template>
  <nav
    class="flex justify-between py-8 fixed z-[100] items-center capitalize fied w-full min-h-20 top-0"
  >
    <div>logo</div>
    <ul class="flex items-center gap-4 cursor-pointer text-150">
      <li v-for="(route, index) in routes" :key="index">
        <NuxtLink :to="route.path">{{ route.label }} </NuxtLink>
      </li>
    </ul>
    <button
      class="border cursor-pointer border-app-orange text-medium px-8 py-3 rounded-lg"
    >
      Download
    </button>
  </nav>
  <main
    class="pt-28 bg-blend-multiply backdrop-grayscale-100 bg-linear-to-br rounded-t-xl shadow-current from-app-orange/10 via-app-dark to-app-orange/20 bg-center h-screen mt-20"
  >
    <div id="hero" class="flex justify-center flex-col items-center mx-auto">
      <h1 class="text-7xl w-[85%] font-bold text-center text-balance">
        Supercharge your text documents
      </h1>
      <p class="text-stone-400 mt-2 font-larger text-150">
        Lorem ipsum dolor sit amet, consectetur adipisicing elit. Iure
        temporibus deserunt q.
      </p>
      <div class="mt-8 flex gap-4 items-center">
        <button class="bg-app-orange text-medium px-8 py-3 rounded-lg">
          Sign up
        </button>

        <button
          class="border border-app-orange text-medium px-8 py-3 rounded-lg"
        >
          Try free
        </button>
      </div>
    </div>
  </main>
  <footer></footer>
</template>

<script lang="ts" setup>
const routes: Array<{ label: string; path: string }> = [
  { label: "home", path: "/" },
  { label: "about", path: "/" },
  { label: "premium", path: "/" },
  { label: "news", path: "/" },
];
</script>

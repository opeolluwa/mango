<template>
  <nav class="flex justify-between py-8 items-center capitalize ">
    <div>logo</div>
    <ul class="flex items-center gap-4 cursor-pointer">
      <li v-for="(route, index) in routes" :key="index">
        <NuxtLink :to="route.path">{{ route.label }} </NuxtLink>
      </li>
    </ul>
    <button class="border cursor-pointer border-app-orange text-medium px-8 py-3 rounded-lg">
      Download
    </button>
  </nav>
  <main class="pt-28 bg-blend-multiply backdrop-grayscale-100 bg-linear-to-bl rounded-t-xl shadow-current from-app-orange/30 via-app-dark to-app-orange/50  bg-center">
    <div id="hero" class="flex justify-center items-center mx-auto">
      <div>
        <h1 class="text-7xl text-balance">Supercharge your text documents</h1>
        <p>
          Lorem ipsum dolor sit amet, consectetur adipisicing elit. Iure
          temporibus deserunt q.
        </p>
        <div class="mt-8 flex gap-4 items-center">
          <button class="bg-app-orange text-medium px-8 py-3 rounded-lg">
            Download
          </button>

          <button
            class="border border-app-orange text-medium px-8 py-3 rounded-lg"
          >
            try free
          </button>
        </div>
      </div>
    <div>
        <NuxtImg src="../public/feed.png" />
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

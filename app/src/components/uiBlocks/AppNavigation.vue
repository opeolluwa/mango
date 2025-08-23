<template>
  <nav
    class="border-r border-r-gray-500/10 layout flex flex-col gap-y-16 h-screen text-app-white/60 bg-white dark:bg-app-dark"
  >
    <div class="nav-section">
      <AppLabeledIcon icon="mynaui:signal" label="Feed" to="/" />
      <AppLabeledIcon
        icon="ph:playlist-duotone"
        label="Library"
        to="/library"
      />
      <AppLabeledIcon icon="hugeicons:bot" label="AI Voice" to="/voices" />
    </div>

    <div class="nav-section">
      <small class="nav-section-heading parent-element">Your audio books</small>
      <AppLabeledIcon
        icon="mdi:book-plus-outline"
        label="Import new"
      />
      <AppLabeledIcon
        icon="fluent:heart-28-regular"
        label="Favourites"
        to="/favourites"
      />
      <AppLabeledIcon
        icon="solar:calendar-broken"
        label="History"
        class=""
        to="/history"
      />
    </div>

    <div class="nav-section">
      <small class="nav-section-heading parent-element">Your playlist</small>
      <PlayListTag
        v-for="(playlist, index) in playlists"
        :key="index"
        :name="playlist.name"
        :color="playlist.color"
      />

      <button
        class="outline-none border-none text-app-orange flex items-center gap-x-2 px-6 cursor-pointer text-sm"
      >
        Create playlist
        <Icon icon="stash:plus" class="icon" />
      </button>
    </div>
  </nav>
</template>

<script lang="ts" setup>
import { Icon } from "@iconify/vue";
import { ref } from "vue";
import PlayListTag from "../PlayListTag.vue";
import AppLabeledIcon from "./AppLabeledIcon.vue";

const playlists = ref<{ name: string; color: string }[]>([
  // {
  //   name: "Engineering",
  //   color: "orange",
  // },
  // {
  //   name: "Self help",
  //   color: "green",
  // },
  // {
  //   name: "Business",
  //   color: "blue",
  // },
  // {
  //   name: "History",
  //   color: "red",
  // },
  // {
  //   name: "Science",
  //   color: "purple",
  // },
]);
</script>

<style scoped>
@reference "../../assets/styles.css";

.nav-section {
  @apply flex flex-col gap-y-6;
}

.nav-section-heading {
  @apply px-8 text-gray-400/50  pb-0 gap-0 -mb-1.5 capitalize;
}
</style>

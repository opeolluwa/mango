<template>
  <nav
    class="border-r z-5000 border-r-gray-500/10 layout flex flex-col gap-y-16 h-screen text-app-white/60"
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
        @click="createNewBook"
      />
      <AppLabeledIcon
        icon="fluent:heart-28-regular"
        label="Favourites"
        to="/favourites"
      />
      <AppLabeledIcon
        icon="fluent:clock-16-regular"
        label="Recent"
        to="/recent"
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
        @click="createNewPlaylist()"
      >
        Import new book
        <Icon icon="stash:plus" class="icon" />
      </button>
    </div>
  </nav>
</template>

<script lang="ts" setup>
import { ref } from "vue";
import { createNewBook } from "../../composibles/book.ts";
import PlayListTag from "../PlayListTag.vue";
import AppLabeledIcon from "./AppLabeledIcon.vue";
import { Icon } from "@iconify/vue";
import { createNewPlaylist } from "../../composibles/modals.ts";

const playlists = ref<{ name: string; color: string }[]>([]);
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

<template>
  <nav
    class="col-span-3 border-r border-r-gray-500/10 layout flex flex-col gap-y-16 h-screen"
  >
    <AppLogo class="parent px-8" />

    <div class="nav-section">
      <AppLabeledIcon icon="mynaui:signal" label="Feed" />
      <AppLabeledIcon icon="ph:playlist-duotone" label="Playlist" />
      <AppLabeledIcon icon="bx:bar-chart" label="Statistics" />
    </div>

    <div class="nav-section">
      <small class="nav-section-heading parent-element">Your music</small>
      <AppLabeledIcon icon="fluent:heart-28-regular" label="Favourites" />
      <AppLabeledIcon icon="fluent:clock-16-regular" label="Listen later" />
      <AppLabeledIcon icon="solar:calendar-broken" label="History" class="" />
      <AppLabeledIcon
        icon="material-symbols-light:podcasts-rounded"
        label="Podcast"
      />
    </div>

    <div class="nav-section">
      <small class="nav-section-heading parent-element">Your playlist</small>
      <PlayListTag name="Metalcore" color="red" />
      <PlayListTag name="Electro" color="lime" />
      <PlayListTag name="Func" color="yellow" />
      <PlayListTag name="Disco" color="purple" />

      <button
        class="outline-none border-none text-app-orange flex items-center gap-x-2 px-6 cursor-pointer text-sm"
      >
        Create a new playlist <Icon icon="stash:plus" class="icon" />
      </button>
    </div>
  </nav>
</template>

<script lang="ts" setup>
import PlayListTag from "../PlayListTag.vue";
import AppLabeledIcon from "./AppLabeledIcon.vue";
import AppLogo from "./AppLogo.vue";
import { Icon } from "@iconify/vue";
</script>

<style scoped>
@reference "../../assets/styles.css";

.nav-section {
  @apply flex flex-col gap-y-5;
}

.nav-section-heading {
  @apply px-8 text-gray-400/50  pb-0 gap-0 -mb-1.5 capitalize;
}
</style>

<template>
  <form
      class="flex flex-col absolute left-[40%] w-[35%] z-[1000] bg-app-gray px-8 pb-8 py-4 rounded-lg shadow-sm shadow-app-gray gap-y-4"
      @submit.prevent="createNewPlaylist"
  >
    <h2 class="mt-6 text-center font-bold">Create a new playlist</h2>

    <div class="max-w-sm mb-3">
      <label for="input-label" class="block text-sm font-medium mb-2 dark:text-white">Name</label>
      <input type="text" id="input-label"
             class="py-2.5 sm:py-3 px-4 block w-full border-gray-200 rounded-lg sm:text-sm focus:border-app-orange/50 focus:ring-app-orange/50 disabled:opacity-50 disabled:pointer-events-none dark:bg-neutral-900 dark:border-neutral-700 dark:text-neutral-400 dark:placeholder-neutral-500 dark:focus:ring-neutral-600"
             placeholder="astronomy" v-model="name">
    </div>

    <div class="max-w-sm mb-3">
      <label for="textarea-label" class="block text-sm font-medium mb-2 dark:text-white">Description</label>
      <textarea id="textarea-label"
                class="py-2 px-3 sm:py-3 sm:px-4 block w-full border-gray-200 rounded-lg sm:text-sm focus:border-app-orange/50 focus:ring-app-orange/50 disabled:opacity-50 disabled:pointer-events-none dark:bg-neutral-900 dark:border-neutral-700 dark:text-neutral-400 dark:placeholder-neutral-500 dark:focus:ring-neutral-600"
                rows="3" placeholder="A collection of the lunar star names..." v-model="description"></textarea>
    </div>

    <button type="submit"
            class="py-3 px-4 inline-flex  items-center justify-center gap-x-2 text-sm font-medium rounded-lg border border-transparent bg-app-orange text-center text-white hover:bg-app-orange/70 focus:outline-hidden focus:bg-app-orange/70 disabled:opacity-50 disabled:pointer-events-none">
      Create
    </button>
  </form>


</template>


<script setup lang="ts">
import {ref} from "vue";
import {Playlist} from "../composibles/database.ts";
import {message} from '@tauri-apps/plugin-dialog';
import _ from "lodash";
// import {closeModal} from "jenesius-vue-modal";

const name = ref("");
const description = ref("");

const createNewPlaylist = async () => {
  console.log("Creating new playlist", name.value, description.value);
  const playlist = new Playlist(name.value, description.value);
  playlist.save().then(async () => {
    await message(`${_.capitalize(name.value)} playlist created successfully`, {title: 'Echo', kind: 'info'});
  }).catch(async (err: any) => {
    await message(`Error creating ${_.capitalize(name.value)} due to ${err.message}`, {title: 'Echo', kind: 'info'});
  });

}
</script>

<style scoped>

</style>
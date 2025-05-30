<template>
  <div
    class="h-screen overflow-hidden bg-secondary bg-[url(cover.jpg)] bg-center-center bg-blend-multiply bg-cover bg-no-repeat bg-accent/60 text-accent-secondary relative flex flex-col justify-center items-center px-4"
  >
    <ViewColumnsIcon class="size-40 mb-5 hidden" />

    <h2 class="text-xl font-bold leading-10 prose-xl">
      No audio book has been created
    </h2>
    <small class="text-center mb-2"
      >Your audio books will appear as soon as you begin to add them</small
    >
    <button
      class="bg-accent-secondary text-accent btn-lg inline-flex gap-x-2 rounded items-center px-8 py-2 mt-2 cursor-pointer shadow-md transition-colors duration-200 ease-linear hover:opacity-95 hover:scale-95 control"
      @click="$emit('createNewBook')"
    >
      <PlusCircleIcon class="size-8" @click="invoke('synthesize_audio')" />
      Create
    </button>
  </div>
</template>

<script lang="ts" setup>
import { PlusCircleIcon, ViewColumnsIcon } from "@heroicons/vue/24/solid";

defineEmits(["createNewBook"]);
</script>
<style scoped></style>

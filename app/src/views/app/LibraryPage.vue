<template>
  <template v-if="empty">
    <div
        class="overflow-hidden bg-center-center bg-blend-multiply bg-cover bg-no-repeat relative flex flex-col justify-center items-center px-4 h-[90%] text-gray-400"
    >
      <Icon
          icon="mdi:book-plus-outline"
          class="text-gray-400/50 size-36 flex items-center cursor-pointer"
      />
      <h6>No audio book has been created</h6>
    </div>
  </template>

  <template v-else>
    <div class="flex parent-element flex-col gap-y-4 mt-12">
      <!-- <AudioBook
          class=""
          v-for="(book, index) in audioBooks"
          :key="index"
          :file-name="String(book.title)"
          duration="book.playBackDuration.toString()"
          :date-last-played="new Date().toLocaleDateString()"
      /> -->
    </div>
  </template>
</template>

<script lang="ts" setup>
import { Icon } from "@iconify/vue";
import { ref } from "vue";
// import { useAudioBookLibrary } from "../../stores/library";

// const musicStore = useAudioBookLibrary();
// const audioBooks = musicStore.audioLibrary.audioBooks;
const empty = ref(false);
</script>

<template>
  <div class="flex parent-element flex-col gap-y-4 mt-12">
    <AudioBook
      class=""
      v-for="(book, index) in audioBooks"
      :key="index"
      :file-name="book.fileName"
      :duration="book.playBackDuration.toString()"
      :date-last-played="new Date().toLocaleDateString()"
    />
  </div>
</template>

<script lang="ts" setup>
import AudioBook from "../components/AudioBook.vue";
import { useMusicLibary } from "../stores/library";
const musicStore = useMusicLibary();
const audioBooks = musicStore.audioLibrary.audioBooks;
</script>

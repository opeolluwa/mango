<template>
  <div class="flex justify-between min-h-20">
    <Icon
      icon="fluent:chevron-left-32-filled"
      :class="['icon size-5 dark:text-white/90']"
      @click="goBack"
    />

    <Icon icon="tabler:dots" :class="['icon size-5 dark:text-white/90']" />
  </div>

  <div class="">
    <h3 class="font-3xl text-gray-400 font-medium mb-2">Recently added</h3>

    <div class="hiden">
      <AudioBook
        v-for="book in audiobooks"
        :key="book.id"
        :title="book.title"
        :author="book.author"
        :cover="book.cover"
        class="hover:shadow-md transition-shadow cursor-pointer"
      />
    </div>
  </div>
</template>

<script lang="ts" setup>
import { Icon } from "@iconify/vue";
import { ref } from "vue";
import { goBack } from "../../composibles/router";
import AudioBook from "../../components/AudioBook.vue";

const audiobooks = ref([
  {
    id: 1,
    title: "The Great Gatsby",
    author: "F. Scott Fitzgerald",
    cover: "/02.jpg",
  },
  {
    id: 2,
    title: "To Kill a Mockingbird",
    author: "Harper Lee",
    cover: "/03.jpg",
  },

  {
    id: 4,
    title: "The Hobbit",
    author: "J.R.R. Tolkien",
    cover: "/04.jpg",
  },

  {
    id: 4,
    title: "The Hobbit",
    author: "J.R.R. Tolkien",
    cover: "/05.jpg",
  },

  {
    id: 4,
    title: "The Hobbit",
    author: "J.R.R. Tolkien",
    cover: "/06.jpg",
  },
]);
</script>

<style scoped></style>

<template>
  <ColumnLayout>
    <div class="grid grid-cols-12 h-48 mt-16 gap-4">
      <div
        class="rounded-lg shadow shadow-app-gray py-6 bg-linear-50 from-app-orange/90 from-20% via-app-gray to-app-dark col-span-4 h-full px-4"
      >
        <small class="text-[12px] text-gray-400"
          >69 tracks . 4 hours 37 minutes</small
        >
        <h2 class="text-leadning-loose prose-h1:first-letter:capitalize">
          Playlist for the day
        </h2>
      </div>

      <div class="rounded py-6 bg-app-gray col-span-8 h-full px-4">
        <h2>Playlist fo the day</h2>
      </div>
    </div>

    <div>
      <JustifiedRowLayout class="flex justify-between items-center">
        <h2 class="text-app-orange font-bold leaidn-loose">Recently played</h2>
        <small>View all</small>
      </JustifiedRowLayout>
      <div class="flex flex-col gap-y-6 mt-8">
        <AudioBook
          file-name="test-file.pdf"
          duration="3:45/1:00:34"
          class
          date-last-played="June 2 2025"
        />
        <AudioBook
          file-name="test-file.pdf"
          duration="3:45/1:00:34"
          class
          date-last-played="June 2 2025"
        />
        <AudioBook
          file-name="test-file.pdf"
          duration="3:45/1:00:34"
          class
          date-last-played="June 2 2025"
        />
        <AudioBook
          file-name="test-file.pdf"
          duration="3:45/1:00:34"
          class
          date-last-played="June 2 2025"
        />
        <AudioBook
          file-name="test-file.pdf"
          duration="3:45/1:00:34"
          class
          date-last-played="June 2 2025"
        />
      </div>
    </div>
  </ColumnLayout>
</template>

<script lang="ts" setup>
import { Icon } from "@iconify/vue";
import ColumnLayout from "../components/layouts/ColumnLayout.vue";
import JustifiedRowLayout from "../components/layouts/JustifiedRowLayout.vue";
import AudioBook from "../components/AudioBook.vue";
</script>

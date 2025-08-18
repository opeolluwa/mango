<template>
  <SectionLabel label="Actor" class="mb-2" />
  <div class="grid grid-cols-12 gap-x-3">
    <div
      v-for="actor in actors"
      :key="actor.name"
      class="col-span-3 flex flex-col items-center justify-center"
    >
      <img
        :src="actor.image"
        :alt="actor.name"
        class="rounded-xl"
        :class="{ 'grayscale-75': !actor.selected }"
      />
      <span class="text-stone-500 mt-1">{{ actor.name }}</span>
    </div>
  </div>
</template>

<script lang="ts" setup>
import alexImage from '@/assets/alex.png';
import aliceImage from '@/assets/alice.png';
import charlotteImage from '@/assets/charlotte.png';
import paulImage from '@/assets/Paul.png';
import SectionLabel from '@/components/settings/SectionLabel.vue';
import { ref } from 'vue';

// Define actor data using a ref for reactivity
const actors = ref([
  { name: 'Alex', image: alexImage, selected: true },
  { name: 'Alice', image: aliceImage, selected: false },
  { name: 'Charlotte', image: charlotteImage, selected: false },
  { name: 'Paul', image: paulImage, selected: false },
]);
</script>
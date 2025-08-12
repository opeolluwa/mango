<template>
  <div @click="swap != swap">
    <div v-show="swap" id="initial">
      <slot name="initial" />
    </div>
    <div v-show="!swap" id="final">
      <slot name="final" />
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ref } from "vue";

const swap = ref(true);
</script>

<template>
  <div @click="swap != swap">
    <div id="initial" v-show="swap">
      <slot name="initial" />
    </div>
    <div id="final" v-show="!swap">
      <slot name="final" />
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ref } from "vue";

const swap = ref(true);
</script>

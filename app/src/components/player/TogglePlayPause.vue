<template>
  <Icon icon="fluent:play-48-filled" class="icon" v-show="!isPlaying" />
  <Icon icon="fluent:pause-48-filled" class="icon p-[2px]" v-show="isPlaying" />
</template>

<script lang="ts" setup>
import { Icon } from "@iconify/vue";

defineProps({
  isPlaying: Boolean,
});
</script>

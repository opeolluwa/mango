<template>
  <Icon v-show="!isPlaying" icon="fluent:play-48-filled" class="icon" />
  <Icon v-show="isPlaying" icon="fluent:pause-48-filled" class="icon p-[2px]" />
</template>

<script lang="ts" setup>
import { Icon } from "@iconify/vue";

defineProps({
  isPlaying: Boolean,
});
</script>

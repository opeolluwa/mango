<template>
  <div class="flex justify-between min-h-20">
    <Icon
      icon="fluent:chevron-left-32-filled"
      :class="['icon size-5 dark:text-white/90']"
      @click="useGoBack"
    />

    <Icon icon="tabler:dots" :class="['icon size-5 dark:text-white/90']" />
  </div>
</template>

<script lang="ts" setup>
import { Icon } from "@iconify/vue";
import { useGoBack } from "../../composibles/router.ts";
</script>

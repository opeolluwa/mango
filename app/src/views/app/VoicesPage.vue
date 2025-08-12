<template>
  <div class="flex justify-between mb-10 pb-3">
    <Icon
      icon="fluent:chevron-left-32-filled"
      :class="['icon size-5 dark:text-white/90']"
      @click="goBack"
    />

    <Icon icon="tabler:dots" :class="['icon size-5 dark:text-white/90']" />
  </div>
</template>

<script lang="ts" setup>
import { Icon } from "@iconify/vue";
import { goBack } from "../../composibles/router.ts";
</script>

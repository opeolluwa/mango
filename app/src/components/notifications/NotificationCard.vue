<template>
  <div
    class="flex justify-between py-3 border-b px-2 border-stone-200 dark:border-stone-700 relative rounded"
    :class="[isRead ? '' : 'bg-app-orange-50/10']"
  >
    <span
      v-if="!isRead"
      class="absolute right-0 size-2 rounded-full top-0"
      :class="[isRead ? '' : 'bg-app-orange-400']"
    ></span>
    <div class="flex items-center gap-x-4 relative">
      <UIcon
        name="i-lucide-check-check"
        class="size-5"
        :class="[isRead ? 'text-gray-400' : 'text-app-orange-300']"
      />

      <div class="flex flex-col gap-y-1">
        <div class="text dark:text-white/70">
          <span class="font-medium">{{ title }}</span>
        </div>
        <small class="text-gray-600"> {{ message }}</small>
      </div>
    </div>

    <div class="flex flex-col justify-end gap-y-2">
      <small class="text-gray-600"> {{ timeAgo }}</small>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { useTimeAgo } from "@vueuse/core";
import { NotificationProps } from ".";

const props = defineProps<NotificationProps>();

const { message, createdAt, title, isRead } = props;

const timeAgo = useTimeAgo(new Date(createdAt));
</script>

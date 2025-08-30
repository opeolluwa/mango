<template>
  <div
    class="flex justify-between py-3 px-2 relative rounded overflow-x-hidden"
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
          <span class="font-medium">{{ subject }}</span>
        </div>
        <div class="text-gray-600 overflow-hidden text-clip text-[12px]">
          {{ body }}
        </div>
      </div>
    </div>

    <div class="flex flex-col justify-end gap-y-2">
      <small class="text-gray-600 text-[12px]"> {{ timeAgo }}</small>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { useTimeAgo } from "@vueuse/core";
import { Notification } from "../../types/notification";

const props = defineProps<Notification>();

const { subject, isRead, createdAt, body } = props;

const timeAgo = useTimeAgo(new Date(createdAt));
</script>

<template>
  <div class="flex items-center gap-x-4 relative">
    <AvatarRoot
      :class="[
        'bg-blackA3 inline-flex size-16 select-none items-center justify-center overflow-hidden rounded-full align-middle',
      ]"
    >
      <AvatarImage
        class="h-full w-full rounded-[inherit] object-cover"
        src="https://images.unsplash.com/photo-1492633423870-43d1cd2775eb?&w=128&h=128&dpr=2&q=80"
        alt="Colm Tuite"
      />
      <AvatarFallback
        class="text-grass11 dark:text-stone-300 leading-1 flex h-full w-full items-center justify-center bg-white dark:bg-stone-800 text-sm font-medium"
        :delay-ms="600"
      >
        CT
      </AvatarFallback>

      <span v-if="editable"
        class="text-xs absolute -bottom-1.5 -right-1.5 bg-white/80 p-1 rounded-full text-app-orange-500"
      >
        <Icon icon="tabler:edit" class="size-4 text-app" />
      </span>
    </AvatarRoot>

    <div v-if="showText" class="flex flex-col">
      <div class="text-2xl font-bold">Adeoye Adefemi</div>
      <p>adefemiadeoye@yahoo.com</p>
      <span class="text-sm font-bold">Basic plan</span>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { Icon } from "@iconify/vue";
import { AvatarFallback, AvatarImage, AvatarRoot } from "reka-ui";
import { toRefs } from "vue";

const props = defineProps({
  showText: { type: Boolean, default: true },
  avatarSize: { type: Number, default: 16 },
  editable: { type: Boolean, default: false },
});

const { showText, avatarSize, editable } = toRefs(props);
</script>

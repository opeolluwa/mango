<template>
  <div class="flex items-center gap-x-4 relative">
    <AvatarRoot
      :class="[
        'bg-blackA3 inline-flex size-16 select-none items-center justify-center overflow-hidden rounded-full align-middle',
      ]"
      :style="{ width: `${avatarSize}px`, height: `${avatarSize}px` }"
    >
      <AvatarImage
        class="h-full w-full rounded-[inherit] object-cover"
        :src="userInformation.profilePicture"
        alt="Colm Tuite"
      />
      <AvatarFallback
        class="text-grass11 dark:text-stone-300 leading-1 flex h-full w-full items-center justify-center bg-app-orange-50 dark:bg-stone-800 text-sm font-medium"
      
      >
        {{ initials }}
      </AvatarFallback>

      <span
        v-if="editable"
        class="text-xs absolute -bottom-1.5 -right-1.5 bg-white/80 p-1 rounded-full text-app-orange-500"
      >
        <Icon icon="tabler:edit" class="size-4 text-app" />
      </span>
    </AvatarRoot>

    <div v-if="showText" class="flex flex-col">
      <div class="text-2xl font-bold dark:text-white/70">
        {{ fullName }}
      </div>
      <p class="text-gray-400 font-medium">{{ userInformation.email }}</p>
      <span class="text-sm font-semibold text-gray-600">Basic plan</span>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { Icon } from "@iconify/vue";
import { AvatarFallback, AvatarImage, AvatarRoot } from "reka-ui";
import { computed, toRefs } from "vue";
import { useUserInformationStore } from "../../stores/user";

const props = defineProps({
  showText: { type: Boolean, default: true },
  avatarSize: { type: Number, default: 75 },
  editable: { type: Boolean, default: false },
});

const { showText, editable, avatarSize } = toRefs(props);

const store = useUserInformationStore();
const userInformation = computed(() => store.user);
const fullName = computed(() => store.fullName);
const initials = computed(() =>
  store.user.firstName[0].concat(store.user.lastName[0])
);
</script>

<template>
  <Transition name="animate__slideInLeft">
    <AppNavigation v-show="showSideNav" @click="showSideNav = false" />
  </Transition>

  <div
    class="flex gap-4 min-h-12 items-center justify-between absolute top-0 left-0 px-4 w-screen py-3 bg-white z-500 shadow-sm dark:bg-app-dark border-gray-100/50 backdrop-blur-md pt-12"
  >
    <template v-if="route.meta.isHome">
      <div
        class="flex gap-x-2 justify-between items-center align-center w-full"
      >
        <Icon
          icon="material-symbols:menu-rounded"
          :class="['size-6 dark:text-white/90']"
          @click="toggleNav"
        />
        <div class="flex gap-x-2 items-center justify-center">
          <RouterLink :to="{ name: '' }">
            <UChip
              size="3xl"
              :color="{'error': Number(unreadNotification) >= 1}"
              :text="unreadNotification"
            >
              <UButton
                icon="i-lucide-bell"
                color=""
                variant="subtle"
                @click="usePush({ name: 'Notification' })"
              />
            </UChip>
          </RouterLink>
          <UserCard
            :editable="false"
            :avatar-size="30"
            :show-text="false"
            @click="usePush({ name: 'Settings' })"
          />
        </div>
      </div>
    </template>

    <template v-else>
      <div class="flex gap-x-2 align-center">
        <Icon
          icon="fluent:chevron-left-32-filled"
          :class="['icon size-5 dark:text-white/90']"
          @click="useGoToPreviousRoute"
        />
        <span class="font-medium"> {{ label }}</span>
      </div>
    </template>
    <slot name="headerIcon" />
  </div>

  <div class="mt-12 min-h-screen" @click="showSideNav = false">
    <slot />
  </div>
</template>

<script lang="ts" setup>
import { onMounted, ref, watch } from "vue";

import { Icon } from "@iconify/vue";
import { useRoute } from "vue-router";
import UserCard from "@components/settings/UserCard.vue";
import AppNavigation from "@components/uiBlocks/AppNavigation.vue";
import { useGoToPreviousRoute, usePush } from "../../composibles/useRouter";
import { useNotificationStore } from "../../stores/notification";

const route = useRoute();
const label = ref<string>("") || "";
const showSideNav = ref(false);

const notificationStore = useNotificationStore();
const unreadNotification = ref<number>();

const toggleNav = () => {
  showSideNav.value = !showSideNav.value;
};
watch(
  () => route.meta.label,
  (newLabel) => {
    label.value = String(newLabel ?? "");
  },
  { immediate: true }
);

onMounted(async () => {
  await notificationStore.countUnreadNotification().then(() => {
    unreadNotification.value = notificationStore.unreadCount;
  });
});
defineEmits(["close"]);
</script>

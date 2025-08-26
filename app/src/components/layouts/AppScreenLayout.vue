<template>
  <AppNavigation
    class="fixed left-0 bottom-16 w-[70vw] z-[6000] dark:bg-app-dark bg-white hidden"
  />

  <div
    class="flex gap-4 min-h-12 items-center justify-between absolute top-0 left-0 px-4 w-screen py-3 bg-white z-500 shadow-sm dark:bg-app-dark border-gray-100/50 backdrop-blur-md pt-12"
  >
    <template v-if="route.meta.isHome">
      <div
        class="flex gap-x-2 justify-between items-center align-center w-full"
      >
        <USlideover
          :close="{
            color: 'primary',
            variant: 'outline',
            class: 'rounded-full',
          }"
        >
          <Icon
            icon="material-symbols:menu-rounded"
            :class="['size-6 dark:text-white/90']"
          />

          <template #content>
            <AppNavigation />
          </template>
        </USlideover>

        <!-- <Icon
          icon="material-symbols:menu-rounded"
          :class="['size-6 dark:text-white/90']"
          @click="showSideNav != showSideNav"
        /> -->
        <div class="flex gap-x-2 items-center justify-center">
          <RouterLink :to="{ name: '' }">
            <UChip
              size="3xl"
              color="error"
              text="5"

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

  <div class="mt-12 min-h-screen">
    <slot />
  </div>
</template>

<script lang="ts" setup>
import { ref, watch } from "vue";

import { Icon } from "@iconify/vue";
import { useRoute } from "vue-router";
import UserCard from "@components/settings/UserCard.vue";
import AppNavigation from "@components/uiBlocks/AppNavigation.vue";
import { useGoToPreviousRoute, usePush } from "../../composibles/useRouter";

const route = useRoute();
const label = ref<string>("") || "";
// const showSideNa÷v = ref(false);

watch(
  () => route.meta.label,
  (newLabel) => {
    label.value = String(newLabel ?? "");
  },
  { immediate: true }
);
</script>

<template>
  <UserCard :editable="false" :show-text="true" />

  <div class="mt-12">
    <h2 class="text-gray-400 font-medium text-xl">Preferences</h2>

    <div
      v-for="(section, index) in sections"
      :key="index"
      class="my-2 py-3 first:mt-0 last:mb-0"
    >
      <component
        :is="section.routeName ? 'RouterLink' : 'div'"
        :to="section.routeName ? { name: section.routeName } : null"
        class="grid grid-cols-12 items-center gap-x-4 cursor-pointer"
        @click="section.isLogout ? logOut() : null"
      >
        <Icon
          :icon="section.icon"
          class="size-6 dark:text-white/90 col-span-2"
        />
        <div class="col-span-8">
          <h2 class="font-medium capitalize">
            {{ section.title }}
          </h2>
          <small class="text-gray-400 first-letter:capitalize">
            {{ section.description }}
          </small>
        </div>
        <Icon
          icon="fluent:chevron-right-32-filled"
          class="icon size-5 dark:text-white/90 col-span-2"
        />
      </component>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { Icon } from "@iconify/vue";
import { useRouter } from "vue-router";
import UserCard from "@/components/settings/UserCard.vue";

interface Section {
  icon: string;
  title: string;
  description: string;
  routeName?: string;
  isLogout?: boolean;
}

const sections: Section[] = [
  {
    title: "user profile",
    description: "email, name, phone",
    icon: "mi:user",
    routeName: "UserProfile",
  },
  {
    title: "security & privacy",
    description: "password, 2FA, sharing and privacy",
    icon: "mingcute:safe-lock-line",
    routeName: "SecurityAndPrivacy",
  },
  {
    title: "payment",
    description: "subscription, billing, cards",
    icon: "wpf:bank-cards",
    routeName: "PaymentAndSubscription",
  },
  {
    title: "help & support",
    description: "AI actor, customer support",
    icon: "famicons:help-circle-outline",
    routeName: "HelpAndSupport",
  },
  {
    title: "Language & Internationalization",
    description: "App language, AI agent language",
    icon: "heroicons:language-16-solid",
    routeName: "HelpAndSupport",
  },
  {
    title: "log out",
    description: "",
    icon: "material-symbols:logout",
    isLogout: true, // New flag to identify the logout section
  },
];

const router = useRouter();

const logOut = () => {
  // Add actual logout logic here (e.g., clearing tokens, etc.)
  router.push({ name: "Login" });
};
</script>

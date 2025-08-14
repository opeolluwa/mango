<template>
  <ScreenLayout>



  <div class="flex items-center gap-x-4">
    <AvatarRoot
      class="bg-blackA3 inline-flex size-16 select-none items-center justify-center overflow-hidden rounded-full align-middle"
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
    </AvatarRoot>
    <div class="flex flex-col">
      <div class="text-2xl font-bold">Adeoye Adefemi</div>
      <p>adefemiadeoye@yahoo.com</p>
      <span class="text-sm text-bold">Basic plan </span>
    </div>
  </div>

  <div class="mt-12">
    <h2 class="font-3xl text-gray-400 font-medium">Preferences</h2>
    <div
      v-for="(section, index) in sections"
      :key="index"
      class="my-2 py-3 first:mt-0 last:mb-0"
    >
      <div class="grid gap-x-4 grid-cols-12">
        <Icon
          :icon="section.icon"
          :class="[' size-6 dark:text-white/90 col-span-2']"
        />
        <div class="col-span-8">
          <h2 class="font-3xl font-medium capitalize leading-loose">
            {{ section.title }}
          </h2>
          <small class="text-gray-400 first-letter:capitalize">{{
            section.description
          }}</small>
        </div>
        <Icon
          icon="fluent:chevron-right-32-filled"
          :class="['icon size-5 dark:text-white/90 col-span-2']"
        />
      </div>
    </div>
  </div>
    </ScreenLayout>
</template>

<script lang="ts" setup>
import { Icon } from "@iconify/vue";
import { AvatarFallback, AvatarImage, AvatarRoot } from "reka-ui";
import ScreenLayout from "./ScreenLayout.vue";
interface Section {
  icon: string;
  title: string;
  description: string;
}

const sections: Section[] = [
  {
    title: "user profile",
    description: "email, name, phone",
    icon: "mi:user",
  },
  {
    title: "security and privacy",
    description: "password, 2FA, sharing and privacy",
    icon: "mingcute:safe-lock-line",
  },

  {
    title: "payment",
    description: "subscription, billing, cards",
    icon: "wpf:bank-cards",
  },
  {
    title: "help and support",
    description: "for any question, contact us",
    icon: "famicons:help-circle-outline",
  },
  {
    title: "log out",
    description: "",
    icon: "material-symbols:logout",
  },
];
</script>

<template>
  <SectionLabel label="Language" />
  <LabledIcon />

  <SectionLabel label="Actor" />
<div class="flex flex-wrap"></div>
</template>

<script lang="ts" setup>
import LabledIcon from "../../../components/settings/LabledIcon.vue";
import SectionLabel from "../../../components/settings/SectionLabel.vue";

interface Section {
  icon: string;
  title: string;
  description: string;
  routeName?: string;
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
];
</script>

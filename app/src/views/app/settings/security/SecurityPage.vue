<template>
  <div class="flex flex-col gap-y-4">
    <SectionLabel label="Security Status" />
    <div class="flex justify-between items-center">
      <LabeledIcon
        title="Two-Factor Authentication"
        description="Require OTP to login"
        icon="tabler:key"
      />
      <USwitch
        v-model="value"
        class=""
        unchecked-icon="i-lucide-x"
        checked-icon="i-lucide-check"
        default-value
        loading-icon="i-lucide-loader"
      />
    </div>
    <div class="flex justify-between items-center">
      <LabeledIcon
        title="Biometrics"
        description="use  figerprint for app  login"
        icon="material-symbols:fingerprint"
      />
      <USwitch
        v-model="value"
        class=""
        unchecked-icon="i-lucide-x"
        checked-icon="i-lucide-check"
        default-value
        loading-icon="i-lucide-loader"
      />
    </div>
  </div>

  <div class="my-8">
    <SectionLabel label="Login & Passcode" />
    <div class="flex items-center">
      <LabeledIcon
        title="Change password"
        description="securly update your password "
        icon="uil:padlock"
        route="UpdatePassword"
      />
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ref } from "vue";
import SectionLabel from "../../../../components/settings/SectionLabel.vue";
import LabeledIcon from "../../../../components/settings/LabledIcon.vue";

const value = ref(true);
</script>

<template>
  <UModal :close="{ onClick: () => emit('close', false), class: 'hidden' }">
    <template #body>
      <UAlert
        :color="kind"
        :title="title"
        variant="soft"
        description="You can change the primary color in your app config."
        icon="i-lucide-terminal"
      />
    </template>
  </UModal>
</template>

<script lang="ts" setup>
import { AppModalInterface } from "../../composibles/useModal";

defineProps<AppModalInterface>();
const emit = defineEmits<{ close: [boolean] }>();
</script>

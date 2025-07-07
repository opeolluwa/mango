<template>
  <Label class="font-medium text-stone-500 leading-[35px]" :for="for">
    {{ text }}
  </Label>
</template>

<script lang="ts" setup>
import { Label } from "reka-ui";

defineProps(["text", "for"]);
</script>

<style scoped></style>

<template>
  <Label class="font-medium text-stone-500 leading-[35px]" >
    {{ text }}
  </Label>
</template>

<script lang="ts" setup>
import { Label } from "reka-ui";

const props = defineProps(["text", "for"]);
 const {text, } = props
</script>

<style scoped></style>

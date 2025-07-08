<template>
  <RouterView v-slot="{ Component }">
    <Transition name="fade" mode="out-in">
      <component :is="Component" />
    </Transition>
  </RouterView>
</template>

<script lang="ts" setup></script>

<style>
h1 {
  font-weight: 400;
  line-height: 45px;
}

h1 + p {
  margin-top: 10px;
}


h1.text-7xl {
  line-height: 1;
}
</style>

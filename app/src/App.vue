<template>
  <div class="w-full">
    <UApp>
      <router-view v-slot="{ Component }">
        <transition name="fade">
          <component :is="Component" />
        </transition>
      </router-view>
    </UApp>
  </div>
</template>

<script lang="ts" setup></script>

<style scoped></style>

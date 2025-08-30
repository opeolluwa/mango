<template>
  <AppScreenLayout>
    <div class="flex flex-col gap-y-6">
      <NotificationCard
        v-for="(data, index) in notifications"
        :key="index"
        :identifier="data.identifier"
        :is-read="data.isRead"
        :timestamp="data.createdAt"
        :created-at="data.createdAt"
        :subject="data.subject"
        :body="data.body"
        :updated-at="data.updatedAt"
        :user-identifier="data.userIdentifier"
      />
    </div>
  </AppScreenLayout>
</template>

<script lang="ts" setup>
import { onMounted, ref } from "vue";
import NotificationCard from "../../components/notifications/NotificationCard.vue";
import { useNotificationStore } from "../../stores/notification";
import { Notification } from "../../types/notification";

const notificationStore = useNotificationStore();
const notifications = ref<Notification[]>();

onMounted(async () => {
  await notificationStore.initialize().then(() => {
    notifications.value = notificationStore.notifications;
  });
});
</script>

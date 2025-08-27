<template>
  <AppScreenLayout>
    <div class="flex flex-col gap-y-6">
      <NotificationCard
        v-for="(data, index) in notifications"
        :key="index"
        :identifier="data.identifier"
        :is-read="data.isRead"
        :message="data.message"
        :timestamp="data.createdAt"
        :title="data.title"
        :type="data.type"
        :created-at="data.createdAt"
      />
    </div>
  </AppScreenLayout>
</template>

<script lang="ts" setup>
import { NotificationProps } from "../../components/notifications";
import NotificationCard from "../../components/notifications/NotificationCard.vue";

const notifications: NotificationProps[] = [
  {
    identifier: "550e8400-e29b-41d4-a716-446655440000",
    type: "info",
    title: "Profile Updated",
    message: "Your profile has been updated successfully.",
    isRead: false,
    createdAt: "2025-08-25T10:15:00Z",
  },
  {
    identifier: "123e4567-e89b-12d3-a456-426614174000",
    type: "warning",
    title: "Password Expiring Soon",
    message: "Your password will expire in 3 days.",
    isRead: false,
    createdAt: "2025-08-24T08:42:00Z",
  },
  {
    identifier: "9f8c5d1e-7a41-4e4b-b8f7-4c5c6f6a7d9b",
    type: "error",
    title: "Payment Failed",
    message: "Payment failed. Please try again.",
    isRead: false,
    createdAt: "2025-08-26T18:20:00Z",
  },
  {
    identifier: "f47ac10b-58cc-4372-a567-0e02b2c3d479",
    type: "success",
    title: "Booking Confirmed",
    message: "Your booking has been confirmed 🎉",
    isRead: true,
    createdAt: "2025-08-23T14:05:00Z",
  },
];
</script>

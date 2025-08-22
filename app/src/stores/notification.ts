import { defineStore } from "pinia";
import { Notification } from "../types/notification";

interface Store {
  notifications: Notification[];
}
export const useNotificationStore = defineStore("app_notification", {
  state: (): Store => ({
    notifications: [],
  }),
  getters: {
    unread: (state): Notification[] =>
      state.notifications.filter((entry) => entry.isRead != true),
    unreadCount: (state): number =>
      state.notifications.filter((entry) => entry.isRead != true).length,
  },
  actions: {
    async markRead(notificationIdentifier: string) {},
    async listenForUpdates(userIdentifier: string) {},
    async initialize() {},
    async delete() {},
  },
  persist: true,
});

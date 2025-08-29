import { defineStore } from "pinia";
import { Notification } from "../types/notification";
import { useTokenStore } from "./token";
import api from "../plugins/api";

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
    async markRead() {},
    async listenForUpdates() {},
    async initialize() {
      const response = await  api.get("/notifications");
      console.log(response);
    },
    async delete() {},
  },
  persist: true,
});

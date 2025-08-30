import { defineStore } from "pinia";
import api from "../plugins/api";
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
    async markRead(identifier: string):Promise<void> {
      try {
        await api.patch(`/notifications/${identifier}`)
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
      } catch (error: any) {
        console.log(error)
      }
    },
    async listenForUpdates() {},
    async initialize() {
      try {
        const response = await api.get("/notifications?page=1&per_page=10");
        console.log(response.data.data);
        this.$patch({ notifications: response.data.data.records });

        // eslint-disable-next-line @typescript-eslint/no-explicit-any
      } catch (error: any) {
        console.log(error);
      }
    },
    async delete() {},
  },
  persist: true,
});

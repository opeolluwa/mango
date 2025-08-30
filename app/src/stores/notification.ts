import { defineStore } from "pinia";
import api from "../plugins/api";
import { Notification } from "../types/notification";

interface Store {
  notifications: Notification[];
  unreadCount: number;
}
export const useNotificationStore = defineStore("app_notification", {
  state: (): Store => ({
    notifications: [],
    unreadCount: 0,
  }),
  getters: {
    unread: (state): Notification[] =>
      state.notifications.filter((entry) => entry.isRead != true),
  },
  actions: {
    async markRead(identifier: string): Promise<void> {
      try {
        await api.patch(`/notifications/${identifier}`);

        // eslint-disable-next-line @typescript-eslint/no-explicit-any
      } catch (error: any) {
        console.log(error);
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

    async countUnreadNotification() {
      const response = await api.get("/notifications/unread");
      console.log(response.data.data);
      this.$patch({ unreadCount: response.data.data.count });
    },
    async delete() {},
    async refresh() {
      await this.initialize();
    },
  },
  persist: true,
});

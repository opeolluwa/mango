import { defineStore } from "pinia";


export const useTokenStore = defineStore("token_store", {
  state: () => ({
    accessToken: "",
    refreshToken: "",
    requestToken: "",
  }),

  actions: {
    persistRequestToken(requestToken: string) {
      this.$patch({ requestToken: requestToken });
    },
    persistRefreshToken(refreshToken: string) {
      this.$patch({ refreshToken: refreshToken });
    },

    persistAccessToken(accessToken: string) {
      this.$patch({ accessToken: accessToken });
    },
  },
  getters: {},
  persist: true,
});

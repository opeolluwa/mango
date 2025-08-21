import { defineStore } from "pinia";
import { load } from "@tauri-apps/plugin-store";

const TokenStore = await load("store.json", {
  autoSave: true,
  defaults: {},
});

export const useTokenStore = defineStore("token_store", {
  state: () => ({
    accessToken: "",
    refreshToken: "",
    requestToken: "",
  }),

  actions: {
    async saveRequestToken(requestToken: string) {
      this.refreshToken = requestToken;
      await TokenStore.set("requestToken", this.requestToken);
    },
    async saveAuthToken({
      accessToken,
      refreshToken,
    }: {
      accessToken: string;
      refreshToken: string;
    }) {
      this.accessToken = accessToken;
      this.refreshToken = refreshToken;
      await TokenStore.set("accessToken", this.accessToken);
      await TokenStore.set("refreshToken", this.refreshToken);
    },

    async extractAccessToken(): Promise<string | undefined> {
      return this.accessToken || (await TokenStore.get("accessToken"));
    },
    async extractRequestToken(): Promise<string | undefined> {
      return this.requestToken || (await TokenStore.get("requestToken"));
    },
  },
  getters: {

  },
});

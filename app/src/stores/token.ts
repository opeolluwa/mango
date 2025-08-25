import axios from "axios";
import { defineStore } from "pinia";

export const useTokenStore = defineStore("token_store", {
  state: () => ({
    accessToken: "",
    refreshToken: "",
    requestToken: "",
    accessTokenExpiry: 0,
    refreshTokenExpiry: 0,
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
    clearTokens() {
      this.$reset();
    },
    setAccessTokenExpiry(expiry: number) {
      this.$patch({ accessTokenExpiry: expiry });
    },
    setRefreshTokenExpiry(expiry: number) {
      this.$patch({ refreshTokenExpiry: expiry });
    },
    extractAccessToken() {
      return this.accessToken;
    },
    async getRefeshToken() {
      const response = await axios.get("/auth/refresh-token", {
        headers: {
          Authorization: `Bearer ${this.refreshToken}`,
          "Content-Type": "application/json",
        },
      });

      const data = response.data.data;
      console.log("Refresh token response", data);
      if (data?.data?.accessToken) {
        this.persistAccessToken(data.accessToken);
        this.setAccessTokenExpiry(data.exp);
        this.persistRefreshToken(data.refreshToken);
        this.setRefreshTokenExpiry(data.refreshTokenExp);
      }
    },

    isAccessTokenValid() {
      const currentTime = Math.floor(Date.now() / 1000); // Current time in seconds
      return this.accessToken && this.accessTokenExpiry > currentTime + 60; // Consider valid if expires in more than 60 seconds
    },
  },
  getters: {},
  persist: true,
});

import { defineStore } from "pinia";
import { UserInformation } from "../types/userProfile";
import axios from "axios";

export const useUserInformation = defineStore("user_information", {
  state: () => ({
    identifier: "",
  }),
  actions: {
    async fetchUserInformation(token: string): Promise<UserInformation> {
      try {
        const response = await axios.get("/user/profile", {
          headers: { Authorization: `Bearer ${token}` },
        });
        const userInformation = response.data.data as UserInformation;
        return userInformation;
      } catch (error) {
        throw new Error(`Failed to fetch user information due to ${error}`);
      }
    },
  },
});

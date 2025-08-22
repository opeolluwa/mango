import { defineStore } from "pinia";
import { UserInformation } from "../types/userProfile";
import axios from "axios";

export const useUserInformationStore = defineStore("user_information", {
  state: () => ({
    identifier: "",
    firstName: "",
    lastName: "",
    email: "",
    profilePicture: "",
  }),

  getters: {
    user: (state): UserInformation => ({
      identifier: state.identifier,
      firstName: state.firstName,
      lastName: state.lastName,
      email: state.email,
      profilePicture: state.profilePicture,
    }),
    fullName: (state) => `${state.firstName} ${state.lastName}`,
    userFirstName: (state) => state.firstName,
  },
  actions: {
    async initialize(token: string): Promise<UserInformation> {
      const userInformation = await this.fetchUserInformation(token);
      this.$patch((state) => {
        state.identifier = userInformation.identifier;
        state.firstName = userInformation.firstName;
        state.lastName = userInformation.lastName;
        state.email = userInformation.email;
      });
      return userInformation;
    },
    async fetchUserInformation(token: string): Promise<UserInformation> {
      try {
        const response = await axios.get("/user/profile", {
          headers: { Authorization: `Bearer ${token}` },
        });
        return response.data.data as UserInformation;
      } catch (error) {
        throw new Error(`Failed to fetch user information due to ${error}`);
      }
    },
 
  },
     persist: true,
});

import { defineStore } from "pinia";
import { type CachedUser } from "../../src-tauri/bindings/CachedUser";
import { type CreateCachedUser } from "../../src-tauri/bindings/CreateCachedUser";
import { invoke } from "@tauri-apps/api/core";

type Store = CachedUser;
export type UserCache = CreateCachedUser;

export const useCachedUserStore = defineStore("cached_user", {
  state: (): Store => ({
    identifier: "",
    firstName: null,
    lastName: null,
    email: null,
    avatarUrl: null,
  }),
  getters: {
    firstName: (state) => state.firstName,
    lastName: (state) => state.lastName,
    email: (state) => state.email,
    storeIsNull: (state): boolean => {
      const entryIsFalsy = (currentEntry: string) =>
        Boolean(currentEntry) == false;
      return Object.values(state).every(entryIsFalsy);
    },
    user: (state) => ({
      identifier: state.identifier,
      firstName: state.firstName,
      lastName: state.lastName,
      email: state.email,
      avatarUrl: state.avatarUrl,
    }),
  },

  actions: {
    async cacheUserData(user: UserCache) {
      await invoke("set_cached_user", { user }).catch((error) => {
        console.log("failed to set user data cache", error);
      });
    },
  },
});

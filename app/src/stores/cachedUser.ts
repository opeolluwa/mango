import { invoke } from "@tauri-apps/api/core";
import { defineStore } from "pinia";
import { type CachedUser } from "../../src-tauri/bindings/CachedUser";
import { type CreateCachedUser } from "../../src-tauri/bindings/CreateCachedUser";

type Store = CachedUser;
export type UserCache = CreateCachedUser;

export const useCachedUserStore = defineStore("cached_user", {
  state: (): Store => ({
    identifier: "",
    firstName: "",
    lastName: "",
    email: "",
    avatarUrl: "",
  }),

  getters: {
    firstName: (state) => state.firstName,
    lastName: (state) => state.lastName,
    email: (state) => state.email,
    storeIsNull: (state): boolean => {
      const entryIsFalsy = (entry: unknown) => !entry;
      return Object.values(state).every(entryIsFalsy);
    },
    user: (state): CachedUser => ({
      identifier: state.identifier,
      firstName: state.firstName,
      lastName: state.lastName,
      email: state.email,
      avatarUrl: state.avatarUrl,
    }),
  },

  actions: {
    async cacheUserData(user: UserCache) {
      try {
        await invoke("set_cached_user", { user });
      } catch (error) {
        console.error("failed to set user data cache", error);
      }
    },

    async fetchCachedUser(): Promise<CachedUser | null> {
      try {
        const cachedUser = await invoke<CachedUser>("fetch_cached_user");
        console.log({ cachedUser });
        if (cachedUser) {
          this.$patch({
            identifier: cachedUser.identifier,
            firstName: cachedUser.firstName,
            lastName: cachedUser.lastName,
            email: cachedUser.email,
            avatarUrl: cachedUser.avatarUrl,
          });
        }
        return cachedUser;
      } catch (error) {
        console.error("failed to fetch user data cache", error);
        return null;
      }
    },
  },
});

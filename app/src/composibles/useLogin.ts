import axios from "axios";
import { useRouter } from "vue-router";
import { useCachedUserStore } from "../stores/cachedUser";
import { useUserInformation } from "../stores/user";
import type { Auth } from "../types/auth";

interface LoginResult {
  success: boolean;
  message: string;
}

export function useLogin() {
  const router = useRouter();
  const cachedUserStore = useCachedUserStore();
  const userStore = useUserInformation();

  const login = async (auth: Auth): Promise<LoginResult> => {
    try {
      const { email, password } = auth;

      const { data, status } = await axios.post(
        "/auth/login",
        { email, password },
        {
          headers: { "Content-Type": "application/json" },
        }
      );

      if (status !== 200 || !data?.data?.accessToken) {
        return { success: false, message: "Failed to login, please retry." };
      }

      const token = data.data.accessToken;
      const userInformation = await userStore.fetchUserInformation(token);

      await cachedUserStore.cacheUserData({
        email: userInformation.email,
        firstName: userInformation.firstName,
        lastName: userInformation.lastName,
        avatarUrl: userInformation.profilePicture,
      });

      // navigate if you want this composable to handle routing
      router.push({ name: "Home" });

      return { success: true, message: "Login successful" };
    } catch (error: unknown) {
      if (axios.isAxiosError(error)) {
        const message =
          error.response?.data?.message ||
          "An unexpected error occurred during login.";
        return { success: false, message };
      }
      return { success: false, message: "Something went wrong." };
    }
  };

  return { login };
}

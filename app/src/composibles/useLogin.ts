import axios from "axios";
import { useCachedUserStore } from "../stores/cachedUser";
import { useTokenStore } from "../stores/token";
import { useUserInformationStore } from "../stores/user";
import { message } from "@tauri-apps/plugin-dialog";

interface LoginRequest {
  email: string;
  password: string;
}
export const useLogin = async (auth: LoginRequest): Promise<void> => {
  const cachedUserStore = useCachedUserStore();
  const userStore = useUserInformationStore();
  const tokenStore = useTokenStore();

  const { email, password } = auth;

  try {
    const { data, status } = await axios.post(
      "/auth/login",
      { email, password },
      {
        headers: { "Content-Type": "application/json" },
      }
    );
    if (status !== 200 || !data?.data?.accessToken) {
      await message(data.data.message || "Failed to login, please retry.", {
        title: "Login failed",
        kind: "error",
      });
      return;
    }

    const token = data.data.accessToken;
    const userInformation = await userStore.initialize(token);

    await cachedUserStore.cacheUserData({
      email: userInformation.email,
      firstName: userInformation.firstName,
      lastName: userInformation.lastName,
      avatarUrl: userInformation.profilePicture,
    });

    const {
      accessToken,
      refreshToken,
      refreshTokenExp,
      exp: accessTokenExp,
    } = data.data;

    tokenStore.persistAccessToken(accessToken);
    tokenStore.persistRefreshToken(refreshToken);
    tokenStore.setAccessTokenExpiry(accessTokenExp);
    tokenStore.setRefreshTokenExpiry(refreshTokenExp);
  } catch (error) {
    if (axios.isAxiosError(error)) {
      await message(
        error.response?.data?.message ||
          "An unexpected error occurred during login.",
        { title: "Login failed", kind: "error" }
      );
      return;
    }
    await message("Something went wrong.", {
      title: "Login failed",
      kind: "error",
    });
    return;
  }
};

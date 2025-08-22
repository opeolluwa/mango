import axios from "axios";
import { useCachedUserStore } from "../stores/cachedUser";
import { useTokenStore } from "../stores/token";
import { useUserInformationStore } from "../stores/user";

interface LoginResult {
  success: boolean;
  message: string;
}

interface LoginRequest {
  email: string;
  password: string;
}
export const useLogin = async (auth: LoginRequest): Promise<LoginResult> => {
  const cachedUserStore = useCachedUserStore();
  const userStore = useUserInformationStore();
  const tokenStore = useTokenStore();

  const { email, password } = auth;
  const { data, status } = await axios.post(
    "/auth/login",
    { email, password },
    {
      headers: { "Content-Type": "application/json" },
    }
  );
  if (status !== 200 || !data?.data?.accessToken) {
    return {
      success: false,
      message: data.data.message || "Failed to login, please retry.",
    };
  }

  const token = data.data.accessToken;
  const userInformation = await userStore.initialize(token);

  await cachedUserStore.cacheUserData({
    email: userInformation.email,
    firstName: userInformation.firstName,
    lastName: userInformation.lastName,
    avatarUrl: userInformation.profilePicture,
  });

  const { accessToken, refreshToken } = data.data;
  await tokenStore.saveAuthToken({ accessToken, refreshToken });

  return { success: true, message: "Login successful" };
};

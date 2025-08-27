import axios from "axios";
import { message } from "@tauri-apps/plugin-dialog";
import router from "../router";
interface SignupPayload {
  email: string;
  password: string;
}

export const useSignup = async ({
  email,
  password,
}: SignupPayload): Promise<void> => {
  try {
    const { data, status } = await axios.post(
      "/auth/signup",
      { email, password },
      { headers: { "Content-Type": "application/json" } }
    );

    if (status !== 201 || !data?.data?.token) {
      const errorMessage = data?.data?.message || "Failed to create user";
      await message(errorMessage, { title: "Sign up failed", kind: "error" });
      return;
    }

    const token = data.data.token;

    router.push({ name: "ConfirmOtp", query: { token } });
  } catch (error: unknown) {
    if (axios.isAxiosError(error)) {
      await message(
        error.response?.data?.message ||
          "An unexpected error occurred during sign up.",
        { title: "sign up failed", kind: "error" }
      );
      return;
    }
    await message("Something went wrong.", {
      title: "Sign up failed",
      kind: "error",
    });
    return;
  }
};

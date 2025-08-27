import { message } from "@tauri-apps/plugin-dialog";
import axios from "axios";
import router from "../router";

export const useForgottenPassword = async (email: string): Promise<void> => {
  try {
    const { data, status } = await axios.post(
      "/auth/forgotten-password",
      { email },
      { headers: { "Content-Type": "application/json" } }
    );

    if (status !== 200) {
      const errorMessage = data?.data?.message || "Failed to send reset email";
      await message(errorMessage, {
        title: "Request failed",
        kind: "error",
      });
      return;
    }
    const token = data.data.token;

    router.push({ name: "VerifyAccountRecovery", query: { token } });
    return;
  } catch (error: unknown) {
    console.error(error);
    if (axios.isAxiosError(error)) {
      await message(
        error.response?.data?.message ||
          "An unexpected error occurred during the request.",
        { title: "Request failed", kind: "error" }
      );
      return;
    }
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    await message((error as any).response?.data?.message, {
      title: "Request failed",
      kind: "error",
    });
    return;
  }
};

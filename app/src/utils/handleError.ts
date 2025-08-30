import { message } from "@tauri-apps/plugin-dialog";
import axios from "axios";

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export const useErrorHandler = async (error: any) => {
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
};

import axios from "axios";
import { useRouter } from "vue-router";

interface SignupPayload {
  email: string;
  password: string;
}

interface SignupResult {
  success: boolean;
  message: string;
  token?: string;
}

export function useSignup() {
  const router = useRouter();

  const signup = async ({
    email,
    password,
  }: SignupPayload): Promise<SignupResult> => {
    try {
      const { data, status } = await axios.post(
        "/auth/signup",
        { email, password },
        { headers: { "Content-Type": "application/json" } }
      );

      if (status !== 201 || !data?.data?.token) {
        return {
          success: false,
          message: data?.message || "Failed to create user",
        };
      }

      const token = data.data.token;

      // Navigate to confirm OTP page
      router.push({ name: "ConfirmOtp", query: { token } });

      return { success: true, message: "Signup successful", token };
    } catch (error: unknown) {
      if (axios.isAxiosError(error)) {
        return {
          success: false,
          message:
            error.response?.data?.message ||
            "An unexpected error occurred during signup.",
        };
      }
      return { success: false, message: "Something went wrong." };
    }
  };

  return { signup };
}

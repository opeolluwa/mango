import api from "../plugins/api";
import { ChangePasswordRequest } from "../types/updatePassword";

export const useUpdatePassword = async ({
  confirmPassword,
  currentPassword,
  newPassword,
}: ChangePasswordRequest) => {
  const response = await api.patch("/user/password", {
    confirmPassword,
    currentPassword,
    newPassword,
  });

  return { success: response.status === 200, data: response.data.data };
};

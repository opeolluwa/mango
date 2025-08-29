import api from "../plugins/api";
import { PartialUserProfile } from "../types/partialUserProfile";

export const useUpdateUserProfile = async ({
  email,
  firstName,
  lastName,
  username
}: PartialUserProfile) => {
  const response = await api.put("/user/profile", {
    email,
    firstName,
    lastName,
    username
  });

  const updatedProfile = response.data.data;

  return updatedProfile;
};

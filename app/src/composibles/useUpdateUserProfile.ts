import api from "../plugins/api";

interface ProfileUpdate {
  email?: string;
  firstName?: string;
  lastName?: string;
}

export const useUpdateUserProfile = async ({
  email,
  firstName,
  lastName,
}: ProfileUpdate) => {
  const response = await api.put("/user/profile", {
    email,
    firstName,
    lastName,
  });

  const updatedProfile = response.data.data;

  return updatedProfile;
};

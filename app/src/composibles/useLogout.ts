import { useTokenStore } from "../stores/token";
import { usePush } from "./useRouter";

export const useLogout = () => {
  const tokenStore = useTokenStore();

  tokenStore.clearTokens();
  usePush({ name: "Login" });
};

import { useRoute, useRouter } from "vue-router";

const router = useRouter();
const route = useRoute();

export const useSetToken = (routeName: string, token: string | number) => {
  router.push({ name: routeName, query: { token: token.toString() } });
};

export const useGetToken = () => {
  const token = route.query["token"] as string;
  return token;
};

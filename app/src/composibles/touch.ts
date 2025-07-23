import { useRouter } from "vue-router";

export const useSwipeLeft = (route: string) => {
  const router = useRouter();
  router.push({ name: route });
};

export const useSwipeRight = () => {
  const router = useRouter();
  router.back();
};

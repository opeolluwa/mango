import router from "../router";

export const useGoBack = () => router.go(-1);
export const useGoForward = () => router.go(+1);

export const usePush = (to: string | { name: string }) => {
  router.push(to);
};

export const useGoToPreviousRoute = () => {
  router.back();
};

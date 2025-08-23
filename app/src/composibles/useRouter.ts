import router from "../router";

export const useGoBack = () => router.go(-1);
export const useGoForward = () => router.go(+1);

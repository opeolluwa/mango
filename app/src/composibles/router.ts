import router from "../routes";

export const useGoBack = () => router.go(-1);
export const useGoForward = () => router.go(+1);

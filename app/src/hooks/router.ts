import router from "../routes";

export const goBack = () => router.go(-1);
export const goForward = () => router.go(+1);

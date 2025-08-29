import { defineStore } from "pinia";
import { type AppPersonalization } from "../types/appPersonalizaton";


export const useAppPersonalization = defineStore("app_peronalization", {
  state: (): AppPersonalization => ({
    theme: null,
    language: null,
    preferredVoice: null,
  }),
});

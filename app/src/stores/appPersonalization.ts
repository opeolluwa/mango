import { defineStore } from "pinia";
import { type AppPersonalization } from "../../src-tauri/bindings/AppPersonalization";

interface Store extends AppPersonalization {}

export const useAppPersonalization = defineStore("app_peronalization", {
  state: (): Store => ({
    theme: null,
    language: null,
    preferredVoice: null,
  }),
  
});

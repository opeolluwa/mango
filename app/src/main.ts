import "./assets/styles.css";
import "./axios.config";
// import "./notification";
import "./plugins/devtools";

import { createApp } from "vue";
import { createPinia } from "pinia";
import piniaPluginPersistedstate from "pinia-plugin-persistedstate";

import ui from "@nuxt/ui/vue-plugin";

import router from "./router";
import App from "./App.vue";
import AppScreenLayout from "@components/layouts/AppScreenLayout.vue";

const pinia = createPinia();
pinia.use(piniaPluginPersistedstate);

const app = createApp(App);

app.component("AppScreenLayout", AppScreenLayout);

app.use(pinia);
app.use(router);
app.use(ui);

app.mount("#app");

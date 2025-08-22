import "./assets/styles.css";
import "./axios.config";
import "./notification";

import "./plugins/devtools";

import { createPinia } from "pinia";
import { createApp } from "vue";

import piniaPluginPersistedstate from "pinia-plugin-persistedstate";

import ui from "@nuxt/ui/vue-plugin";
import App from "./App.vue";
import router from "./routes";

const pinia = createPinia();
pinia.use(piniaPluginPersistedstate);

const app = createApp(App);

app.use(pinia);
app.use(router);
app.use(ui);

app.mount("#app");

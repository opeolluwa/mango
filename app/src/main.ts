import "./assets/styles.css";
import 'vue-final-modal/style.css'

import "./plugins/axios";
import "./plugins/notification";
import "./plugins/devtools";

import { createApp } from "vue";
import { createPinia } from "pinia";
import piniaPluginPersistedstate from "pinia-plugin-persistedstate";
import { createVfm } from 'vue-final-modal'

import ui from "@nuxt/ui/vue-plugin";

import router from "./router";
import App from "./App.vue";
import AppScreenLayout from "@components/layouts/AppScreenLayout.vue";

const pinia = createPinia();
pinia.use(piniaPluginPersistedstate);

const vfm = createVfm()


const app = createApp(App);

app.component("AppScreenLayout", AppScreenLayout);

app.use(pinia);
app.use(router);
app.use(ui);
app.use(vfm)

app.mount("#app");

import "./assets/styles.css";
import "./axios.config";
// import "./revenueCat"

import { createPinia } from "pinia";
import { createApp } from "vue";
import App from "./App.vue";
import router from "./routes";
import ui from "@nuxt/ui/vue-plugin";

const store = createPinia();
const app = createApp(App);

app.use(store);
app.use(router);
app.use(ui);

app.mount("#app");

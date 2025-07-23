import "./assets/styles.css";
import { createPinia } from "pinia";
import { createApp } from "vue";
import App from "./App.vue";
import router from "./routes";

import Vue3TouchEvents, {
  type Vue3TouchEventsOptions,
} from "vue3-touch-events";

const store = createPinia();
const app = createApp(App);

app.use(store);
app.use(router);

app.use<Vue3TouchEventsOptions>(Vue3TouchEvents, {
  disableClick: true,
});

app.mount("#app");

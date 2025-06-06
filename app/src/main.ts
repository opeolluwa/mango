import "./assets/styles.css";
import {createPinia} from "pinia";
import {createApp} from "vue";
import PDFObjectPlugin from 'pdfobject-vue';
import App from "./App.vue";
import router from "./routes";

const store = createPinia();
const app = createApp(App);

app.use(store);
app.use(router);
app.use(PDFObjectPlugin);
app.mount("#app");

import("preline/dist/index.js");
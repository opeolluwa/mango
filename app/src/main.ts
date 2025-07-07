import "./assets/styles.css";
import { createPinia } from "pinia";
import { createApp } from "vue";
import PDFObjectPlugin from "pdfobject-vue";
import App from "./App.vue";
import router from "./routes";
import { invoke } from "@tauri-apps/api/core";

function sleep(seconds: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, seconds * 1000));
}

async function setup() {
  console.log("Performing really heavy frontend setup task...");
  await sleep(3);
  console.log("Frontend setup task complete!");
  invoke("set_complete", { task: "frontend" });
}

window.addEventListener("DOMContentLoaded", () => {
  setup();
});

const store = createPinia();
const app = createApp(App);

app.use(store);
app.use(router);
app.use(PDFObjectPlugin);
app.mount("#app");

// import("preline/dist/index.js");

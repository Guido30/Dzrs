import { createApp } from "vue";
import { appWindow } from "@tauri-apps/api/window";
import router from "./router";
import "./styles.css";
import App from "./App.vue";

const app = createApp(App);
app.use(router);
app.mount("#app");

appWindow.listen("page-change", (event) =>
  router.push({ path: event.payload })
);

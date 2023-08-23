import { createApp } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import App from "./App.vue";

const response = await invoke("get_config_values")
  .then((res) => res)
  .catch((_) => "");

const appConfig = JSON.parse(response);

const app = createApp(App);
app.provide("appConfig", appConfig);
app.mount("#app");

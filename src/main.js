import { createApp } from "vue";
import App from "./App.vue";
import FloatingVue from "floating-vue";
import "floating-vue/dist/style.css";

const app = createApp(App);
app.use(FloatingVue);
app.mount("#app");

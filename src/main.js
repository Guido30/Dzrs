import { createApp } from "vue";
import FloatingVue from "floating-vue";
import "floating-vue/dist/style.css";
import App from "./App.vue";

const app = createApp(App);
app.use(FloatingVue);
app.mount("#app");

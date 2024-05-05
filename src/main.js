import { createApp } from "vue";
import FloatingVue from "floating-vue";
import "floating-vue/dist/style.css";
import PrimeVue from "primevue/config";
import "primevue/resources/themes/aura-dark-amber/theme.css";
import "primeicons/primeicons.css";
import App from "./App.vue";

const app = createApp(App);
app.use(FloatingVue);
app.use(PrimeVue);
app.mount("#app");

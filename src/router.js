import { createRouter, createWebHistory } from "vue-router";
import Main from "./pages/Main.vue";
import Download from "./pages/Download.vue";
import Settings from "./pages/Settings.vue";
import About from "./pages/About.vue";

const routes = [
  { path: "/", component: Main },
  { path: "/download", component: Download },
  { path: "/settings", component: Settings },
  { path: "/about", component: About },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;

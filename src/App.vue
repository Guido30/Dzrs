<script setup>
import { onMounted, shallowRef, ref } from "vue";
import { invoke } from "@tauri-apps/api";
import { appWindow } from "@tauri-apps/api/window";
import Main from "./pages/Main.vue";
import Download from "./pages/Download.vue";
import Settings from "./pages/Settings.vue";
import About from "./pages/About.vue";
import HeaderBar from "./components/HeaderBar.vue";
import Notifications from "./components/Notifications.vue";
import InstantNotifications from "./components/InstantNotifications.vue";

import { appConfig, globalEmitter } from "./globals";

const activePage = shallowRef(Main);
const showNotifications = ref(false);

appWindow.listen("page-change", (event) => {
  switch (event.payload) {
    case "Download":
      activePage.value = Download;
      break;
    case "Settings":
      activePage.value = Settings;
      break;
    case "About":
      activePage.value = About;
      break;
    default:
      activePage.value = Main;
  }
});

onMounted(async () => {
  // Prevent default browser context menu
  document.addEventListener("contextmenu", (event) => event.preventDefault());
  const path = appConfig.directoryViewPath;
  if (path) {
    await invoke("watch_dir", { dir: path }).catch((err) => globalEmitter.emit("notification-add", { type: "Error", origin: "watch_directory", msg: err }));
  }
  if (!appConfig._loaded) {
    globalEmitter.emit("notification-add", { type: "Info", origin: "Config", msg: "Config file could not be loaded! Ignore this message if its the first time running Dzrs" });
  }
  // Finally when everything is initialized, show the main window
  await appWindow.show();
});
</script>

<template>
  <div class="container" style="height: 100vh; padding: 0px">
    <HeaderBar @showNotifications="(res) => (showNotifications = res)" />
    <keep-alive>
      <component :is="activePage" />
    </keep-alive>
    <Transition>
      <Notifications v-show="showNotifications" />
    </Transition>
    <InstantNotifications />
  </div>
</template>

<style>
@import url("https://fonts.googleapis.com/css2?family=Noto+Sans:ital,wght@0,100;0,200;0,300;0,400;0,500;0,600;0,700;0,800;0,900;1,100;1,200;1,300;1,400;1,500;1,600;1,700;1,800;1,900&display=swap");

:root {
  --color-bg-1: #202020;
  --color-bg-2: #292929;
  --color-bg-3: #2c2c2c;
  --color-shadow: #20202077;
  --color-accent: #5a2a0a;
  --color-hover: #272727;
  --color-text: #ececec;

  --color-info: #3994b8;
  --color-bg-info: #1a494b;
  --color-success: #39b863;
  --color-error: #b84139;
  --color-bg-error: #5e2323;

  font-family: "Noto Sans", sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: var(--color-text);
  background-color: var(--color-bg-3);

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

::-webkit-scrollbar {
  width: 10px;
  background-color: var(--color-bg-2);
  border-radius: 5px;
}
::-webkit-scrollbar-thumb {
  background-color: var(--color-accent);
  border-radius: 5px;
}
::-webkit-scrollbar-thumb:hover {
  background-color: var(--color-accent);
}

body {
  margin: 0px;
}

.container {
  box-sizing: border-box;
  height: 100vh;
  margin: 0px;
  padding: 10px;
  display: flex;
  flex-direction: column;
  justify-content: start;
  text-align: center;
}

.row {
  display: flex;
  justify-content: center;
}

.column {
  display: flex;
  flex-direction: column;
  justify-content: center;
}

.frame {
  padding: 4px;
  border-radius: 4px;
  border: 1px solid transparent;
  background-color: var(--color-bg-1);
  box-shadow: 2px 2px 5px var(--color-shadow);
}

.icon {
  margin-top: auto;
  margin-bottom: auto;
  flex-shrink: 0;
}

.icon:focus {
  outline: none;
}

input,
button,
select,
.button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-family: inherit;
  background-color: var(--color-bg-1);
  transition: all 0.3s ease;
  box-shadow: 0 2px 2px var(--color-shadow);
}

select {
  padding: 0.1em 0.8em;
  color: var(--color-text);
  border-radius: 4px;
  cursor: pointer;
}

.checkbox {
  margin-top: auto;
  margin-bottom: auto;
  width: 20px;
  height: 20px;
}

.checkbox:checked {
  background-color: var(--color-accent);
}

input:focus {
  outline: none !important;
  border: 1px solid var(--color-accent);
}

button:hover {
  border-color: var(--color-accent);
}

button {
  cursor: pointer;
}

button:disabled {
  cursor: not-allowed;
  opacity: 0.7;
  pointer-events: none;
}

.icon-loading {
  animation: icon-loading-anim 1.8s linear infinite;
}

.v-enter-active,
.v-leave-active {
  transition: all 0.2s ease;
}

.v-enter-from,
.v-leave-to {
  opacity: 0;
}

@keyframes icon-loading-anim {
  0% {
    transform: rotate(0deg);
  }
  100% {
    transform: rotate(360deg);
  }
}
</style>

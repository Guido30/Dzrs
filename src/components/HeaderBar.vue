<script setup>
import { onMounted, toRef } from "vue";
import { appWindow } from "@tauri-apps/api/window";
import { IconLayoutList } from "@tabler/icons-vue";

import { activePageNameGlobal, showNotificationsGlobal, showNotificationDotGlobal } from "../globals";

const activePageName = toRef(activePageNameGlobal);
const showNotifications = toRef(showNotificationsGlobal);
const showNotificationDot = toRef(showNotificationDotGlobal);

async function changeView(page) {
  activePageName.value = page;
  await appWindow.emit("view-change", page);
}

async function toggleNotifications() {
  showNotifications.value = !showNotifications.value;
  await appWindow.emit("show-notifications", showNotifications.value);
}

onMounted(async () => {
  await appWindow.listen("show-notifications-dot", (value) => {
    if (value.payload === true) {
      showNotificationDot.value = true;
    } else {
      showNotificationDot.value = false;
    }
  });
});
</script>

<template>
  <div class="row header-bar">
    <img src="../assets/logox256.png" />
    <slot>
      <div style="flex-grow: 1"><!-- Fallback spacer --></div>
    </slot>
    <div class="row header-btns">
      <div class="row header-btn" @click="changeView('Main')" :class="{ active: activePageName === 'Main' }">
        <p>Tagging</p>
      </div>
      <div class="row header-btn" @click="changeView('Settings')" :class="{ active: activePageName === 'Settings' }">
        <p>Settings</p>
      </div>
      <div class="row header-btn" @click="changeView('About')" :class="{ active: activePageName === 'About' }">
        <p>About</p>
      </div>
      <div class="row header-btn" :class="{ 'btn-hover': showNotifications }" style="position: relative" @click="toggleNotifications">
        <IconLayoutList class="icon" />
        <div class="notification-dot" :class="{ shown: showNotificationDot, hidden: !showNotificationDot }"></div>
      </div>
    </div>
  </div>
</template>

<style scoped>
h1 {
  font-size: 1.2em;
  font-style: italic;
  margin-right: 10px;
  margin-top: 2px;
  margin-bottom: 2px;
  user-select: none;
}

.row {
  padding-left: 10px;
  padding-right: 10px;
}

.header-bar {
  background-color: var(--color-bg-1);
  box-shadow: 0 2px 2px transparent;
  border-bottom: 1px solid var(--color-accent);
  box-shadow: 4px 4px 5px var(--color-shadow);
  padding: 8px;
  padding-left: 20px;
  padding-right: 20px;
  z-index: 10;
}

.header-btns {
  gap: 15px;
  margin-top: auto;
  margin-bottom: auto;
}

.header-btn {
  border-bottom-left-radius: 3px;
  border-bottom-right-radius: 3px;
  gap: 5px;
  padding: 5px;
  padding-left: 10px;
  padding-right: 10px;
  border: 1px solid transparent;
  cursor: pointer;
  transition: all 0.2s ease-in-out;
}

.active {
  background-color: var(--color-hover);
  border-top: 1px solid var(--color-accent);
}

.header-btn:hover,
.btn-hover {
  background-color: var(--color-hover);
  border-top: 1px solid var(--color-accent);
}

.notification-dot {
  width: 15px;
  height: 15px;
  position: absolute;
  right: 10px;
  top: 5px;
  border-radius: 50%;
  background-color: var(--color-accent);
  transition: all 0.2s ease;
}

.hidden {
  opacity: 0%;
}

.shown {
  opacity: 100%;
}

img {
  width: 60px;
  height: 50px;
  margin-top: auto;
  margin-bottom: auto;
  user-select: none;
  padding-right: 10px;
}

p {
  font-size: 1.1em;
  margin-top: 0px;
  margin-bottom: 0px;
  user-select: none;
}
</style>

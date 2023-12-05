<script setup>
import { onMounted, ref } from "vue";
import { appWindow } from "@tauri-apps/api/window";
import { IconLayoutList } from "@tabler/icons-vue";

import { globalEmitter } from "../globals";

const emit = defineEmits(["showNotifications"]);

const activeButton = ref("Main");
const showNotifications = ref(false);
const notificationDot = ref(null);

function emitPageChange(page) {
  activeButton.value = page;
  appWindow.emit("page-change", page);
}

function emitShowNotifications() {
  showNotifications.value = !showNotifications.value;
  emit("showNotifications", showNotifications.value);
}

onMounted(() => {
  globalEmitter.on("notifications-state", (value) => {
    if (value === true) {
      notificationDot.value.classList.remove("hidden");
      notificationDot.value.classList.add("shown");
    } else {
      notificationDot.value.classList.remove("shown");
      notificationDot.value.classList.add("hidden");
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
      <div class="row header-btn" @click="emitPageChange('Main')" :class="{ active: activeButton === 'Main' }">
        <p>Tagging</p>
      </div>
      <div class="row header-btn" @click="emitPageChange('Download')" :class="{ active: activeButton === 'Download' }">
        <p>Download</p>
      </div>
      <div class="row header-btn" @click="emitPageChange('Settings')" :class="{ active: activeButton === 'Settings' }">
        <p>Settings</p>
      </div>
      <div class="row header-btn" @click="emitPageChange('About')" :class="{ active: activeButton === 'About' }">
        <p>About</p>
      </div>
      <div class="row header-btn" :class="{ 'btn-hover': showNotifications }" style="position: relative" @click="emitShowNotifications">
        <IconLayoutList class="icon" />
        <div class="notification-dot hidden" ref="notificationDot"></div>
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
  padding-left: 15px;
  padding-right: 15px;
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
  border-radius: 50%;
  background-color: var(--color-accent);
  transition: all 0.2s ease;
  animation-fill-mode: forwards;
}

.hidden {
  opacity: 0%;
  top: 25px;
}

.shown {
  animation: notification-dot-anim 0.4s forwards;
}

img {
  width: 50px;
  height: 50px;
  margin-top: auto;
  margin-bottom: auto;
  user-select: none;
}

p {
  font-size: 1.1em;
  margin-top: 0px;
  margin-bottom: 0px;
  user-select: none;
}

@keyframes notification-dot-anim {
  0% {
    opacity: 0%;
    top: 25px;
  }
  60% {
    opacity: 100%;
  }
  80% {
    top: 0px;
  }
  100% {
    top: 5px;
  }
}
</style>

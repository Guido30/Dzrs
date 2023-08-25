<script setup>
import { ref } from "vue";
import { appWindow } from "@tauri-apps/api/window";
import { IconLayoutList } from "@tabler/icons-vue";

const emit = defineEmits(["showNotifications"]);

const activeButton = ref("Main");
const showNotifications = ref(false);

function emitPageChange(page) {
  activeButton.value = page;
  appWindow.emit('page-change', page);
}

function emitShowNotifications() {
    showNotifications.value = !showNotifications.value;
    emit("showNotifications", showNotifications.value);
};

</script>

<template>
  <div class="row header-bar" style="margin-bottom: 10px;">
    <img src="../assets/logox256.png">
    <div class="row header-btns">
        <div class="row header-btn" @click="emitPageChange('Main')" :class="{ active: activeButton === 'Main'}">
          <p>Home</p>
        </div>
        <div class="row header-btn" @click="emitPageChange('Download')" :class="{ active: activeButton === 'Download'}">
          <p>Download</p>
        </div>
        <div class="row header-btn" @click="emitPageChange('Settings')" :class="{ active: activeButton === 'Settings'}">
          <p>Settings</p>
        </div>
        <div class="row header-btn" @click="emitPageChange('About')" :class="{ active: activeButton === 'About'}">
          <p>About</p>
        </div>
        <div class="row header-btn" @click="emitShowNotifications">
          <IconLayoutList class="icon"/>
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
  justify-content: space-between;
}

.header-btns {
  gap: 10px;
  margin-top: auto;
  margin-bottom: auto;
}

.header-btn {
  border-radius: 20px;
  gap: 5px;
  padding: 10px;
  padding-left: 15px;
  padding-right: 15px;
  border: 1px solid transparent;
  cursor: pointer;
  transition: all 0.2s ease-in-out;
}

.active {
  background-color: var(--color-hover);
}

.header-btn:hover {
  background-color: var(--color-hover);
  border: 1px solid var(--color-accent);
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
</style>

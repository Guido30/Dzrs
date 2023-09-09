<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { open } from "@tauri-apps/api/dialog";
import { downloadDir } from "@tauri-apps/api/path";
import { writeText } from '@tauri-apps/api/clipboard';
import { IconFolder, IconTextSize, IconCheck, IconFile } from "@tabler/icons-vue";
import SettingsGroup from "../components/SettingsGroup.vue";

import { appConfig, filterColumnsDownload, globalEmitter } from "../helpers";

const downloadInputValue = ref(appConfig.download_path);
const fileTemplateInput = ref(null);
const fileTemplateValue = ref(appConfig.file_template);
const overwriteDownloadsInput = ref(null);
const overwriteDownloadsValue = ref(appConfig.overwrite_downloads);
const localFilesInputValue = ref(appConfig.directory_view_path);

async function setDownloadPath() {
  const defaultPath = await downloadDir()
    .then((result) => result)
    .catch((err) => globalEmitter.emit("notification-add", { type: "Error", origin: "setDownloadPath", msg: err }));
  const path = await open({ defaultPath: defaultPath, directory: true, multiple: false})
    .then((result) => result)
    .catch((err) => globalEmitter.emit("notification-add", { type: "Error", origin: "setDownloadPath", msg: err })); 
  if (path !== null) {
    await invoke("update_config", { key: "download_path", value: path })
      .then((_) => "")
      .catch((err) => globalEmitter.emit("notification-add", { type: "Error", origin: "setDownloadPath", msg: err }));
    appConfig.download_path = path;
    downloadInputValue.value = path;
    globalEmitter.emit("instant-notification-add", { type: "Info", origin: "Settings", msg: "Setting Updated!" });
  };
};

async function saveFileTemplate() {
  await invoke("update_config", { key: "file_template", value: fileTemplateInput.value.value })
    .then((_) => "")
    .catch((err) => globalEmitter.emit("notification-add", { type: "Error", origin: "saveFileTemplate", msg: err }));
  appConfig.file_template = fileTemplateInput.value.value
  globalEmitter.emit("instant-notification-add", { type: "Info", origin: "Settings", msg: "Setting Updated!" });
}

async function saveOverwriteDownloads() {
  const value = new String(overwriteDownloadsInput.value.checked)
  await invoke("update_config", { key: "overwrite_downloads", value: value })
    .then((_) => "")
    .catch((err) => globalEmitter.emit("notification-add", { type: "Error", origin: "saveOverwriteDownloads", msg: err }));
}

async function copyEventTargetToClipboard(event) {
  await writeText(event.target.innerHTML)
    .then((_) => "")
    .catch((err) => globalEmitter.emit("notification-add", { type: "Error", origin: "copyEventTargetToClipboard", msg: err }));
};

async function setLocalFilesPath() {
  const path = await open({ directory: true, multiple: false})
    .then((result) => result)
    .catch((err) => globalEmitter.emit("notification-add", { type: "Error", origin: "setLocalFilesPath", msg: err })); 
  if (path !== null) {
    await invoke("update_config", { key: "directory_view_path", value: path })
      .then((_) => "")
      .catch((err) => globalEmitter.emit("notification-add", { type: "Error", origin: "setLocalFilesPath", msg: err }));
    appConfig.directory_view_path = path;
    localFilesInputValue.value = path;
    globalEmitter.emit("instant-notification-add", { type: "Info", origin: "Settings", msg: "Setting Updated!" });
  };
}

</script>

<template>
  <div class="container">
    <div class="column">
      <SettingsGroup :body-as-column="true" class="group-download">
        <template #head>
          <IconFolder size="30" class="icon setting-icon"/>
          <h1>Download</h1>
        </template>
        <template #body>
          <p>Download Path</p>
          <div class="row">
            <input :value="downloadInputValue" type="text" placeholder="Open..." style="flex-grow: 1;">
            <button style="margin-left: 15px;" @click="setDownloadPath">
              <IconFolder color="var(--color-text)"/>
            </button>
          </div>
          <div class="row" style="justify-content: flex-start; padding-top: 20px;">
            <p style="margin-right: 5px;">Overwrite existing files</p>
            <input @change="saveOverwriteDownloads" type="checkbox" class="checkbox" ref="overwriteDownloadsInput" :checked="overwriteDownloadsValue">
          </div>
        </template>
      </SettingsGroup>
      <SettingsGroup :body-as-column="true" class="group-file-template">
        <template #head>
          <IconTextSize size="30" class="icon setting-icon"/>
          <h1>File Template</h1>
        </template>
        <template #body>
          <p style="font-size: 1.0em;">Downloaded files will be saved with the following name</p>
          <div class="row">
            <input style="flex-grow: 1;" type="text" spellcheck="false" placeholder="File name template..." :value="fileTemplateValue" ref="fileTemplateInput">
            <button @click.prevent="saveFileTemplate" style="margin-left: 10px;">
              <IconCheck color="var(--color-text)"/>
            </button>
          </div>
          <div class="row" style="margin-top: 10px; padding-left: 10px; padding-right: 10px; justify-content: flex-start; flex-wrap: wrap;">
            <p>Available Variables:</p>
            <p v-for="item in filterColumnsDownload" :key="item.id" @click="copyEventTargetToClipboard">{{ `%${item.key}%` }}</p>
          </div>
        </template>
      </SettingsGroup>
      <SettingsGroup :body-as-column="true" class="group-local-files">
        <template #head>
          <IconFile size="30" class="icon setting-icon"/>
          <h1>Local Files</h1>
        </template>
        <template #body>
          <p>Default Directory</p>
          <div class="row">
            <input :value="localFilesInputValue" type="text" placeholder="Open..." style="flex-grow: 1;">
            <button style="margin-left: 15px;" @click="setLocalFilesPath">
              <IconFolder color="var(--color-text)"/>
            </button>
          </div>
        </template>
      </SettingsGroup>
    </div>
  </div>
</template>

<style scoped>
.setting-icon {
  margin-left: 30px;
  margin-right: 20px;
}

h1 {
  font-size: 1.4em;
  font-style: italic;
  margin-top: auto;
  margin-bottom: auto;
  text-align: center;
  user-select: none;
}

.group-download p {
  text-align: left;
}

.group-file-template p {
  text-align: left;
}

.group-file-template .row p {
  text-align: left;
  font-size: 0.9em;
  margin: 4px;
  padding: 6px;
  border: 2px solid transparent;
}

.group-file-template .row p:not(:first-child) {
  border-radius: 10px;
  border: 2px solid var(--color-bg-2);
  background-color: var(--color-bg-1);
  cursor: pointer;
}

.group-file-template .row p:not(:first-child):hover {
  border: 2px solid var(--color-accent);
}

.group-local-files {
  text-align: left;
}
</style>

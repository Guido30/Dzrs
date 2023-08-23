<script setup>
import { inject, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { open } from "@tauri-apps/api/dialog";
import { downloadDir } from "@tauri-apps/api/path";
import { writeText } from '@tauri-apps/api/clipboard';
import { IconFolder, IconTextSize, IconCheck } from "@tabler/icons-vue";
import SettingsGroup from "../components/SettingsGroup.vue";

const fileTemplateInput = ref(null);
const fileTemplateValue = ref(inject("appConfig").file_template);

const downloadInputValue = ref(inject("appConfig").download_path);

async function setDownloadPath() {
  const defaultPath = await downloadDir().then((result) => result).catch((_) => "");
  const path = await open({ defaultPath: defaultPath, directory: true, multiple: false}).then((result) => result).catch((_) => ""); 
  if (path !== null) {
    downloadInputValue.value = path;
    await invoke("update_config", { key: "download_path", value: path })
  };
};

async function saveFileTemplate() {
  await invoke("update_config", { key: "file_template", value: fileTemplateInput.value.value })
}

async function copyEventTargetToClipboard(event) {
  await writeText(event.target.innerHTML);
};

</script>

<template>
  <div class="container">
    <div class="column">
      <SettingsGroup>
        <template #head>
          <IconFolder size="30" class="icon setting-icon"/>
          <h1>Download Path</h1>
        </template>
        <template #body>
          <input :value="downloadInputValue" type="text" placeholder="Open..." style="flex-grow: 1;">
          <button style="margin-left: 15px;" @click="setDownloadPath">
            <IconFolder color="var(--color-text)"/>
          </button>
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
            <input style="flex-grow: 1;" type="text" placeholder="File name template..." :value="fileTemplateValue" ref="fileTemplateInput">
            <button @click.prevent="saveFileTemplate" style="margin-left: 10px;">
              <IconCheck color="var(--color-text)"/>
            </button>
          </div>
          <div class="row" style="margin-top: 10px; padding-left: 10px; padding-right: 10px; justify-content: flex-start;">
            <p>Available Variables:</p>
            <p v-for="item in inject('appConfig').fileTemplateVars" :key="item.id" @click="copyEventTargetToClipboard">{{ item }}</p>
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
</style>

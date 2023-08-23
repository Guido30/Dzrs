<script setup>
import { inject, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { open } from "@tauri-apps/api/dialog";
import { downloadDir } from "@tauri-apps/api/path";
import { IconFolder, IconTextSize } from "@tabler/icons-vue";
import SettingsGroup from "../components/SettingsGroup.vue";

const downloadInputValue = ref(inject("appConfig").download_path);
const fileTemplateValue = ref(inject("appConfig").file_template);

async function setDownloadPath() {
  const defaultPath = await downloadDir().then((result) => result).catch((_) => "");
  const path = await open({ defaultPath: defaultPath, directory: true, multiple: false}).then((result) => result).catch((_) => ""); 
  if (path !== null) {
    downloadInputValue.value = path;
    invoke("update_config", { key: "download_path", value: path })
  };
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
      <SettingsGroup>
        <template #head>
          <IconTextSize size="30" class="icon setting-icon"/>
          <h1>File Template</h1>
        </template>
        <template #body >
          <p>Downloaded file names template</p>
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
</style>

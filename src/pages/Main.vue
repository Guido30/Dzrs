<script setup>
import { ref, reactive, onMounted, computed } from "vue";
import { invoke } from "@tauri-apps/api";
import { appWindow } from "@tauri-apps/api/window";
import { open } from "@tauri-apps/api/dialog";
import { IconDotsVertical, IconFolder, IconClipboardList } from "@tabler/icons-vue";

import { appConfig, filterColumnsDirView, globalEmitter, fileIconPaths } from "../helpers";

const files = reactive([{}]);
const selectedFilePaths = ref([]);
const activeFilePath = computed(() => selectedFilePaths.value.length >= 1 ? selectedFilePaths.value[selectedFilePaths.value.length - 1] : false);
const showFilterMenu = ref(false);
const dirPanelColumns = reactive([]);
const currentWatchedPath = ref(appConfig.directory_view_path);

async function loadFilesIntoView() {
  const result = await invoke("watcher_get_files", { path: currentWatchedPath.value })
    .then((res) => res)
    .catch((err) => globalEmitter.emit("notification-add", { type: "Error", origin: "watcher_get_files", msg: err }));
  if (result) {
    files.value = result.items;
  };
}

async function updateViewPath() {
  const path = await open({ defaultPath: appConfig.directory_view_path, directory: true, multiple: false })
    .then((res) => res)
    .catch((err) => globalEmitter.emit("notification-add", { type: "Error", origin: "updateViewPath", msg: err }));
  if (path !== null) {
    currentWatchedPath.value = path;
    loadFilesIntoView();
    await invoke("watch_directory", { path: path })
      .catch((err) => globalEmitter.emit("notification-add", { type: "Error", origin: "updateViewPath", msg: err }));
  };
};

function iconPathFromExtension(extension) {
  const path = fileIconPaths.hasOwnProperty(extension) ? fileIconPaths[`${extension}`] : fileIconPaths["default"];
  return path
};

function selectFiles(event, file) {
  if (event.shiftKey) {
    let indexStart;
    if (selectedFilePaths.value.length >= 1) {
      const _filePath = selectedFilePaths.value[selectedFilePaths.value.length - 1];
      const _i = files.value.findIndex(_file => _file.path === _filePath);
      indexStart = _i;
    } else {
      indexStart = 0;
    };
    const indexEnd = files.value.indexOf(file);
    if (indexStart < indexEnd) {
      for (let i = indexStart + 1; i <= indexEnd; i++) {
        const _filePath = files.value[i].path;
        if (selectedFilePaths.value.includes(_filePath)) {
          const _i = selectedFilePaths.value.indexOf(_filePath);
          selectedFilePaths.value.splice(_i, 1);
        } else {
          selectedFilePaths.value.push(_filePath);
        };
      };
    } else {
      for (let i = indexStart - 1; i >= indexEnd; i--) {
        const _filePath = files.value[i].path;
        if (selectedFilePaths.value.includes(_filePath)) {
          const _i = selectedFilePaths.value.indexOf(_filePath);
          selectedFilePaths.value.splice(_i, 1);
        } else {
          selectedFilePaths.value.push(_filePath);
        };
      };
    };
  } else if (event.ctrlKey) {
    if (selectedFilePaths.value.includes(file.path)) {
      let i = selectedFilePaths.value.indexOf(file.path);
      selectedFilePaths.value.splice(i, 1);
    } else {
      selectedFilePaths.value.push(file.path);
    }
  } else {
    selectedFilePaths.value = [file.path];
  };
};

async function saveFilterColumn(filterColumnDirView) {
  await invoke("update_config", { key: filterColumnDirView.config, value: `${filterColumnDirView.enabled}` })
    .then((_) => "")
    .catch((err) => globalEmitter.emit("notification-add", { type: "Error", origin: "saveFilterColumn", msg: err }));
}

onMounted(() => {
  loadFilesIntoView();
  appWindow.listen("watcher_fired", async (_) => {
    loadFilesIntoView();
  });
});
</script>

<template>
  <div class="container" style="gap: 5px;">
    <div class="row" style="gap: 5px; flex-grow: 1;">
      <div class="directory-panel">
        <div class="row" style="gap: 10px; margin-bottom: 8px;">
          <button style="padding: 2px 6px;" @click="updateViewPath">
            <IconFolder size="20" color="var(--color-text)" class="icon" style="margin-top: 4px;"/>
          </button>
          <p style="font-weight: 300; letter-spacing: 0.12em; padding: 2px 6px;" class="button">
            {{ currentWatchedPath ? currentWatchedPath : "..." }}
          </p>
          <button style="padding: 2px 8px; margin-left: auto;">
            <div class="row" style="color: var(--color-text);">
              Fetch Tags
              <IconClipboardList size="20" color="var(--color-text)" class="icon" style="margin-left: 3px;"/>
            </div>
          </button>
        </div>
        <div class="frame">
          <table>
            <thead class="table-header">
              <tr>
                <th style="width: 20px; position: relative;">
                  <IconDotsVertical size="18" class="icon table-filter-btn" :class="{ 'filter-btn-expanded': showFilterMenu }" @click="showFilterMenu = !showFilterMenu"/>
                  <div class="filter-menu" v-if="showFilterMenu" @click.stop>
                    <div class="filter-menu-arrow"></div>
                    <div v-for="column in filterColumnsDirView" :key="column.key">
                      <label>
                        <input class="filterItemInput" type="checkbox" @change="saveFilterColumn(column)" :disabled="column.readonly" v-model="column.enabled"/>
                        {{ column.label }}
                      </label>
                    </div>
                  </div>
                </th>
                <th><!-- Reserved for image --></th>
                <th v-for="column in filterColumnsDirView" :key="column.key" v-show="column.enabled">{{ column.label }}</th>
              </tr>
            </thead>
            <tbody>
              <template v-for="file in files.value" :key="file.path">
                <tr @click="selectFiles($event, file)" :class="{ 'selected-file': selectedFilePaths.includes(file.path) }">
                  <td><!-- Empty cell reserved for table filter --></td>
                  <td class="img-container">
                    <img :src="iconPathFromExtension(file.extension)" class="icon">
                  </td>
                  <td v-show="filterColumnsDirView.find((col) => col.key === 'filename' && col.enabled)" style="text-align: left; font-style: italic;">{{ file.path }}</td>
                  <td v-show="filterColumnsDirView.find((col) => col.key === 'size' && col.enabled)">{{ Math.round( file.size / 1024 ) }} KB</td>
                  <td v-show="filterColumnsDirView.find((col) => col.key === 'extension' && col.enabled)">{{ file.extension }}</td>
                </tr>
              </template>
            </tbody>
          </table>
        </div>
      </div>
      <div class="source-panel">
        <div class="column" style="flex-grow: 1;">
          <div class="frame">
            Ok
          </div>
        </div>
      </div>
    </div>
    <div class="tags-panel">
      <div class="column" style="flex-grow: 1;">
        <div class="frame">
          <table>
            <thead class="table-header">
              <tr>
                <th style="width: 10%;">Tag</th>
                <th>Current Value</th>
                <th>Future Value</th>
              </tr>
            </thead>
            <tbody>
              <tr>
                <th>Title</th>
                <td></td>
                <td><input type="text"></td>
              </tr>
              <tr>
                <th>Artist</th>
                <td></td>
                <td><input type="text"></td>
              </tr>
              <tr>
                <th>Album</th>
                <td></td>
                <td><input type="text"></td>
              </tr>
              <tr>
                <th>Genre</th>
                <td></td>
                <td><input type="text"></td>
              </tr>
              <tr>
                <th>Length</th>
                <td></td>
                <td><input type="text"></td>
              </tr>
              <tr>
                <th>Date</th>
                <td></td>
                <td><input type="text"></td>
              </tr>
              <tr>
                <th>Album Artist</th>
                <td></td>
                <td><input type="text"></td>
              </tr>
              <tr>
                <th>Artists</th>
                <td></td>
                <td><input type="text"></td>
              </tr>
              <tr>
                <th>ISRC</th>
                <td></td>
                <td><input type="text"></td>
              </tr>
              <tr>
                <th>Track Number</th>
                <td></td>
                <td><input type="text"></td>
              </tr>
              <tr>
                <th>Total Tracks</th>
                <td></td>
                <td><input type="text"></td>
              </tr>
              <tr>
                <th>Disc Number</th>
                <td></td>
                <td><input type="text"></td>
              </tr>
              <tr>
                <th>Total Discs</th>
                <td></td>
                <td><input type="text"></td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.directory-panel, .tags-panel, .source-panel {
  flex-grow: 1;
  display: flex;
  flex-direction: column;
}

.tags-panel {
  flex-basis: 300px;
  overflow-y: auto;
  flex-grow: 0;
}

.tags-panel tbody {
  font-size: 0.8em;
}

.frame {
  flex-grow: 1;
}

p {
  margin-top: auto;
  margin-bottom: auto;
}

table {
  width: 100%;
  border-collapse: separate;
  border-spacing: 0px 4px;
  user-select: none;
}

tbody {
  font-size: 0.95em;
  font-weight: 300;
}

tbody tr td:nth-child(1) {
  border-top-left-radius: 8px;
  border-bottom-left-radius: 8px;
}

tbody tr td:last-child {
  border-top-right-radius: 8px;
  border-bottom-right-radius: 8px;
}

tbody tr:hover, .selected-file {
  background-color: var(--color-hover);
}

.table-header {
  font-style: italic;
  background-color: var(--color-bg-1);
  position: sticky;
  top: 0;
  z-index: 1;
}

.table-header th {
  font-weight: 500;
  border-bottom: 1px solid var(--color-bg-2);
  text-wrap: nowrap;
}

tbody td:not(:last-child) {
  border-right: 1px solid var(--color-hover);
}

.tags-panel tbody input {
  width: 100%;
  border-radius: unset;
  border: unset;
  padding: 1px 0px;
  font-size: unset;
  font-family: inherit;
  box-shadow: unset;
}

.img-container, .img-container img {
  width: 30px;
}

.table-filter-btn {
  margin-top: 4px;
  padding: 2px;
  cursor: pointer;
  border-radius: 10px;
  border: 1px solid transparent;
  transform: rotate(0deg);
  transition: all 0.2s ease;
}

.table-filter-btn:hover {
  border: 1px solid var(--color-accent);
  background-color: var(--color-hover);
}

.expanded {
  display: flex;
  width: 200px;
}

.filter-btn-expanded {
  transform: rotate(90deg);
}

.filter-menu {
  position: absolute;
  min-width: max-content;
  text-align: left;
  margin-top: 8px;
  padding: 10px;
  padding-top: 5px;
  padding-bottom: 5px;
  background-color: var(--color-bg-2);
  border: 1px solid var(--color-accent);
  border-radius: 10px;
  border-top-left-radius: 4px;
  z-index: 2;
}

.filter-menu-arrow {
  width: 0; 
  height: 0; 
  border-top: 0px solid transparent; 
  border-bottom: 15px solid var(--color-accent);
  border-left: 10px solid transparent;
  border-right: 10px solid transparent;
  position: absolute;
  top: -15px;
  left: 1px;
}

.filter-menu label {
  color: var(--color-text);
  font-size: 1em;
  font-weight: 400;
  user-select: none;
}
</style>

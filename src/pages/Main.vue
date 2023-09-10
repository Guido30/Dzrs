<script setup>
import { ref, reactive, onMounted, computed } from "vue";
import { invoke } from "@tauri-apps/api";
import { appWindow } from "@tauri-apps/api/window";
import { open } from "@tauri-apps/api/dialog";
import { IconDotsVertical, IconFolder, IconClipboardList, IconDeviceFloppy } from "@tabler/icons-vue";

import { appConfig, filterColumnsDirView, globalEmitter, fileIconPaths } from "../helpers";

const dzrsTracks = reactive([{}]);
const dzrsFiles = reactive([{}]);
const selectedDzrsFilePaths = ref([]);
const activeDzrsFilePath = computed(() => selectedDzrsFilePaths.value.length >= 1 ? selectedDzrsFilePaths.value[selectedDzrsFilePaths.value.length - 1] : false);
const showFilterMenu = ref(false);
const currentWatchedPath = ref(appConfig.directory_view_path);
const tagsNeedAmend = ref(false);

async function loadFilesIntoView() {
  const result = await invoke("watcher_get_files", { path: currentWatchedPath.value })
    .then((res) => res)
    .catch((err) => globalEmitter.emit("notification-add", { type: "Error", origin: "watcher_get_files", msg: err }));
  if (result) {
    dzrsFiles.value = result.items;
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
    if (selectedDzrsFilePaths.value.length >= 1) {
      const _filePath = selectedDzrsFilePaths.value[selectedDzrsFilePaths.value.length - 1];
      const _i = dzrsFiles.value.findIndex(_file => _file.path === _filePath);
      indexStart = _i;
    } else {
      indexStart = 0;
    };
    const indexEnd = dzrsFiles.value.indexOf(file);
    if (indexStart < indexEnd) {
      for (let i = indexStart + 1; i <= indexEnd; i++) {
        const _filePath = dzrsFiles.value[i].path;
        if (selectedDzrsFilePaths.value.includes(_filePath)) {
          const _i = selectedDzrsFilePaths.value.indexOf(_filePath);
          selectedDzrsFilePaths.value.splice(_i, 1);
        } else {
          selectedDzrsFilePaths.value.push(_filePath);
        };
      };
    } else {
      for (let i = indexStart - 1; i >= indexEnd; i--) {
        const _filePath = dzrsFiles.value[i].path;
        if (selectedDzrsFilePaths.value.includes(_filePath)) {
          const _i = selectedDzrsFilePaths.value.indexOf(_filePath);
          selectedDzrsFilePaths.value.splice(_i, 1);
        } else {
          selectedDzrsFilePaths.value.push(_filePath);
        };
      };
    };
  } else if (event.ctrlKey) {
    if (selectedDzrsFilePaths.value.includes(file.path)) {
      let i = selectedDzrsFilePaths.value.indexOf(file.path);
      selectedDzrsFilePaths.value.splice(i, 1);
    } else {
      selectedDzrsFilePaths.value.push(file.path);
    }
  } else {
    selectedDzrsFilePaths.value = [file.path];
  };
};

async function saveFilterColumn(filterColumnDirView) {
  await invoke("update_config", { key: filterColumnDirView.config, value: `${filterColumnDirView.enabled}` })
    .then((_) => "")
    .catch((err) => globalEmitter.emit("notification-add", { type: "Error", origin: "saveFilterColumn", msg: err }));
}

async function getDzrsTracks() {
  let flacs;
  if (selectedDzrsFilePaths.value.length === 0) {
    flacs = dzrsFiles.value.filter((file) => file.extension === "flac");
  } else {
    flacs = dzrsFiles.value.filter((file) => file.extension === "flac" && selectedDzrsFilePaths.value.includes(file.path));
  };
  const flac_paths = flacs.map((f) => f.path);
  const result = await invoke("get_dzrs_tracks", { paths: flac_paths })
    .then((res) => res)
    .catch((err) => globalEmitter.emit("notification-add", { type: "Error", origin: "getDzrsTracks", msg: err }));
  dzrsTracks.value = result.items;
  console.log(dzrsTracks.value);
};

onMounted(() => {
  loadFilesIntoView();
  appWindow.listen("watcher_fired", async (_) => {
    loadFilesIntoView();
  });
  document.addEventListener('click', (event) => {
    if (!event.target.closest('.table-filter-btn')) {
      showFilterMenu.value = false;
    }
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
          <p style="font-weight: 300; letter-spacing: 0.12em; padding: 2px 6px; margin-right: auto;" class="button">
            {{ currentWatchedPath ? currentWatchedPath : "..." }}
          </p>
          <button style="padding: 2px 8px; margin-left: auto;" v-show="tagsNeedAmend">
            <div class="row" style="color: var(--color-text);">
              Amend Tags
              <IconDeviceFloppy size="20" color="var(--color-text)" class="icon" style="margin-left: 3px;"/>
            </div>
          </button>
          <button style="padding: 2px 8px;" @click="getDzrsTracks">
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
              <template v-for="file in dzrsFiles.value" :key="file.path">
                <tr @click="selectFiles($event, file)" :class="{ 'selected-file': selectedDzrsFilePaths.includes(file.path) }">
                  <td><!-- Empty cell reserved for table filter --></td>
                  <td class="img-container">
                    <img :src="iconPathFromExtension(file.extension)" class="icon">
                  </td>
                  <td v-show="filterColumnsDirView.find((col) => col.key === 'filename' && col.enabled)" style="text-align: left; font-style: italic;">{{ file.filename }}</td>
                  <td v-show="filterColumnsDirView.find((col) => col.key === 'size' && col.enabled)">{{ Math.round( file.size / 1024 ) }} KB</td>
                  <td v-show="filterColumnsDirView.find((col) => col.key === 'extension' && col.enabled)">{{ file.extension }}</td>
                  <td v-show="filterColumnsDirView.find((col) => col.key === 'tagStatus' && col.enabled)">{{ file.tag_status }}</td>
                </tr>
              </template>
            </tbody>
          </table>
        </div>
      </div>
      <div class="source-panel">
        <div class="column" style="flex-grow: 1;">
          <div class="frame">
            <div class="column">
              <p>Sources</p>
            </div>
          </div>
        </div>
      </div>
    </div>
    <div class="tags-panel">
      <div class="frame row">
        <div class="image-tag column" :set="track = dzrsTracks.find((track) => track.path === activeDzrsFilePath) ">
          <div class="column">
            <div v-if="track" v-for="picture in track.pictures">
              <img :src="`data:image/png;base64, ${picture.b64}`">
              <p>{{ picture.pic_type }}</p>
              <p>{{ picture.description }}</p>
              <p>{{ picture.width }}x{{ picture.height }}</p>
            </div>
            <div v-else>
              <img src="assets/tag-image-placeholder.png">
            </div>
          </div>
        </div>
        <div class="column" style="flex-grow: 1;">
          <div class="column" style="flex-basis: 400px; overflow-y: auto;">
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
  </div>
</template>

<style scoped>
.directory-panel, .tags-panel, .source-panel {
  flex-grow: 1;
  display: flex;
  flex-direction: column;
}

.tags-panel {
  flex-grow: 0;
}

.image-tag {
  flex-basis: 150px;
  flex-grow: 0;
  flex-shrink: 0;
  border-right: 1px solid var(--color-bg-2);
}

.image-tag .column {
  flex-basis: 400px;
  justify-content: start;
  align-items: center;
  overflow-y: auto;
  overflow-x: hidden;
}

.image-tag .column::-webkit-scrollbar {
  display: none;
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
  padding-top: 0px;
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

.image-tag, .image-tag img {
  width: 150px;
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

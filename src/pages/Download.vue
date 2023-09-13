<script setup>
import { ref, computed, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/tauri"
import { IconSearch, IconFolder, IconTrash, IconDotsVertical, IconArrowBarLeft, IconLoader2 } from "@tabler/icons-vue";
import SlavartDownloadItem from "../components/SlavartDownloadItem.vue";
import DownloadInfoItem from "../components/DownloadInfoItem.vue";

import { appConfig, parseFileName, openFileBrowser, filterColumnsDownload, globalEmitter } from "../helpers";

const slavartItems = ref([]);
const infoItems = ref([]);
const inputElement = ref(null);
const isDownloadExpanded = ref(false);
const showFilterMenu = ref(false);
const isSearchPending = ref(false);

const infoItemsIds = computed({
  get: () => infoItems.value.map((item) => item.id),
  set: (val) => {
    const toRemoveIndex = infoItems.value.findIndex((item) => item.id === val);
    if (toRemoveIndex !== -1) {
      infoItems.value.splice(toRemoveIndex, 1);
    };
  }
});

async function searchTracks() {
  slavartItems.value = [];
  isSearchPending.value = true;
  await invoke("get_slavart_tracks", { query: `${inputElement.value.value}` })
    .then((result) => {slavartItems.value = result.items})
    .catch((err) => globalEmitter.emit("notification-add", { type: "Error", origin: "searchTracks", msg: err }));
  isSearchPending.value = false;
}

async function downloadTrack(item) {
  if (!infoItemsIds.value.includes(item.id)) {
    const i = infoItems.value.push(item) - 1;
    if (i === 0) {
      isDownloadExpanded.value = true;
    };
    const fileName = parseFileName(item, appConfig.fileTemplate);
    const downloadStatus = await invoke("download_track", { id: item.id, filename: fileName })
      .then((_) => infoItems.value[i].status = true)
      .catch((err) => {
        infoItems.value[i].status = false;
        infoItems.value[i].statusMsg = err
      });
  } else {
    globalEmitter.emit("notification-add", { type: "Info", origin: "downloadTrack", msg: `Track ${item.title} - ${item.album_title} is already in the queue` });
  };
};

function removeInfoItem(id) {
  infoItemsIds.value = id;
}

async function saveFilterColumn(filterColumnDownload) {
  await invoke("update_config", { key: filterColumnDownload.config, value: `${filterColumnDownload.enabled}` })
    .then((_) => "")
    .catch((err) => globalEmitter.emit("notification-add", { type: "Error", origin: "saveFilterColumn", msg: err }));
}

onMounted(() => {
  document.addEventListener('click', (event) => {
    if (!event.target.closest('.table-filter-btn')) {
      showFilterMenu.value = false;
    }
  });
});
</script>

<template>
  <div class="container" style="flex-direction: row; gap: 15px; max-width: 100vw;">
    <div class="column" style="flex-grow: 1; gap: 15px;">
      <div class="row" style="flex-basis: 50px;">
        <input placeholder="Song name..." spellcheck="false" @keypress.enter="searchTracks" ref="inputElement"/>
        <button style="margin-left: 10px;" @click="searchTracks">
            <IconSearch size="20" color="var(--color-text)" class="icon"/>
        </button>
        <button style="margin-left: 10px;" @click="isDownloadExpanded = !isDownloadExpanded">
          <IconArrowBarLeft size="20" color="var(--color-text)" class="icon" :class="{ 'download-expand-btn-expanded': isDownloadExpanded }"/>
        </button>
      </div>
      <div class="row" style="flex-grow: 1; overflow-y: auto; gap: 15px;">
        <div class="frame" style="flex-grow: 1">
          <table>
            <thead class="table-header">
              <tr>
                <th style="width: 20px; position: relative;">
                  <IconDotsVertical size="18" class="icon table-filter-btn" :class="{ 'filter-btn-expanded': showFilterMenu }" @click="showFilterMenu = !showFilterMenu"/>
                  <div class="filter-menu" v-if="showFilterMenu" @click.stop>
                    <div class="filter-menu-arrow"></div>
                    <div v-for="column in filterColumnsDownload" :key="column.key">
                      <label>
                        <input class="filterItemInput" type="checkbox" @change="saveFilterColumn(column)" :disabled="column.readonly" v-model="column.enabled"/>
                        {{ column.label }}
                      </label>
                    </div>
                  </div>
                </th>
                <th><!-- Reserved for image --></th>
                <th v-for="column in filterColumnsDownload" :key="column.key" v-show="column.enabled">{{ column.label }}</th>
              </tr>
            </thead>
            <tbody>
              <SlavartDownloadItem @downloadRequested="downloadTrack" :item-data="item" :columns="filterColumnsDownload" v-for="item in slavartItems" :key="item.id"></SlavartDownloadItem>
            </tbody>
          </table>
          <IconLoader2 size="100" class="icon-loading" style="height: 90%;" v-show="isSearchPending"/>
        </div>
      </div>
    </div>
    <div class="frame info-items" :class="{ 'expanded': isDownloadExpanded }">
        <div class="table-header">
          <p style="width: 100%; margin-top: 5px; margin-bottom: 0px; padding-bottom: 5px; border-bottom: 1px solid var(--color-bg-2);">Downloads</p>
        </div>
        <div>
          <DownloadInfoItem @removeRequested="removeInfoItem" :item-data="item" v-for="item in infoItems" :key="item.id"></DownloadInfoItem>
        </div>
        <div class="row downloads-btns">
            <IconFolder @click="openFileBrowser(appConfig.downloadPath)" size="30" style="cursor: pointer;" class="icon"/>
            <IconTrash @click="infoItems = []" size="30" style="cursor: pointer;" class="icon"/>
        </div>
      </div>
  </div>
</template>

<style scoped>
.frame {
  overflow-y: auto;
  overflow-x: hidden;
  padding-left: 10px;
  padding-right: 10px;
  padding-top: 0px;
  padding-bottom: 0px;
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

.items-header > p {
  font-style: italic;
  margin: 0px;
  margin-top: auto;
  margin-bottom: auto;
  margin-left: 20px;
  margin-right: 20px;
  text-align: center;
}

.info-items {
  width: 0px;
  display: none;
  flex-direction: column;
  flex-shrink: 0;
}

.downloads-btns {
  margin-top: auto;
  justify-content: flex-end;
  padding: 5px;
  border-top: 1px solid var(--color-bg-2);
  position: sticky;
  bottom: 0px;
  background-color: var(--color-bg-1);
  transition: all 0.2s ease;
}

.download-expand-btn-expanded {
  transform: rotate(180deg);
}

input {
  flex-grow: 1;
}

table {
  width: 100%;
  border-collapse: separate;
  border-spacing: 0px 4px;
}

tbody {
  font-size: 0.95em;
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

.icon-loading {
    animation: icon-loading-anim 1.8s linear infinite;
}
</style>

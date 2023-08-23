<script setup>
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/tauri"
import { IconSearch, IconFolder, IconTrash, IconDotsVertical, IconArrowBarLeft, IconArrowBarRight } from "@tabler/icons-vue";
import SlavartDownloadItem from "../components/SlavartDownloadItem.vue";
import DownloadInfoItem from "../components/DownloadInfoItem.vue";

const slavartItems = ref([]);
const infoItems = ref([]);

const inputElement = ref(null);

const isDownloadExpanded = ref(false);

const infoItemsIds = computed({
  get: () => infoItems.value.map((item) => item.id),
  set: (val) => {
    const toRemoveIndex = infoItems.value.findIndex((item) => item.id === val);
    if (toRemoveIndex !== -1) {
      infoItems.value.splice(toRemoveIndex, 1);
    };
  }
});

async function handleInput() {
  await invoke("get_slavart_tracks", { query: `${inputElement.value}` })
    .then((result) => {slavartItems.value = result.items})
    .catch((err) => console.log("ERR", err));
}

async function downloadTrack(item) {
  if (!infoItemsIds.value.includes(item.id)) {
    infoItems.value.push(item)
  };
  const downloadStatus = await invoke("download_track", { id: item.id, filename: item.title }).then((res) => true).catch((err) => {false; console.log(err);});
};

function removeInfoItem(id) {
  infoItemsIds.value = id
}
</script>

<template>
  <div class="container" style="flex-direction: row; gap: 15px; max-width: 100vw;">
    <div class="column" style="flex-grow: 1; gap: 15px;">
      <div class="row" style="flex-basis: 50px;">
        <input placeholder="Song name..." @keypress.enter="handleInput" :ref="inputElement"/>
        <button style="margin-left: 10px;" @click="handleInput()">
            <IconSearch size="20" color="var(--color-text)" class="icon"/>
        </button>
        <button style="margin-left: 10px;" @click="isDownloadExpanded = !isDownloadExpanded">
          <IconArrowBarRight size="20" color="var(--color-text)" class="icon" v-if="isDownloadExpanded"/>
          <IconArrowBarLeft color="var(--color-text)" class="icon" v-else/>
        </button>
      </div>
      <div class="row" style="flex-grow: 1; overflow-y: auto; gap: 15px;">
        <div class="frame" style="flex-grow: 1">
          <table>
            <thead class="table-header">
              <tr>
                <th style="width: 2%;">
                  <IconDotsVertical size="18" class="icon table-filter-btn"/>
                </th>
                <th><!-- Reserved for image --></th>
                <th>Title</th>
                <th>Album</th>
                <th>Artist</th>
                <th>Duration</th>
              </tr>
            </thead>
            <tbody>
              <SlavartDownloadItem @downloadRequested="downloadTrack" :item-data="item" v-for="item in slavartItems" :key="item.id"></SlavartDownloadItem>
            </tbody>
          </table>
        </div>
      </div>
    </div>
    <div class="frame info-items" :class="{ 'expanded': isDownloadExpanded}">
        <div class="table-header">
          <p style="width: 100%; margin-top: 5px; margin-bottom: 0px; padding-bottom: 5px; border-bottom: 1px solid var(--color-bg-2);">Downloads</p>
        </div>
        <div>
          <DownloadInfoItem @removeRequested="removeInfoItem" :item-data="item" v-for="item in infoItems" :key="item.id"></DownloadInfoItem>
        </div>
        <div class="row downloads-btns">
            <IconFolder size="30" class="icon"/>
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
}

input {
  flex-grow: 1;
}

table {
  width: 100%;
  border-collapse: separate;
  border-spacing: 0px 4px;
}

.table-filter-btn {
  margin-top: 4px;
  padding: 2px;
  cursor: pointer;
  border-radius: 10px;
  border: 1px solid transparent;
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
</style>

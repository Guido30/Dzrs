<script setup>
import { ref, reactive, computed } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { IconSearch, IconFolder, IconTrash, IconArrowBarLeft, IconLoader2, IconDownload, IconAlertCircle, IconCircleCheck } from "@tabler/icons-vue";
import TableFilter from "../components/TableFilter.vue";

import { appConfig, globalEmitter, filterColumnsDownload } from "../globals";

const slavartTracks = ref([]);
const queuedTracks = ref([]);
const inputElement = ref(null);

// Visibility toggles for various elements
const isDownloadExpanded = ref(false);
const isSearchPending = ref(false);

// Style variable for animating the track image when cliking on it
// TODO ITS CURRENTLY GLOBAL FOR EVERY IMG TAG
const imgDarkenStyle = reactive({ transform: "scale(1.0)" });

function parseFileName(fileData, template) {
  const fileName = template
    .replaceAll("%title%", fileData.title)
    .replaceAll("%album%", fileData.albumTitle)
    .replaceAll("%artist%", fileData.artist)
    .replaceAll("%genre%", fileData.genre)
    .replaceAll("%duration%", `${Math.floor(fileData.duration / 60)}.${(fileData.duration % 60).toString().padStart(2, "0")}`)
    .replaceAll("%date%", fileData.date)
    .replaceAll("%composer%", fileData.composer)
    .replaceAll("%isrc%", fileData.isrc)
    .replaceAll("%copyright%", fileData.copyright)
    .replaceAll("%bitDepth%", fileData.bitDepth)
    .replaceAll("%samplingRate%", fileData.samplingRate)
    .replaceAll(/[<>:"\/\\|?*]/g, " ");
  return fileName;
}

async function searchTracks() {
  if (inputElement.value.value.length === 0) return;
  slavartTracks.value = [];
  isSearchPending.value = true;
  await invoke("get_slavart_tracks", { query: `${inputElement.value.value}` })
    .then((result) => (slavartTracks.value = result.items))
    .catch((err) => globalEmitter.emit("notification-add", { type: "Error", origin: "searchTracks", msg: err }));
  isSearchPending.value = false;
}

async function downloadTrack(tr) {
  // Animate the track image
  imgDarkenStyle.transform = "scale(1.1)";
  setTimeout(() => {
    imgDarkenStyle.transform = "scale(1.0)";
  }, 100);
  // Download the track
  if (!queuedTracks.value.find((tr1) => tr1.id === tr.id)) {
    const i = queuedTracks.value.push(tr) - 1;
    if (i === 0) {
      isDownloadExpanded.value = true;
    }
    const fileName = parseFileName(tr, appConfig.fileTemplate);
    await invoke("download_track", { id: tr.id, filename: fileName })
      .then(() => (queuedTracks.value[i].status = true))
      .catch((err) => {
        queuedTracks.value[i].status = false;
        queuedTracks.value[i].statusMsg = err;
      });
  } else {
    globalEmitter.emit("notification-add", { type: "Info", origin: "downloadTrack", msg: `Track ${tr.title} - ${tr.albumTitle} is already in the queue` });
  }
}
</script>

<template>
  <div class="container" style="flex-direction: row; gap: 15px; max-width: 100vw">
    <div class="column" style="flex-grow: 1; gap: 15px">
      <div class="row" style="flex-basis: 50px">
        <input placeholder="Song name..." spellcheck="false" @keypress.enter="searchTracks" ref="inputElement" />
        <button style="margin-left: 10px" @click="searchTracks">
          <IconSearch size="20" color="var(--color-text)" class="icon" />
        </button>
        <button style="margin-left: 10px" @click="isDownloadExpanded = !isDownloadExpanded">
          <IconArrowBarLeft size="20" color="var(--color-text)" class="icon" :class="{ 'download-expand-btn-expanded': isDownloadExpanded }" />
        </button>
      </div>
      <div class="row" style="flex-grow: 1; overflow-y: auto; gap: 15px">
        <div class="frame slavart-tracks-container" style="flex-grow: 1">
          <table>
            <thead class="table-header">
              <tr>
                <th style="width: 0%; position: relative">
                  <TableFilter :columns="filterColumnsDownload" />
                </th>
                <th style="width: 0%"><!-- Reserved for image --></th>
                <th v-for="column in filterColumnsDownload" :key="column.key" v-show="column.enabled" :style="{ width: column.width + '%' }">
                  {{ column.label }}
                </th>
              </tr>
            </thead>
            <tbody v-show="!isSearchPending">
              <tr v-for="slavTrack in slavartTracks" :key="slavTrack.id" class="table-row">
                <td><!-- Empty cell reserved for table filter --></td>
                <td class="img-container" @click="downloadTrack(slavTrack)">
                  <IconDownload size="30" class="icon-download" />
                  <div class="img-darken" :style="imgDarkenStyle">
                    <img :src="slavTrack.thumbnail" width="50" />
                  </div>
                </td>
                <td v-show="filterColumnsDownload.find((i) => i.key === 'title').enabled" class="text-pad">{{ slavTrack.title }}</td>
                <td v-show="filterColumnsDownload.find((i) => i.key === 'album').enabled" class="text-pad">{{ slavTrack.albumTitle }}</td>
                <td v-show="filterColumnsDownload.find((i) => i.key === 'artist').enabled" class="text-pad">{{ slavTrack.artist }}</td>
                <td v-show="filterColumnsDownload.find((i) => i.key === 'genre').enabled" class="text-pad">{{ slavTrack.genre }}</td>
                <td v-show="filterColumnsDownload.find((i) => i.key === 'duration').enabled">{{ Math.floor(slavTrack.duration / 60) }}:{{ (slavTrack.duration % 60).toString().padStart(2, "0") }}</td>
                <td v-show="filterColumnsDownload.find((i) => i.key === 'date').enabled" class="text-pad">{{ slavTrack.date }}</td>
                <td v-show="filterColumnsDownload.find((i) => i.key === 'composer').enabled" class="text-pad">{{ slavTrack.composer }}</td>
                <td v-show="filterColumnsDownload.find((i) => i.key === 'isrc').enabled" class="text-pad">{{ slavTrack.isrc }}</td>
                <td v-show="filterColumnsDownload.find((i) => i.key === 'copyright').enabled" class="text-pad">{{ slavTrack.copyright }}</td>
                <td v-show="filterColumnsDownload.find((i) => i.key === 'bitDepth').enabled">{{ slavTrack.bitDepth }}</td>
                <td v-show="filterColumnsDownload.find((i) => i.key === 'samplingRate').enabled">{{ slavTrack.samplingRate }} kHz</td>
              </tr>
            </tbody>
          </table>
          <IconLoader2 size="100" class="icon-loading" style="height: 90%" v-show="isSearchPending" />
        </div>
      </div>
    </div>
    <div class="frame queued-tracks-container" :class="{ expanded: isDownloadExpanded }">
      <div class="table-header">
        <p style="width: 100%; margin-top: 5px; margin-bottom: 0px; padding-bottom: 5px; border-bottom: 1px solid var(--color-bg-2)">Downloads</p>
      </div>
      <div>
        <div v-for="(queuedTrack, i) in queuedTracks" :key="queuedTrack.id" class="queued-track">
          <img :src="queuedTrack.large" />
          <p>{{ queuedTrack.title }}</p>
          <p>{{ queuedTrack.albumTitle }}</p>
          <div class="row">
            <p style="flex-grow: 1">{{ queuedTrack.artist }}</p>
            <IconTrash class="icon icon-trash" style="cursor: pointer; margin-left: 5px" @click="queuedTracks.splice(i, 1)" />
            <IconLoader2 class="icon icon-loading" v-if="!queuedTrack.hasOwnProperty('status')" />
            <IconCircleCheck class="icon" color="var(--color-success)" v-else-if="queuedTrack.status" />
            <IconAlertCircle @click="queuedTrack.showStatusMessage = !queuedTrack.showStatusMessage" class="icon" style="cursor: pointer" color="var(--color-error)" v-else />
          </div>
          <div class="status-menu" v-show="queuedTrack.showStatusMessage">
            {{ queuedTrack.statusMsg }}
          </div>
        </div>
      </div>
      <div class="row footer">
        <IconFolder v-tooltip="'Open Downloads'" @click="invoke('browse_cmd', { path: appConfig.downloadPath })" size="30" style="cursor: pointer" class="icon" />
        <IconTrash v-tooltip="'Clear Queue'" @click="queuedTracks = []" size="30" style="cursor: pointer" class="icon" />
      </div>
    </div>
  </div>
</template>

<style scoped>
.container {
  height: calc(100vh - 70px);
}

.frame {
  overflow-y: auto;
  overflow-x: hidden;
  padding-left: 10px;
  padding-right: 10px;
  padding-top: 0px;
  padding-bottom: 0px;
}

.download-expand-btn-expanded {
  transform: rotate(180deg);
}

input {
  flex-grow: 1;
}

.slavart-tracks-container .table-header {
  font-style: italic;
  background-color: var(--color-bg-1);
  position: sticky;
  top: 0;
  z-index: 1;
}

.slavart-tracks-container .table-header th {
  font-weight: 500;
  border-bottom: 1px solid var(--color-bg-2);
  text-wrap: nowrap;
}

.queued-tracks-container .footer {
  margin-top: auto;
  justify-content: flex-end;
  padding: 5px;
  border-top: 1px solid var(--color-bg-2);
  position: sticky;
  bottom: 0px;
  background-color: var(--color-bg-1);
  transition: all 0.2s ease;
}

.slavart-tracks-container table {
  width: 100%;
  border-collapse: separate;
  border-spacing: 0px 4px;
}

.slavart-tracks-container tbody {
  font-size: 0.95em;
}

.slavart-tracks-container .table-row {
  transition: background-color 0.2s ease-in-out;
}

.slavart-tracks-container .table-row:hover {
  background-color: var(--color-hover);
}

.slavart-tracks-container .img-container {
  width: 50px;
  min-width: 50px;
  max-width: 50px;
  margin-left: 20px;
  margin-right: 20px;
  margin-top: auto;
  margin-bottom: auto;
  padding-top: 5px;
  padding-bottom: 5px;
  position: relative;
  cursor: pointer;
  user-select: none;
}

.slavart-tracks-container .img-darken {
  opacity: 100%;
}

.slavart-tracks-container .img-container:hover > .img-darken {
  transition: opacity 0.2s ease;
  opacity: 50%;
}

.slavart-tracks-container img {
  display: block;
  border-radius: 8px;
}

.slavart-tracks-container .table-row td {
  max-width: 10px;
  overflow: scroll;
  white-space: nowrap;
  -ms-overflow-style: none;
}

.slavart-tracks-container .table-row td::-webkit-scrollbar {
  display: none;
}

.slavart-tracks-container .text-pad {
  padding-left: 10px;
}

.slavart-tracks-container .icon-download {
  opacity: 0%;
  position: absolute;
  top: 28px;
  left: 26px;
  transform: translate(-50%, -50%);
  z-index: 1;
}

.slavart-tracks-container .img-container:hover > .icon-download {
  transition: opacity 0.2s ease;
  opacity: 100%;
}

.queued-tracks-container {
  width: 0px;
  display: none;
  flex-direction: column;
  flex-shrink: 0;
}

.queued-tracks-container .queued-track {
  font-size: 14px;
  display: flex;
  flex-direction: column;
  min-height: 60px;
  padding-top: 10px;
  padding-left: 2px;
  padding-bottom: 5px;
  border-bottom: 1px solid var(--color-bg-2);
}

.queued-tracks-container .queued-track p {
  text-align: left;
  margin-top: 2px;
  margin-bottom: 0px;
  margin-right: 2px;
  padding-left: 5px;
  padding-right: 5px;
  overflow: scroll;
  white-space: nowrap;
  -ms-overflow-style: none;
}

.queued-tracks-container p::-webkit-scrollbar {
  display: none;
}

.queued-tracks-container img {
  border-radius: 10px;
  padding: 5px;
  user-select: none;
}

.queued-tracks-container .status-menu {
  text-align: left;
  margin-top: 5px;
  margin-bottom: 5px;
  padding: 5px;
  overflow-wrap: break-word;
  background-color: var(--color-bg-2);
  border: 1px solid var(--color-accent);
  border-radius: 5px;
}

.queued-tracks-container .icon-loading {
  animation: icon-loading-anim 1.8s linear infinite;
}

.queued-tracks-container .icon-trash {
  opacity: 0%;
  transition: opacity 0.2s ease;
}

.queued-track:hover .icon-trash {
  opacity: 100%;
}

.queued-tracks-container.expanded {
  display: flex !important;
  width: 200px !important;
}

.queued-tracks-container::-webkit-scrollbar {
  width: 8px;
}
</style>

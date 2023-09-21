<script setup>
import { ref, onMounted, computed } from "vue";
import { invoke } from "@tauri-apps/api";
import { appWindow } from "@tauri-apps/api/window";
import { open } from "@tauri-apps/api/dialog";
import { IconDotsVertical, IconFolder, IconClipboardList, IconDeviceFloppy, IconProgress, IconProgressAlert, IconProgressBolt, IconProgressHelp, IconProgressCheck, IconMusic, IconFile } from "@tabler/icons-vue";

import { appConfig, filterColumnsDirView, globalEmitter } from "../helpers";

const dzrsTracks = ref([{}]);
const dzrsFiles = ref([{}]);
const selectedDzrsFilePaths = ref([]);
const activeDzrsFilePath = computed(() => selectedDzrsFilePaths.value.length >= 1 ? selectedDzrsFilePaths.value[selectedDzrsFilePaths.value.length - 1] : false);
const activeDzrsTrack = computed(() => {
  if (activeDzrsFilePath.value) {
    const track = dzrsTracks.value.find((track) => track.path === activeDzrsFilePath.value);
    if (track) {
      return track
    } else {
      return false
    };
  } else {
    return false
  };
});
const showFilterMenu = ref(false);
const currentWatchedPath = ref(appConfig.directoryViewPath);
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
  const path = await open({ defaultPath: appConfig.directoryViewPath, directory: true, multiple: false })
    .then((res) => res)
    .catch((err) => globalEmitter.emit("notification-add", { type: "Error", origin: "updateViewPath", msg: err }));
  if (path !== null) {
    currentWatchedPath.value = path;
    await loadFilesIntoView();
    await getDzrsTracks();
    await invoke("watch_directory", { path: path })
      .catch((err) => globalEmitter.emit("notification-add", { type: "Error", origin: "updateViewPath", msg: err }));
  };
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
  const flacs = dzrsFiles.value.filter((file) => file.extension === "flac");
  const flac_paths = flacs.map((f) => f.path);
  const result = await invoke("get_dzrs_tracks", { paths: flac_paths, clearStored: true, getDeezerTags: false })
    .then((res) => res)
    .catch((err) => globalEmitter.emit("notification-add", { type: "Error", origin: "getDzrsTracksFromSelection", msg: err }));
  dzrsTracks.value = result.items;
  updateDzrsFilesStatus();
};

async function getDzrsTracksFromSelection() {
  let flacs;
  if (selectedDzrsFilePaths.value.length === 0) {
    flacs = dzrsFiles.value.filter((file) => file.extension === "flac");
  } else {
    flacs = dzrsFiles.value.filter((file) => file.extension === "flac" && selectedDzrsFilePaths.value.includes(file.path));
  };
  const flac_paths = flacs.map((f) => f.path);
  const result = await invoke("get_dzrs_tracks", { paths: flac_paths, clearStored: false, getDeezerTags: true })
    .then((res) => res)
    .catch((err) => globalEmitter.emit("notification-add", { type: "Error", origin: "getDzrsTracksFromSelection", msg: err }));
  dzrsTracks.value = result.items;
  updateDzrsFilesStatus();
};

function updateDzrsFilesStatus() {
  const pathsAndStatuses = dzrsTracks.value.map((track) => {
    let status;
    if (track.saved) {
      status = "Finalized"
    } else if (track.matched) {
      status = "Matched"
    } else if (track.fetched) {
      status = "Successfull"
    } else if (track.fetched && !track.candidates) {
      status = "Unsuccessfull"
    } else {
      status = "NotFetched"
    }
    return [track.path, status]
  });
  pathsAndStatuses.forEach((v) => {
    const i = dzrsFiles.value.findIndex((file) => file.path === v[0]);
    if (i !== -1) {
      dzrsFiles.value[i].tagStatus = v[1]
    }
  });
};

onMounted(async () => {
  await loadFilesIntoView();
  await getDzrsTracks();
  appWindow.listen("watcher_fired", async (_) => {
    await loadFilesIntoView();
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
          <button style="padding: 2px 8px;" @click="getDzrsTracksFromSelection">
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
                <th><!-- Reserved for icon --></th>
                <th v-for="column in filterColumnsDirView" :key="column.key" v-show="column.enabled">{{ column.label }}</th>
              </tr>
            </thead>
            <tbody>
              <template v-for="file in dzrsFiles" :key="file.path">
                <tr @click="selectFiles($event, file)" :class="{ 'selected-file': selectedDzrsFilePaths.includes(file.path) }">
                  <td><!-- Empty cell reserved for table filter --></td>
                  <td class="img-container">
                    <IconMusic v-if="['flac', 'mp3'].includes(file.extension)"/>
                    <IconFile v-else/>
                    <!-- <img :src="iconPathFromExtension(file.extension)" class="icon"> -->
                  </td>
                  <td v-show="filterColumnsDirView.find((col) => col.key === 'filename' && col.enabled)" style="text-align: left; font-style: italic;">{{ file.filename }}</td>
                  <td v-show="filterColumnsDirView.find((col) => col.key === 'size' && col.enabled)">{{ (file.size / (1024 * 1024)).toFixed(1) }} MB</td>
                  <td v-show="filterColumnsDirView.find((col) => col.key === 'extension' && col.enabled)">{{ file.extension }}</td>
                  <td v-show="filterColumnsDirView.find((col) => col.key === 'tagStatus' && col.enabled)">
                    <IconProgressCheck v-if="file.tagStatus === 'Finalized'"/>
                    <IconProgressBolt v-else-if="file.tagStatus === 'Matched'"/>
                    <IconProgressHelp v-else-if="file.tagStatus === 'Successfull'"/>
                    <IconProgressAlert v-else-if="file.tagStatus === 'Unsuccessfull'"/>
                    <IconProgress v-else/>
                  </td>
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
      <div class="frame row" style="gap: 4px;">
        <div class="image-tag column">
          <div class="column">
            <div v-if="activeDzrsTrack" v-for="picture in activeDzrsTrack.pictures">
              <img :src="`data:image/png;base64, ${picture.b64}`" style="border-radius: 5%;">
              <p>{{ picture.picType }}</p>
              <p>{{ picture.description }}</p>
              <p>{{ picture.width }}x{{ picture.height }}</p>
            </div>
            <div v-else>
              <img src="/assets/tag-image-placeholder.png">
              <p style="font-style: italic;">Cover</p>
            </div>
          </div>
        </div>
        <div class="column" style="flex-grow: 1;">
          <div class="column" style="flex-basis: 400px; overflow-y: auto; justify-content: start;">
            <table>
              <thead class="table-header">
                <tr>
                  <th style="width: 12%;">Tag</th>
                  <th>Current Value</th>
                  <th>Future Value</th>
                </tr>
              </thead>
              <tbody :set="hasTags = activeDzrsTrack.hasOwnProperty('tags') ? true : false">
                <tr>
                  <th>Title</th>
                  <td>{{ hasTags ? activeDzrsTrack.tags.title : "" }}</td>
                  <td><input type="text" :value="hasTags ? activeDzrsTrack.tagsDeezer.title : ''"></td>
                </tr>
                <tr>
                  <th>Artist</th>
                  <td>{{ hasTags ? activeDzrsTrack.tags.artist : "" }}</td>
                  <td><input type="text" :value="hasTags ? activeDzrsTrack.tagsDeezer.artist : ''"></td>
                </tr>
                <tr>
                  <th>Artists</th>
                  <td>{{ hasTags ? activeDzrsTrack.tags.artists : "" }}</td>
                  <td><input type="text" :value="hasTags ? activeDzrsTrack.tagsDeezer.artists : ''"></td>
                </tr>
                <tr>
                  <th>Album</th>
                  <td>{{ hasTags ? activeDzrsTrack.tags.album : "" }}</td>
                  <td><input type="text" :value="hasTags ? activeDzrsTrack.tagsDeezer.album : ''"></td>
                </tr>
                <tr>
                  <th>Album Artist</th>
                  <td>{{ hasTags ? activeDzrsTrack.tags.albumArtist : "" }}</td>
                  <td><input type="text" :value="hasTags ? activeDzrsTrack.tagsDeezer.albumArtist : ''"></td>
                </tr>
                <tr>
                  <th>Composer</th>
                  <td>{{ hasTags ? activeDzrsTrack.tags.composer : "" }}</td>
                  <td><input type="text" :value="hasTags ? activeDzrsTrack.tagsDeezer.composer : ''"></td>
                </tr>
                <tr>
                  <th>Performer</th>
                  <td>{{ hasTags ? activeDzrsTrack.tags.performer : "" }}</td>
                  <td><input type="text" :value="hasTags ? activeDzrsTrack.tagsDeezer.performer : ''"></td>
                </tr>
                <tr>
                  <th>Producer</th>
                  <td>{{ hasTags ? activeDzrsTrack.tags.producer : "" }}</td>
                  <td><input type="text" :value="hasTags ? activeDzrsTrack.tagsDeezer.producer : ''"></td>
                </tr>
                <tr>
                  <th>Genre</th>
                  <td>{{ hasTags ? activeDzrsTrack.tags.genre : "" }}</td>
                  <td><input type="text" :value="hasTags ? activeDzrsTrack.tagsDeezer.genre : ''"></td>
                </tr>
                <tr>
                  <th>Lyrics</th>
                  <td>{{ hasTags ? activeDzrsTrack.tags.lyrics : "" }}</td>
                  <td><input type="text" :value="hasTags ? activeDzrsTrack.tagsDeezer.lyrics : ''"></td>
                </tr>
                <tr>
                  <th>Copyright</th>
                  <td>{{ hasTags ? activeDzrsTrack.tags.copyright : "" }}</td>
                  <td><input type="text" :value="hasTags ? activeDzrsTrack.tagsDeezer.copyright : ''"></td>
                </tr>
                <tr>
                  <th>Description</th>
                  <td>{{ hasTags ? activeDzrsTrack.tags.description : "" }}</td>
                  <td><input type="text" :value="hasTags ? activeDzrsTrack.tagsDeezer.description : ''"></td>
                </tr>
                <tr>
                  <th>Track Number</th>
                  <td>{{ hasTags ? activeDzrsTrack.tags.trackNumber : "" }}</td>
                  <td><input type="text" :value="hasTags ? activeDzrsTrack.tagsDeezer.trackNumber : ''"></td>
                </tr>
                <tr>
                  <th>Total Tracks</th>
                  <td>{{ hasTags ? activeDzrsTrack.tags.trackTotal : "" }}</td>
                  <td><input type="text" :value="hasTags ? activeDzrsTrack.tagsDeezer.trackTotal : ''"></td>
                </tr>
                <tr>
                  <th>Disc Number</th>
                  <td>{{ hasTags ? activeDzrsTrack.tags.discNumber : "" }}</td>
                  <td><input type="text" :value="hasTags ? activeDzrsTrack.tagsDeezer.discNumber : ''"></td>
                </tr>
                <tr>
                  <th>Total Discs</th>
                  <td>{{ hasTags ? activeDzrsTrack.tags.discTotal : "" }}</td>
                  <td><input type="text" :value="hasTags ? activeDzrsTrack.tagsDeezer.discTotal : ''"></td>
                </tr>
                <tr>
                  <th>Length</th>
                  <td></td>
                  <td><input type="text"></td>
                </tr>
                <tr>
                  <th>Date</th>
                  <td>{{ hasTags ? activeDzrsTrack.tags.date : "" }}</td>
                  <td><input type="text" :value="hasTags ? activeDzrsTrack.tagsDeezer.date : ''"></td>
                </tr>
                <tr>
                  <th>Year</th>
                  <td>{{ hasTags ? activeDzrsTrack.tags.year : "" }}</td>
                  <td><input type="text" :value="hasTags ? activeDzrsTrack.tagsDeezer.year : ''"></td>
                </tr>
                <tr>
                  <th>Original Date</th>
                  <td>{{ hasTags ? activeDzrsTrack.tags.originalDate : "" }}</td>
                  <td><input type="text" :value="hasTags ? activeDzrsTrack.tagsDeezer.originalDate : ''"></td>
                </tr>
                <tr>
                  <th>Original Year</th>
                  <td>{{ hasTags ? activeDzrsTrack.tags.originalYear : "" }}</td>
                  <td><input type="text" :value="hasTags ? activeDzrsTrack.tagsDeezer.originalYear : ''"></td>
                </tr>
                <tr>
                  <th>Comment</th>
                  <td>{{ hasTags ? activeDzrsTrack.tags.comment : "" }}</td>
                  <td><input type="text" :value="hasTags ? activeDzrsTrack.tagsDeezer.comment : ''"></td>
                </tr>
                <tr>
                  <th>Label</th>
                  <td>{{ hasTags ? activeDzrsTrack.tags.label : "" }}</td>
                  <td><input type="text" :value="hasTags ? activeDzrsTrack.tagsDeezer.label : ''"></td>
                </tr>
                <tr>
                  <th>Barcode</th>
                  <td>{{ hasTags ? activeDzrsTrack.tags.barcode : "" }}</td>
                  <td><input type="text" :value="hasTags ? activeDzrsTrack.tagsDeezer.barcode : ''"></td>
                </tr>
                <tr>
                  <th>ISRC</th>
                  <td>{{ hasTags ? activeDzrsTrack.tags.isrc : "" }}</td>
                  <td><input type="text" :value="hasTags ? activeDzrsTrack.tagsDeezer.isrc : ''"></td>
                </tr>
                <tr>
                  <th>BPM</th>
                  <td>{{ hasTags ? activeDzrsTrack.tags.bpm : "" }}</td>
                  <td><input type="text" :value="hasTags ? activeDzrsTrack.tagsDeezer.bpm : ''"></td>
                </tr>
                <tr>
                  <th>Replaygain Track Gain</th>
                  <td>{{ hasTags ? activeDzrsTrack.tags.replaygainTrackGain : "" }}</td>
                  <td><input type="text" :value="hasTags ? activeDzrsTrack.tagsDeezer.replaygainTrackGain : ''"></td>
                </tr>
                <tr>
                  <th>Replaygain Track Peak</th>
                  <td>{{ hasTags ? activeDzrsTrack.tags.replaygainTrackPeak : "" }}</td>
                  <td><input type="text" :value="hasTags ? activeDzrsTrack.tagsDeezer.replaygainTrackPeak : ''"></td>
                </tr>
                <tr>
                  <th>Replaygain Album Gain</th>
                  <td>{{ hasTags ? activeDzrsTrack.tags.replaygainAlbumGain : "" }}</td>
                  <td><input type="text" :value="hasTags ? activeDzrsTrack.tagsDeezer.replaygainAlbumGain : ''"></td>
                </tr>
                <tr>
                  <th>Replaygain Album Peak</th>
                  <td>{{ hasTags ? activeDzrsTrack.tags.replaygainAlbumPeak : "" }}</td>
                  <td><input type="text" :value="hasTags ? activeDzrsTrack.tagsDeezer.replaygainAlbumPeak : ''"></td>
                </tr>
                <tr>
                  <th>Encoder</th>
                  <td>{{ hasTags ? activeDzrsTrack.tags.encoder : "" }}</td>
                  <td><input type="text" :value="hasTags ? activeDzrsTrack.tagsDeezer.encoder : ''"></td>
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

.directory-panel .frame {
  padding-top: 0px;
  height: 100px;
  overflow-y: auto;
}

.tags-panel {
  flex-grow: 0;
}

.image-tag {
  flex-basis: 150px;
  flex-grow: 0;
  flex-shrink: 0;
  border-right: 1px solid var(--color-bg-2);
  padding-right: 4px;
  padding-top: 5px;
  padding-bottom: 5px;
}

.image-tag .column {
  flex-basis: 400px;
  justify-content: start;
  align-items: center;
  overflow-y: auto;
  overflow-x: hidden;
}

.image-tag img {
  width: 140px;
  user-select: none;
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

.tags-panel table {
  table-layout: fixed;
}

.img-container, .img-container * {
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

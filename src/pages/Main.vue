<script setup>
import { ref, onMounted, computed } from "vue";
import { invoke } from "@tauri-apps/api";
import { appWindow } from "@tauri-apps/api/window";
import { open, confirm } from "@tauri-apps/api/dialog";
import { isEqual } from 'lodash';
import { IconPointFilled, IconLoader2, IconDotsVertical, IconFolder, IconClipboardList, IconDeviceFloppy, IconProgress, IconProgressAlert, IconProgressBolt, IconProgressHelp, IconProgressCheck, IconMusic, IconFile, IconRestore } from "@tabler/icons-vue";

import { appConfig, filterColumnsDirView, globalEmitter, openFileBrowser, emptyTrack } from "../helpers";

const dzrsTracks = ref([{}]);
const dzrsFiles = ref([{}]);
const selectedDzrsFilePaths = ref([]);
const activeDzrsFilePath = computed(() => selectedDzrsFilePaths.value.length >= 1 ? selectedDzrsFilePaths.value[selectedDzrsFilePaths.value.length - 1] : false);
const showFilterMenu = ref(false);
const currentWatchedPath = ref(appConfig.directoryViewPath);
const tagsFetching = ref(false);
const activeDzrsTrack = computed(() => {
  if (activeDzrsFilePath.value) {
    const track = dzrsTracks.value.find((track) => track.path === activeDzrsFilePath.value);
    if (track) {
      return track;
    };
    return emptyTrack;
  };
  return emptyTrack;
});
const tagsNeedSave = computed(() => {
  const found = dzrsTracks.value.find((t) => !isEqual(t.tags, t.tagsToSave));
  if (found) {
    return true;
  } else {
    return false;
  };
});

async function loadFilesIntoView() {
  const result = await invoke("watcher_get_files", { path: currentWatchedPath.value })
    .then((res) => res)
    .catch((err) => globalEmitter.emit("notification-add", { type: "Error", origin: "watcher_get_files", msg: err }));
  if (result) {
    dzrsFiles.value = result.items;
  };
}

async function openLocalFilesExplorer() {
  await openFileBrowser(appConfig.directoryViewPath)
    .catch((err) => globalEmitter.emit("notification-add", { type: "Error", origin: "openLocalFilesExplorer", msg: err }));
}

async function updateViewPath() {
  const path = await open({ defaultPath: appConfig.directoryViewPath, directory: true, multiple: false })
    .then((res) => res)
    .catch((err) => globalEmitter.emit("notification-add", { type: "Error", origin: "updateViewPath", msg: err }));
  if (path !== null) {
    currentWatchedPath.value = path;
    await loadFilesIntoView();
    await getAllDzrsTracks();
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

async function getAllDzrsTracks() {
  const flacs = dzrsFiles.value.filter((file) => file.extension === "flac");
  const flac_paths = flacs.map((f) => f.path);
  const result = await invoke("get_all_dzrs_tracks", { paths: flac_paths, clearStored: true, getDeezerTags: false })
    .then((res) => res)
    .catch((err) => globalEmitter.emit("notification-add", { type: "Error", origin: "getDzrsTracksFromSelection", msg: err }));
  dzrsTracks.value = result.items;
  updateDzrsFilesStatus();
};

async function updateDzrsTracks(paths) {
  const tracks = await invoke("update_dzrs_tracks", { paths: paths, getDeezerTags: false })
    .then((res) => res)
    .catch((err) => globalEmitter.emit("notification-add", { type: "Error", origin: "updateDzrsTracks", msg: err }));
  tracks.forEach((t) => {
    const i = dzrsTracks.value.findIndex((_t) => _t.path === t.path);
    if (i !== -1) {
      dzrsTracks.value[i] = t;
    };
  });
};

async function getDzrsTracksFromSelection() {
  let flacs;
  tagsFetching.value = true;
  if (selectedDzrsFilePaths.value.length === 0) {
    flacs = dzrsFiles.value.filter((file) => file.extension === "flac");
  } else {
    flacs = dzrsFiles.value.filter((file) => file.extension === "flac" && selectedDzrsFilePaths.value.includes(file.path));
  };
  const flac_paths = flacs.map((f) => f.path);
  const result = await invoke("get_all_dzrs_tracks", { paths: flac_paths, clearStored: false, getDeezerTags: true })
    .then((res) => res)
    .catch((err) => globalEmitter.emit("notification-add", { type: "Error", origin: "getDzrsTracksFromSelection", msg: err }));
  dzrsTracks.value = result.items;
  updateDzrsFilesStatus();
  tagsFetching.value = false;
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

function showFileModifiedIcon(file) {
  const track = dzrsTracks.value.find((t) => t.path === file.path);
  if (track) {
    if (!isEqual(track.tags, track.tagsToSave)) {
      return true
    } else {
      return false
    };
  } else {
    return false
  };
};

async function saveModifiedTracks() {
  const modifiedTracks = dzrsTracks.value.filter((t) => !isEqual(t.tags, t.tagsToSave));
  const modifiedTracksPaths = modifiedTracks.map((t) => t.path);
  const confirmation = await confirm('Files will be saved with updated tags, are you sure?', { title: 'Save', type: 'warning' });
  if (confirmation) {
    if (selectedDzrsFilePaths.value.length >= 1) {
      // Save only selected files
      const savedPaths = [];
      for (const track of modifiedTracks) {
        if (selectedDzrsFilePaths.value.includes(track.path)) {
          await invoke("save_tags_to_file", { path: track.path, tags: track.tagsToSave })
          .catch((err) => globalEmitter.emit("notification-add", { type: "Error", origin: "saveModifiedTracks", msg: `${err} Path ${track.path}` }));
          savedPaths.push(track.path);
        };
      };
      updateDzrsTracks(savedPaths);
    } else {
      // Save all modified files instead
      for (const track of modifiedTracks) {
        await invoke("save_tags_to_file", { path: track.path, tags: track.tagsToSave })
          .catch((err) => globalEmitter.emit("notification-add", { type: "Error", origin: "saveModifiedTracks", msg: `${err} Path ${track.path}` }));
      };
      updateDzrsTracks(modifiedTracksPaths);
    };
  };
};

onMounted(async () => {
  await loadFilesIntoView();
  await getAllDzrsTracks();
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
          <button style="padding: 2px 8px;" @click="updateViewPath">
            <div class="row" style="color: var(--color-text);">
              <IconFolder size="20" color="var(--color-text)" class="icon" style="margin-right: 3px;"/>
              Open
            </div>
          </button>
          <p style="font-weight: 300; letter-spacing: 0.12em; padding: 2px 6px; margin-right: auto;" class="button">
            {{ currentWatchedPath ? currentWatchedPath : "..." }}
          </p>
          <button style="padding: 2px 8px; margin-left: auto;" @click="openLocalFilesExplorer" v-show="appConfig.directoryViewPath">
            <div class="row" style="color: var(--color-text);">
              Browse
              <IconFolder size="20" color="var(--color-text)" class="icon" style="margin-left: 3px;"/>
            </div>
          </button>
          <button style="padding: 2px 8px;" @click="getDzrsTracksFromSelection" :disabled="tagsFetching">
            <div class="row" style="color: var(--color-text);" v-tooltip.bottom="{ content: 'Retrieve Deezer Tags' }">
              Fetch
              <IconClipboardList v-if="!tagsFetching" size="20" color="var(--color-text)" class="icon" style="margin-left: 3px;"/>
              <IconLoader2 v-else size="20" color="var(--color-text)" class="icon icon-loading" style="margin-left: 3px;"/>
            </div>
          </button>
          <button style="padding: 2px 8px;" @click="saveModifiedTracks" :disabled="!tagsNeedSave">
            <div class="row" style="color: var(--color-text);">
              Save
              <IconDeviceFloppy size="20" color="var(--color-text)" class="icon" style="margin-left: 3px;"/>
            </div>
          </button>
        </div>
        <div class="frame" @click.self="selectedDzrsFilePaths = []">
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
                  <td>
                    <IconPointFilled v-if="showFileModifiedIcon(file)"/>
                  </td>
                  <td class="img-container">
                    <IconMusic v-if="['flac', 'mp3'].includes(file.extension)" color="#c9c9c9"/>
                    <IconFile v-else color="#c9c9c9"/>
                  </td>
                  <td v-show="filterColumnsDirView.find((col) => col.key === 'filename' && col.enabled)" style="text-align: left; font-style: italic;">{{ file.filename }}</td>
                  <td v-show="filterColumnsDirView.find((col) => col.key === 'size' && col.enabled)">{{ (file.size / (1024 * 1024)).toFixed(1) }} MB</td>
                  <td v-show="filterColumnsDirView.find((col) => col.key === 'extension' && col.enabled)">{{ file.extension }}</td>
                  <td v-show="filterColumnsDirView.find((col) => col.key === 'tagStatus' && col.enabled)">
                    <IconProgressCheck v-if="file.tagStatus === 'Finalized'" color="var(--color-success)" v-tooltip="'File Saved'"/>
                    <IconProgressBolt v-else-if="file.tagStatus === 'Matched'" color="#578867" v-tooltip="'Good Match Applied'"/>
                    <IconProgressHelp v-else-if="file.tagStatus === 'Successfull'" color="#998f40" v-tooltip="'Multiple Candidates Found'"/>
                    <IconProgressAlert v-else-if="file.tagStatus === 'Unsuccessfull'" color="var(--color-error)" v-tooltip="'No Tags Found'"/>
                    <IconProgress v-else color="#8c8c8c" v-tooltip="'Tags not Fetched'"/>
                  </td>
                </tr>
              </template>
            </tbody>
          </table>
        </div>
      </div>
      <div class="source-panel frame">
        <div class="sources-header">
          <p style="width: 100%; margin-top: 5px; margin-bottom: 0px; padding-bottom: 5px; border-bottom: 1px solid var(--color-bg-2);">Sources</p>
        </div>
        <div v-for="(i, source) in activeDzrsTrack.tagCandidates" :key="i"></div>
        <div class="row sources-footer">
            <IconRestore v-tooltip="'Restore Original Tags'" @click="" size="25" style="cursor: pointer;" class="icon"/>
        </div>
      </div>
    </div>
    <div class="tags-panel">
      <div class="frame row" style="gap: 4px;">
        <div class="image-tag column">
          <div class="column">
            <div v-if="activeDzrsTrack !== emptyTrack" v-for="picture in activeDzrsTrack.pictures" style="margin-bottom: 4px;">
              <img :src="`data:image/png;base64, ${picture.b64}`" style="border-radius: 5%;">
              <p>{{ picture.picType }}</p>
              <p style="font-size: 0.8em;">{{ picture.description }}</p>
              <p style="font-style: italic; font-size: 0.8em;">{{ picture.width }}x{{ picture.height }}</p>
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
              <tbody>
                <tr>
                  <th>Title</th>
                  <td><div><textarea spellcheck="false" type="text" readonly>{{ activeDzrsTrack.tags.title }}</textarea></div></td>
                  <td><div><textarea spellcheck="false" type="text" :class="{'tag-yellow-text': activeDzrsTrack.tagsToSave.title !== activeDzrsTrack.tags.title}" v-model="activeDzrsTrack.tagsToSave.title"></textarea></div></td>
                </tr>
                <tr>
                  <th>Artist</th>
                  <td><div><textarea spellcheck="false" type="text" readonly>{{ activeDzrsTrack.tags.artist }}</textarea></div></td>
                  <td><div><textarea spellcheck="false" type="text" :class="{'tag-yellow-text': activeDzrsTrack.tagsToSave.artist !== activeDzrsTrack.tags.artist}" v-model="activeDzrsTrack.tagsToSave.artist"></textarea></div></td>
                </tr>
                <tr>
                  <th>Album</th>
                  <td><div><textarea spellcheck="false" type="text" readonly>{{ activeDzrsTrack.tags.album }}</textarea></div></td>
                  <td><div><textarea spellcheck="false" type="text" :class="{'tag-yellow-text': activeDzrsTrack.tagsToSave.album !== activeDzrsTrack.tags.album}" v-model="activeDzrsTrack.tagsToSave.album"></textarea></div></td>
                </tr>
                <tr>
                  <th>Album Artist</th>
                  <td><div><textarea spellcheck="false" type="text" readonly>{{ activeDzrsTrack.tags.albumArtist }}</textarea></div></td>
                  <td><div><textarea spellcheck="false" type="text" :class="{'tag-yellow-text': activeDzrsTrack.tagsToSave.albumArtist !== activeDzrsTrack.tags.albumArtist}" v-model="activeDzrsTrack.tagsToSave.albumArtist"></textarea></div></td>
                </tr>
                <tr>
                  <th>Composer</th>
                  <td><div><textarea spellcheck="false" type="text" readonly>{{ activeDzrsTrack.tags.composer }}</textarea></div></td>
                  <td><div><textarea spellcheck="false" type="text" :class="{'tag-yellow-text': activeDzrsTrack.tagsToSave.composer !== activeDzrsTrack.tags.composer}" v-model="activeDzrsTrack.tagsToSave.composer"></textarea></div></td>
                </tr>
                <tr>
                  <th>Performer</th>
                  <td><div><textarea spellcheck="false" type="text" readonly>{{ activeDzrsTrack.tags.performer }}</textarea></div></td>
                  <td><div><textarea spellcheck="false" type="text" :class="{'tag-yellow-text': activeDzrsTrack.tagsToSave.performer !== activeDzrsTrack.tags.performer}" v-model="activeDzrsTrack.tagsToSave.performer"></textarea></div></td>
                </tr>
                <tr>
                  <th>Producer</th>
                  <td><div><textarea spellcheck="false" type="text" readonly>{{ activeDzrsTrack.tags.producer }}</textarea></div></td>
                  <td><div><textarea spellcheck="false" type="text" :class="{'tag-yellow-text': activeDzrsTrack.tagsToSave.producer !== activeDzrsTrack.tags.producer}" v-model="activeDzrsTrack.tagsToSave.producer"></textarea></div></td>
                </tr>
                <tr>
                  <th>Genre</th>
                  <td><div><textarea spellcheck="false" type="text" readonly>{{ activeDzrsTrack.tags.genre }}</textarea></div></td>
                  <td><div><textarea spellcheck="false" type="text" :class="{'tag-yellow-text': activeDzrsTrack.tagsToSave.genre !== activeDzrsTrack.tags.genre}" v-model="activeDzrsTrack.tagsToSave.genre"></textarea></div></td>
                </tr>
                <tr style="height: 300px;">
                  <th>Lyrics</th>
                  <td><div><textarea spellcheck="false" type="text" readonly>{{ activeDzrsTrack.tags.lyrics }}</textarea></div></td>
                  <td><div><textarea spellcheck="false" type="text" :class="{'tag-yellow-text': activeDzrsTrack.tagsToSave.lyrics !== activeDzrsTrack.tags.lyrics}" v-model="activeDzrsTrack.tagsToSave.lyrics"></textarea></div></td>
                </tr>
                <tr style="height: 80px;">
                  <th>Copyright</th>
                  <td><div><textarea spellcheck="false" type="text" readonly>{{ activeDzrsTrack.tags.copyright }}</textarea></div></td>
                  <td><div><textarea spellcheck="false" type="text" :class="{'tag-yellow-text': activeDzrsTrack.tagsToSave.copyright !== activeDzrsTrack.tags.copyright}" v-model="activeDzrsTrack.tagsToSave.copyright"></textarea></div></td>
                </tr>
                <tr>
                  <th style="height: 80px;">Description</th>
                  <td><div><textarea spellcheck="false" type="text" readonly>{{ activeDzrsTrack.tags.description }}</textarea></div></td>
                  <td><div><textarea spellcheck="false" type="text" :class="{'tag-yellow-text': activeDzrsTrack.tagsToSave.description !== activeDzrsTrack.tags.description}" v-model="activeDzrsTrack.tagsToSave.description"></textarea></div></td>
                </tr>
                <tr>
                  <th>Track Number</th>
                  <td><div><textarea spellcheck="false" type="text" readonly>{{ activeDzrsTrack.tags.trackNumber }}</textarea></div></td>
                  <td><div><textarea spellcheck="false" type="text" :class="{'tag-yellow-text': activeDzrsTrack.tagsToSave.trackNumber !== activeDzrsTrack.tags.trackNumber}" v-model="activeDzrsTrack.tagsToSave.trackNumber"></textarea></div></td>
                </tr>
                <tr>
                  <th>Total Tracks</th>
                  <td><div><textarea spellcheck="false" type="text" readonly>{{ activeDzrsTrack.tags.trackTotal }}</textarea></div></td>
                  <td><div><textarea spellcheck="false" type="text" :class="{'tag-yellow-text': activeDzrsTrack.tagsToSave.trackTotal !== activeDzrsTrack.tags.trackTotal}" v-model="activeDzrsTrack.tagsToSave.trackTotal"></textarea></div></td>
                </tr>
                <tr>
                  <th>Disk Number</th>
                  <td><div><textarea spellcheck="false" type="text" readonly>{{ activeDzrsTrack.tags.diskNumber }}</textarea></div></td>
                  <td><div><textarea spellcheck="false" type="text" :class="{'tag-yellow-text': activeDzrsTrack.tagsToSave.diskNumber !== activeDzrsTrack.tags.diskNumber}" v-model="activeDzrsTrack.tagsToSave.diskNumber"></textarea></div></td>
                </tr>
                <tr>
                  <th>Total Disks</th>
                  <td><div><textarea spellcheck="false" type="text" readonly>{{ activeDzrsTrack.tags.diskTotal }}</textarea></div></td>
                  <td><div><textarea spellcheck="false" type="text" :class="{'tag-yellow-text': activeDzrsTrack.tagsToSave.diskTotal !== activeDzrsTrack.tags.diskTotal}" v-model="activeDzrsTrack.tagsToSave.diskTotal"></textarea></div></td>
                </tr>
                <tr>
                  <th>Length</th>
                  <td><div><textarea spellcheck="false" type="text" readonly></textarea></div></td>
                  <td><div><textarea spellcheck="false" type="text"></textarea></div></td>
                </tr>
                <tr>
                  <th>Date</th>
                  <td><div><textarea spellcheck="false" type="text" readonly>{{ activeDzrsTrack.tags.date }}</textarea></div></td>
                  <td><div><textarea spellcheck="false" type="text" :class="{'tag-yellow-text': activeDzrsTrack.tagsToSave.date !== activeDzrsTrack.tags.date}" v-model="activeDzrsTrack.tagsToSave.date"></textarea></div></td>
                </tr>
                <tr>
                  <th>Year</th>
                  <td><div><textarea spellcheck="false" type="text" readonly>{{ activeDzrsTrack.tags.year }}</textarea></div></td>
                  <td><div><textarea spellcheck="false" type="text" :class="{'tag-yellow-text': activeDzrsTrack.tagsToSave.year !== activeDzrsTrack.tags.year}" v-model="activeDzrsTrack.tagsToSave.year"></textarea></div></td>
                </tr>
                <tr>
                  <th>Original Date</th>
                  <td><div><textarea spellcheck="false" type="text" readonly>{{ activeDzrsTrack.tags.originalDate }}</textarea></div></td>
                  <td><div><textarea spellcheck="false" type="text" :class="{'tag-yellow-text': activeDzrsTrack.tagsToSave.originalDate !== activeDzrsTrack.tags.originalDate}" v-model="activeDzrsTrack.tagsToSave.originalDate"></textarea></div></td>
                </tr>
                <tr>
                  <th>Comment</th>
                  <td><div><textarea spellcheck="false" type="text" readonly>{{ activeDzrsTrack.tags.comment }}</textarea></div></td>
                  <td><div><textarea spellcheck="false" type="text" :class="{'tag-yellow-text': activeDzrsTrack.tagsToSave.comment !== activeDzrsTrack.tags.comment}" v-model="activeDzrsTrack.tagsToSave.comment"></textarea></div></td>
                </tr>
                <tr>
                  <th>Label</th>
                  <td><div><textarea spellcheck="false" type="text" readonly>{{ activeDzrsTrack.tags.label }}</textarea></div></td>
                  <td><div><textarea spellcheck="false" type="text" :class="{'tag-yellow-text': activeDzrsTrack.tagsToSave.label !== activeDzrsTrack.tags.label}" v-model="activeDzrsTrack.tagsToSave.label"></textarea></div></td>
                </tr>
                <tr>
                  <th>Barcode</th>
                  <td><div><textarea spellcheck="false" type="text" readonly>{{ activeDzrsTrack.tags.barcode }}</textarea></div></td>
                  <td><div><textarea spellcheck="false" type="text" :class="{'tag-yellow-text': activeDzrsTrack.tagsToSave.barcode !== activeDzrsTrack.tags.barcode}" v-model="activeDzrsTrack.tagsToSave.barcode"></textarea></div></td>
                </tr>
                <tr>
                  <th>ISRC</th>
                  <td><div><textarea spellcheck="false" type="text" readonly>{{ activeDzrsTrack.tags.isrc }}</textarea></div></td>
                  <td><div><textarea spellcheck="false" type="text" :class="{'tag-yellow-text': activeDzrsTrack.tagsToSave.isrc !== activeDzrsTrack.tags.isrc}" v-model="activeDzrsTrack.tagsToSave.isrc"></textarea></div></td>
                </tr>
                <tr>
                  <th>BPM</th>
                  <td><div><textarea spellcheck="false" type="text" readonly>{{ activeDzrsTrack.tags.bpm }}</textarea></div></td>
                  <td><div><textarea spellcheck="false" type="text" :class="{'tag-yellow-text': activeDzrsTrack.tagsToSave.bpm !== activeDzrsTrack.tags.bpm}" v-model="activeDzrsTrack.tagsToSave.bpm"></textarea></div></td>
                </tr>
                <tr>
                  <th>RG Track Gain</th>
                  <td><div><textarea spellcheck="false" type="text" readonly>{{ activeDzrsTrack.tags.replaygainTrackGain }}</textarea></div></td>
                  <td><div><textarea spellcheck="false" type="text" :class="{'tag-yellow-text': activeDzrsTrack.tagsToSave.replaygainTrackGain !== activeDzrsTrack.tags.replaygainTrackGain}" v-model="activeDzrsTrack.tagsToSave.replaygainTrackGain"></textarea></div></td>
                </tr>
                <tr>
                  <th>RG Track Peak</th>
                  <td><div><textarea spellcheck="false" type="text" readonly>{{ activeDzrsTrack.tags.replaygainTrackPeak }}</textarea></div></td>
                  <td><div><textarea spellcheck="false" type="text" :class="{'tag-yellow-text': activeDzrsTrack.tagsToSave.replaygainTrackPeak !== activeDzrsTrack.tags.replaygainTrackPeak}" v-model="activeDzrsTrack.tagsToSave.replaygainTrackPeak"></textarea></div></td>
                </tr>
                <tr>
                  <th>RG Album Gain</th>
                  <td><div><textarea spellcheck="false" type="text" readonly>{{ activeDzrsTrack.tags.replaygainAlbumGain }}</textarea></div></td>
                  <td><div><textarea spellcheck="false" type="text" :class="{'tag-yellow-text': activeDzrsTrack.tagsToSave.replaygainAlbumGain !== activeDzrsTrack.tags.replaygainAlbumGain}" v-model="activeDzrsTrack.tagsToSave.replaygainAlbumGain"></textarea></div></td>
                </tr>
                <tr>
                  <th>RG Album Peak</th>
                  <td><div><textarea spellcheck="false" type="text" readonly>{{ activeDzrsTrack.tags.replaygainAlbumPeak }}</textarea></div></td>
                  <td><div><textarea spellcheck="false" type="text" :class="{'tag-yellow-text': activeDzrsTrack.tagsToSave.replaygainAlbumPeak !== activeDzrsTrack.tags.replaygainAlbumPeak}" v-model="activeDzrsTrack.tagsToSave.replaygainAlbumPeak"></textarea></div></td>
                </tr>
                <tr>
                  <th>Encoder</th>
                  <td><div><textarea spellcheck="false" type="text" readonly>{{ activeDzrsTrack.tags.encoder }}</textarea></div></td>
                  <td><div><textarea spellcheck="false" type="text" :class="{'tag-yellow-text': activeDzrsTrack.tagsToSave.encoder !== activeDzrsTrack.tags.encoder}" v-model="activeDzrsTrack.tagsToSave.encoder"></textarea></div></td>
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

.directory-panel table {
  width: 100%;
  border-collapse: separate;
  border-spacing: 0px 4px;
  user-select: none;
}

.directory-panel tbody {
  font-size: 0.95em;
  font-weight: 300;
}

.directory-panel tbody tr td:nth-child(1) {
  border-top-left-radius: 8px;
  border-bottom-left-radius: 8px;
}

.directory-panel tbody tr td:last-child {
  border-top-right-radius: 8px;
  border-bottom-right-radius: 8px;
}

.tags-panel tr td:nth-child(2) {
  user-select: text;
}

.directory-panel tbody tr:hover, .tags-panel tbody tr:hover, .selected-file {
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

.tags-panel tbody tr td {
  position: relative;
}

.tags-panel tbody tr td div {
  position: absolute;
  display: inline-block;
  top: 0;
  right: 0;
  bottom: 0;
  left: 0;
  padding-top: 4px;
  padding-bottom: 8px;
  padding-left: 4px;
  padding-right: 8px;
}

.tags-panel tbody textarea {
  width: 100%;
  height: 100%;
  background-color: var(--color-bg-1);
  border-radius: unset;
  border: unset;
  font-size: unset;
  font-family: inherit;
  resize:none;
}

.tags-panel tbody textarea:focus {
  outline: none;
}

.tags-panel tbody textarea::-webkit-scrollbar {
  width: 6px;
}

.tags-panel tbody textarea::-webkit-scrollbar-thumb {
    border-radius: 2px;
}

.tags-panel table {
  table-layout: fixed;
  padding-right: 5px;
  border-spacing: 0px 0px;
}

.tags-panel table tbody tr {
  height: 35px;
}

.sources-header {
  font-style: italic;
  position: sticky;
  top: 0;
}

.sources-footer {
  margin-top: auto;
  justify-content: flex-end;
  padding: 5px;
  border-top: 1px solid var(--color-bg-2);
  position: sticky;
  bottom: 0px;
  background-color: var(--color-bg-1);
  transition: all 0.2s ease;
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

.tag-yellow-text {
  color: #998f40;
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

::-webkit-scrollbar {
    width: 8px;
}
</style>

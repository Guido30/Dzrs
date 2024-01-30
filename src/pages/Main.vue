<script setup>
import { ref, computed, onBeforeMount } from "vue";
import { invoke } from "@tauri-apps/api";
import { appWindow } from "@tauri-apps/api/window";
import { open, confirm } from "@tauri-apps/api/dialog";
import { isEqual, remove as loRemove } from "lodash";
import { IconSearch, IconExternalLink, IconCloudDownload, IconPointFilled, IconLoader2, IconFolder, IconTagStarred, IconTag, IconDeviceFloppy, IconProgress, IconProgressAlert, IconProgressBolt, IconProgressHelp, IconProgressCheck, IconMusic, IconFile, IconRestore } from "@tabler/icons-vue";

import TableFilter from "../components/TableFilter.vue";
import HeaderBar from "../components/HeaderBar.vue";

import { appConfig, filterColumnsDirView, defaultDzrsTrackObject } from "../globals";

// Track objects
const dzrsTrackObjects = ref([]);
const activeDzrsTrackObject = computed(() => {
  let activeTrack = defaultDzrsTrackObject;
  if (selectedFilePaths.value.length >= 1) {
    const activeFilePath = selectedFilePaths.value[selectedFilePaths.value.length - 1];
    const track = dzrsTrackObjects.value.find((file) => file.filePath === activeFilePath);
    if (track) {
      activeTrack = track;
    }
  }
  return activeTrack;
});

// Elements
const inputFetchSources = ref(null);

// Dynamic variable, updated using selectFiles(), this maps to every selected TRACK_OBJ.filePath in the local files main panel
// used mostly for manipulating said files through invoking commands to the backend
const selectedFilePaths = ref([]);

// Used for displaying the user the currently open local files path, and as an argument for invoking commands
const activeLocalFilesPath = ref(appConfig.directoryViewPath);

// Visibility/interactivity toggles for various elements
const showFilterMenu = ref(false);
const tracksIsLoading = ref(false);
const tagsIsFetchingOrSaving = ref(false);
const tagsFetchingOrSavingEnabled = computed(() => {
  return activeDzrsTrackObject.value.fileExtension === "flac" ? true : false;
});
const tagsNeedSave = computed(() => {
  return dzrsTrackObjects.value.find((t) => !isEqual(t.tags, t.tagsToSave)) ? true : false;
});

// Obtain track objects currently loaded in the backend, all tracks will be loaded without providing paths
// this operation needs to be manually called after invoking manipulating commands over the track objects in the backend
// to ensure that they are always synchronized.
// Three ways of using this:
// 1 - getDzrsTrackObjects() : will get all tracks and fit the frontend to match
// 2 - getDzrsTrackObjects(paths) : will get only tracks from the provided paths and fit the frontend to match
// 3 - getDzrsTrackObjects(paths, remove) : will remove tracks from the provided paths and fit the frontend to match
// For performance reasons the more specific approaches 2 and 3 should be used, serializing and deserializing
// from backend to frontend is expensive especially with large amount of tracks
async function getDzrsTrackObjects(paths, remove) {
  // Remove tracks if requested then stop
  if (remove) {
    for (const p of paths) {
      await invoke("tracks_remove", { path: p }).catch((_) => _);
      const i = dzrsTrackObjects.value.findIndex((tr) => tr.filePath === p);
      if (i !== -1) {
        dzrsTrackObjects.value.splice(i, 1);
      }
    }
    return;
  }
  // Get the stored tracks
  const result = await invoke("tracks_get", { paths: paths ? paths : null })
    .then((res) => res)
    .catch((err) => appWindow.emit("notification-add", { type: "Error", origin: "getDzrsTrackObjects", msg: err }));
  if (result) {
    // Join tracks found in both frontend and backend
    result.forEach((newTr) => {
      const i = dzrsTrackObjects.value.findIndex((tr) => tr.filePath === newTr.filePath);
      if (i !== -1 && !isEqual(newTr, dzrsTrackObjects.value[i])) {
        dzrsTrackObjects.value[i] = newTr;
      } else if (i === -1) {
        dzrsTrackObjects.value.push(newTr);
      }
    });
    // Remove tracks in the frontend but NOT found in the backend
    if (!paths) {
      loRemove(dzrsTrackObjects.value, (tr) => !result.some((tr1) => tr1.filePath === tr.filePath));
    }
  }
}

// Obtain all track objects from a given directory and loads them in the backend, this operation clears all pre-exising objects and reassigns the new ones
async function getDzrsTrackObjectsDir() {
  tracksIsLoading.value = true;
  const result = await invoke("tracks_get_dir", { dir: activeLocalFilesPath.value })
    .then((res) => res)
    .catch((err) => appWindow.emit("notification-add", { type: "Error", origin: "getDzrsTrackObjectsDir", msg: err }));
  if (result) {
    dzrsTrackObjects.value = result;
  }
  tracksIsLoading.value = false;
}

// Called when setting a new local files directory from the main panel, the track objects have to be reassigned to match the files in the new directory
// a call is also made to the backend to instruct the watcher to watch the new directory
async function changeFilesDir() {
  const path = await open({ defaultPath: appConfig.directoryViewPath, directory: true, multiple: false })
    .then((res) => res)
    .catch((err) => appWindow.emit("notification-add", { type: "Error", origin: "changeFilesDir", msg: err }));
  if (path !== null) {
    activeLocalFilesPath.value = path;
    await getDzrsTrackObjectsDir();
    await invoke("watch_dir", { dir: path }).catch((err) => appWindow.emit("notification-add", { type: "Error", origin: "changeFilesDir", msg: err }));
  }
}

// Keeps selectedFilePaths updated based on which files have been selected on the local files panel
// this function supports selecting files using CTRL and SHIFT keys
function selectFiles(event, file) {
  if (event.shiftKey) {
    let indexStart;
    if (selectedFilePaths.value.length >= 1) {
      const _filePath = selectedFilePaths.value[selectedFilePaths.value.length - 1];
      const _i = dzrsTrackObjects.value.findIndex((_file) => _file.filePath === _filePath);
      indexStart = _i;
    } else {
      indexStart = 0;
    }
    const indexEnd = dzrsTrackObjects.value.indexOf(file);
    if (indexStart < indexEnd) {
      for (let i = indexStart + 1; i <= indexEnd; i++) {
        const _filePath = dzrsTrackObjects.value[i].filePath;
        if (selectedFilePaths.value.includes(_filePath)) {
          const _i = selectedFilePaths.value.indexOf(_filePath);
          selectedFilePaths.value.splice(_i, 1);
        } else {
          selectedFilePaths.value.push(_filePath);
        }
      }
    } else {
      for (let i = indexStart - 1; i >= indexEnd; i--) {
        const _filePath = dzrsTrackObjects.value[i].filePath;
        if (selectedFilePaths.value.includes(_filePath)) {
          const _i = selectedFilePaths.value.indexOf(_filePath);
          selectedFilePaths.value.splice(_i, 1);
        } else {
          selectedFilePaths.value.push(_filePath);
        }
      }
    }
  } else if (event.ctrlKey) {
    if (selectedFilePaths.value.includes(file.filePath)) {
      let i = selectedFilePaths.value.indexOf(file.filePath);
      selectedFilePaths.value.splice(i, 1);
    } else {
      selectedFilePaths.value.push(file.filePath);
    }
  } else {
    selectedFilePaths.value = [file.filePath];
  }
}

// Fetches tags from deezer for the selected flac files, then retrieves the new track objects from backend
async function fetchDzrsTrackObjects() {
  tagsIsFetchingOrSaving.value = true;
  let flacs = [];
  if (selectedFilePaths.value.length === 0) {
    flacs = dzrsTrackObjects.value.filter((t) => t.fileExtension === "flac").map((f) => f.filePath);
  } else {
    flacs = dzrsTrackObjects.value.filter((t) => t.fileExtension === "flac" && selectedFilePaths.value.includes(t.filePath)).map((f) => f.filePath);
  }
  for (const p of flacs) {
    await invoke("tracks_fetch", { paths: [p] }).catch((err) => appWindow.emit("notification-add", { type: "Error", origin: "fetchDzrsTrackObjects", msg: err.join("") }));
    await getDzrsTrackObjects([p]);
  }
  tagsIsFetchingOrSaving.value = false;
}

// Replace tags of a track by fetching another deezer payload for a given track_id
// Called when applying a source for a specific track
async function fetchTrackTagsFromSource(id) {
  tagsIsFetchingOrSaving.value = true;
  const path = activeDzrsTrackObject.value.filePath;
  await invoke("tracks_source", { path: path, id: id }).catch((err) => appWindow.emit("notification-add", { type: "Error", origin: "fetchTrackTagsFromSource", msg: err }));
  await getDzrsTrackObjects([path]);
  tagsIsFetchingOrSaving.value = false;
}

async function fetchTrackSources() {
  tagsIsFetchingOrSaving.value = true;
  await invoke("tracks_fetch_sources", { path: activeDzrsTrackObject.value.filePath }).catch((err) => appWindow.emit("notification-add", { type: "Error", origin: "fetchTrackSources", msg: err }));
  await getDzrsTrackObjects([activeDzrsTrackObject.value.filePath]);
  tagsIsFetchingOrSaving.value = false;
}

async function fetchTrackSourcesManual() {
  tagsIsFetchingOrSaving.value = true;
  await invoke("tracks_fetch_sources_manual", { path: activeDzrsTrackObject.value.filePath, query: inputFetchSources.value.value }).catch((err) => appWindow.emit("notification-add", { type: "Error", origin: "fetchTrackSources", msg: err }));
  await getDzrsTrackObjects([activeDzrsTrackObject.value.filePath]);
  tagsIsFetchingOrSaving.value = false;
}

// Reload tags in the tags_to_save field for the active track to match the ones currently saved in the file
async function reloadTagsFromFile() {
  await invoke("tracks_reload", { path: activeDzrsTrackObject.value.filePath }).catch((err) => appWindow.emit("notification-add", { type: "Error", origin: "restoreTagsFromFile", msg: err }));
  await getDzrsTrackObjects([activeDzrsTrackObject.value.filePath]);
}

// Saves edited files based on selection or all of them if no selection was made, then retrieves the new track objects from backend
async function saveModifiedTracks() {
  const confirmation = await confirm("Save modified files?", { title: "Save", type: "warning" });
  if (confirmation) {
    tagsIsFetchingOrSaving.value = true;
    let modifiedTracks = [];
    if (selectedFilePaths.value.length >= 1) {
      modifiedTracks = dzrsTrackObjects.value.filter((t) => !isEqual(t.tags, t.tagsToSave) && selectedFilePaths.value.includes(t.filePath));
    } else {
      modifiedTracks = dzrsTrackObjects.value.filter((t) => !isEqual(t.tags, t.tagsToSave));
    }
    if (modifiedTracks.length !== 0) {
      for (const t of modifiedTracks) {
        await invoke("save_tags", { path: t.filePath, tags: t.tagsToSave }).catch((err) => appWindow.emit("notification-add", { type: "Error", origin: "saveModifiedTracks", msg: err }));
        await getDzrsTrackObjects([t.filePath]);
      }
    }
    tagsIsFetchingOrSaving.value = false;
  }
}

// Starts listening to the file watcher initialized in the backend
// for every event we send the appropriate request to the backend for syncronizing the
// inner DzrsTrackObjects stored in memory to match the files
async function listenFileWatcher() {
  await appWindow.listen("watcher_triggered", async (e) => {
    for (const p of e.payload.paths) {
      if (Object.hasOwn(e.payload.type, "create")) {
        await invoke("tracks_insert", { path: p });
      } else if (Object.hasOwn(e.payload.type, "remove")) {
        await invoke("tracks_remove", { path: p }).catch((_) => _);
      }
    }
    if (Object.hasOwn(e.payload.type, "create")) {
      await getDzrsTrackObjects(e.payload.paths);
    } else if (Object.hasOwn(e.payload.type, "remove")) {
      await getDzrsTrackObjects(e.payload.paths, true);
    }
  });
}

onBeforeMount(async () => {
  // At startup load files located in the configured directory
  await getDzrsTrackObjectsDir();
  // Initialize the listener for handling changes in the watched directory
  await listenFileWatcher();
  document.addEventListener("click", (event) => {
    if (!event.target.closest(".table-filter-btn")) {
      showFilterMenu.value = false;
    }
  });
});
</script>

<template>
  <HeaderBar>
    <div class="row header-content">
      <button style="padding: 2px 8px" @click="changeFilesDir">
        <div class="row clickable-effect" style="color: var(--color-text)">
          <IconFolder size="20" class="icon" style="margin-right: 3px" />
          <span>Open</span>
        </div>
      </button>
      <div class="row" style="padding: 2px 6px; flex-grow: 1; justify-content: start; overflow: hidden">
        <p style="font-size: 0.9em; font-weight: 300; letter-spacing: 0.11em; white-space: nowrap; max-width: 20px">
          {{ activeLocalFilesPath ? activeLocalFilesPath : "..." }}
        </p>
      </div>
      <button style="padding: 2px 8px" @click="invoke('browse_cmd', { path: appConfig.directoryViewPath })" v-show="appConfig.directoryViewPath">
        <div class="row clickable-effect" style="color: var(--color-text)">
          <span>Browse</span>
          <IconFolder size="20" class="icon" style="margin-left: 3px" />
        </div>
      </button>
      <button style="padding: 2px 8px" @click="fetchDzrsTrackObjects" :disabled="tagsIsFetchingOrSaving || !tagsFetchingOrSavingEnabled">
        <div class="row clickable-effect" style="color: var(--color-text)" v-tooltip.bottom="'Retrieve Deezer Tags'">
          <span>Fetch</span>
          <IconTagStarred v-if="!tagsIsFetchingOrSaving" size="20" class="icon" style="margin-left: 3px" />
          <IconLoader2 v-else size="20" class="icon icon-loading" style="margin-left: 3px" />
        </div>
      </button>
      <button style="padding: 2px 8px" @click="saveModifiedTracks" :disabled="!tagsNeedSave || tagsIsFetchingOrSaving || !tagsFetchingOrSavingEnabled">
        <div class="row clickable-effect" style="color: var(--color-text)">
          <span>Save</span>
          <IconDeviceFloppy size="20" class="icon" style="margin-left: 3px" />
        </div>
      </button>
    </div>
  </HeaderBar>
  <div class="container" style="gap: 5px">
    <div class="row" style="gap: 5px; flex-grow: 1">
      <div class="directory-panel">
        <div class="frame" @click.self="selectedFilePaths = []">
          <table>
            <thead class="table-header">
              <tr>
                <th style="width: 20px; position: relative">
                  <TableFilter :columns="filterColumnsDirView" />
                </th>
                <th><!-- Reserved for icon --></th>
                <th v-for="column in filterColumnsDirView" :key="column.key" v-show="column.enabled">
                  {{ column.label }}
                </th>
              </tr>
            </thead>
            <tbody v-show="!tracksIsLoading">
              <template v-for="file in dzrsTrackObjects" :key="file.filePath">
                <tr @click="selectFiles($event, file)" :class="{ 'selected-file': selectedFilePaths.includes(file.filePath), 'greyed-file': file.fileExtension !== 'flac' }">
                  <td>
                    <IconPointFilled v-if="!isEqual(file.tags, file.tagsToSave)" />
                  </td>
                  <td class="img-container">
                    <div class="row">
                      <img v-if="file.tagsPictures.length >= 1" :src="`data:image/png;base64, ${file.tagsPictures[0].b64}`" style="border-radius: 4px" />
                      <IconMusic v-else-if="['flac', 'mp3'].includes(file.fileExtension)" color="#c9c9c9" />
                      <IconFile v-else color="#c9c9c9" />
                    </div>
                  </td>
                  <td v-show="filterColumnsDirView.find((col) => col.key === 'filename' && col.enabled)" style="padding-left: 4px; text-align: left">
                    {{ file.fileName }}
                  </td>
                  <td v-show="filterColumnsDirView.find((col) => col.key === 'size' && col.enabled)">{{ (file.fileSize / (1024 * 1024)).toFixed(1) }} MB</td>
                  <td v-show="filterColumnsDirView.find((col) => col.key === 'extension' && col.enabled)">
                    {{ file.fileExtension }}
                  </td>
                  <td v-show="filterColumnsDirView.find((col) => col.key === 'tagStatus' && col.enabled)">
                    <IconProgressCheck v-if="file.tagsStatus === 'finalized'" color="var(--color-success)" v-tooltip="'File Saved'" class="icon" />
                    <IconProgressBolt v-else-if="file.tagsStatus === 'matched'" color="#578867" v-tooltip="'Good Match Applied'" class="icon" />
                    <IconProgressHelp v-else-if="file.tagsStatus === 'successfull'" color="#998f40" v-tooltip="'Multiple Sources Found'" class="icon" />
                    <IconProgressAlert v-else-if="file.tagsStatus === 'unsuccessfull'" color="var(--color-error)" v-tooltip="'No Tags Found'" class="icon" />
                    <IconProgress v-else color="#8c8c8c" v-tooltip="'Tags not Fetched'" class="icon" />
                  </td>
                </tr>
              </template>
            </tbody>
          </table>
          <IconLoader2 size="60" class="icon-loading" v-show="tracksIsLoading" style="height: 78%" />
        </div>
      </div>
      <div class="source-panel frame">
        <div class="column" style="height: 100px; flex-grow: 1; justify-content: start">
          <div class="row sources-header" style="margin-bottom: 2px; border-bottom: 1px solid var(--color-bg-2)">
            <IconTag v-tooltip="'Fetch Sources'" size="1.5em" style="cursor: pointer; margin-top: 5px; margin-bottom: 5px" class="icon clickable-effect" :class="{ 'disabled-icon': tagsIsFetchingOrSaving || !tagsFetchingOrSavingEnabled }" @click="fetchTrackSources" />
            <p style="flex-grow: 1; margin-top: 5px; margin-bottom: 5px">Sources</p>
            <IconRestore v-tooltip="'Reload Original Tags'" size="1.5em" style="cursor: pointer; margin-top: 5px; margin-bottom: 5px" class="icon clickable-effect" :class="{ 'disabled-icon': tagsIsFetchingOrSaving || !tagsFetchingOrSavingEnabled }" @click="reloadTagsFromFile" />
          </div>
          <div class="column" style="font-size: 0.92em; overflow-x: hidden; overflow-y: auto; justify-content: start; flex-grow: 1">
            <div v-for="(source, i) in activeDzrsTrackObject.tagsSources" :key="i" class="row sources-item" style="border: 1px solid var(--color); padding: 5px">
              <a :href="source.link" target="_blank" class="sources-item-cover">
                <IconExternalLink size="40" color="var(--color-text)" class="icon-link" />
                <div>
                  <img :src="source.cover" width="80" height="80" />
                </div>
              </a>
              <div class="column sources-item-text-col" style="flex-grow: 1; text-align: left">
                <div class="row">
                  <div class="row" style="overflow: hidden; justify-content: start; flex-grow: 1">
                    <p>
                      <span class="sources-item-text-head">Title:</span>
                      {{ source.title }}
                    </p>
                  </div>
                  <IconCloudDownload class="icon icon-check clickable-effect" :class="{ 'disabled-icon': tagsIsFetchingOrSaving || !tagsFetchingOrSavingEnabled }" @click="fetchTrackTagsFromSource(source.id)" v-tooltip="'Apply'" style="margin-left: 5px" />
                </div>
                <div class="row" style="overflow: hidden; justify-content: start">
                  <p>
                    <span class="sources-item-text-head">Album:</span>
                    {{ source.album }}
                  </p>
                </div>
                <div class="row">
                  <div class="row" style="overflow: hidden; justify-content: start; flex-grow: 1">
                    <p>
                      <span class="sources-item-text-head">Artist:</span>
                      {{ source.artist }}
                    </p>
                  </div>
                  <p style="text-align: right; max-width: fit-content">
                    <span>Length:</span>
                    {{ `${Math.floor(source.duration / 60)}:${(source.duration % 60).toString().padStart(2, "0")}` }}
                  </p>
                </div>
              </div>
            </div>
          </div>
          <div class="row" style="padding: 0px 2px; padding-top: 6px; border-top: 1px solid var(--color-bg-2)">
            <input placeholder="Manual Search..." spellcheck="false" @keypress.enter="fetchTrackSourcesManual" ref="inputFetchSources" :disabled="tagsIsFetchingOrSaving || !tagsFetchingOrSavingEnabled" style="padding: 2px 4px; flex-grow: 1; font-size: 0.8em; height: 25px" />
            <button style="margin-left: 5px; padding: 2px 4px" @click="fetchTrackSourcesManual" :disabled="tagsIsFetchingOrSaving || !tagsFetchingOrSavingEnabled">
              <div class="row clickable-effect" style="color: var(--color-text)">
                <IconSearch size="18" class="icon" />
              </div>
            </button>
          </div>
        </div>
      </div>
    </div>
    <div class="tags-panel">
      <div class="frame row" style="gap: 4px">
        <div class="image-tag column">
          <div class="column">
            <div v-if="activeDzrsTrackObject.fileExtension === 'flac'">
              <div v-for="(picture, i) in activeDzrsTrackObject.tagsPictures" :key="i" style="margin-bottom: 4px">
                <img :src="`data:image/png;base64, ${picture.b64}`" style="border-radius: 5%" />
                <p>{{ picture.picType }}</p>
                <p style="font-size: 0.8em">{{ picture.description }}</p>
                <p style="font-style: italic; font-size: 0.8em">{{ picture.width }}x{{ picture.height }}</p>
              </div>
            </div>
            <div v-else>
              <img src="/assets/tag-image-placeholder.png" />
              <p style="font-style: italic">Cover</p>
            </div>
          </div>
        </div>
        <div class="column" style="flex-grow: 1">
          <div class="column" style="flex-basis: 400px; overflow-y: auto; justify-content: start">
            <table>
              <thead class="table-header">
                <tr>
                  <th style="width: 12%">Tag</th>
                  <th>Current Value</th>
                  <th>Future Value</th>
                </tr>
              </thead>
              <tbody>
                <tr>
                  <th>Title</th>
                  <td>
                    <div>
                      <textarea spellcheck="false" type="text" v-model="activeDzrsTrackObject.tags.title" readonly></textarea>
                    </div>
                  </td>
                  <td>
                    <div>
                      <textarea spellcheck="false" type="text" :class="{ 'tag-accent-text': activeDzrsTrackObject.tagsToSave.title !== activeDzrsTrackObject.tags.title }" v-model="activeDzrsTrackObject.tagsToSave.title"></textarea>
                    </div>
                  </td>
                </tr>
                <tr>
                  <th>Artist</th>
                  <td>
                    <div>
                      <textarea spellcheck="false" type="text" v-model="activeDzrsTrackObject.tags.artist" readonly></textarea>
                    </div>
                  </td>
                  <td>
                    <div>
                      <textarea spellcheck="false" type="text" :class="{ 'tag-accent-text': activeDzrsTrackObject.tagsToSave.artist !== activeDzrsTrackObject.tags.artist }" v-model="activeDzrsTrackObject.tagsToSave.artist"></textarea>
                    </div>
                  </td>
                </tr>
                <tr>
                  <th>Album</th>
                  <td>
                    <div>
                      <textarea spellcheck="false" type="text" v-model="activeDzrsTrackObject.tags.album" readonly></textarea>
                    </div>
                  </td>
                  <td>
                    <div>
                      <textarea spellcheck="false" type="text" :class="{ 'tag-accent-text': activeDzrsTrackObject.tagsToSave.album !== activeDzrsTrackObject.tags.album }" v-model="activeDzrsTrackObject.tagsToSave.album"></textarea>
                    </div>
                  </td>
                </tr>
                <tr>
                  <th>Album Artist</th>
                  <td>
                    <div>
                      <textarea spellcheck="false" type="text" v-model="activeDzrsTrackObject.tags.albumArtist" readonly></textarea>
                    </div>
                  </td>
                  <td>
                    <div>
                      <textarea spellcheck="false" type="text" :class="{ 'tag-accent-text': activeDzrsTrackObject.tagsToSave.albumArtist !== activeDzrsTrackObject.tags.albumArtist }" v-model="activeDzrsTrackObject.tagsToSave.albumArtist"></textarea>
                    </div>
                  </td>
                </tr>
                <tr>
                  <th>Composer</th>
                  <td>
                    <div>
                      <textarea spellcheck="false" type="text" v-model="activeDzrsTrackObject.tags.composer" readonly></textarea>
                    </div>
                  </td>
                  <td>
                    <div>
                      <textarea spellcheck="false" type="text" :class="{ 'tag-accent-text': activeDzrsTrackObject.tagsToSave.composer !== activeDzrsTrackObject.tags.composer }" v-model="activeDzrsTrackObject.tagsToSave.composer"></textarea>
                    </div>
                  </td>
                </tr>
                <tr>
                  <th>Performer</th>
                  <td>
                    <div>
                      <textarea spellcheck="false" type="text" v-model="activeDzrsTrackObject.tags.performer" readonly></textarea>
                    </div>
                  </td>
                  <td>
                    <div>
                      <textarea spellcheck="false" type="text" :class="{ 'tag-accent-text': activeDzrsTrackObject.tagsToSave.performer !== activeDzrsTrackObject.tags.performer }" v-model="activeDzrsTrackObject.tagsToSave.performer"></textarea>
                    </div>
                  </td>
                </tr>
                <tr>
                  <th>Producer</th>
                  <td>
                    <div>
                      <textarea spellcheck="false" type="text" v-model="activeDzrsTrackObject.tags.producer" readonly></textarea>
                    </div>
                  </td>
                  <td>
                    <div>
                      <textarea spellcheck="false" type="text" :class="{ 'tag-accent-text': activeDzrsTrackObject.tagsToSave.producer !== activeDzrsTrackObject.tags.producer }" v-model="activeDzrsTrackObject.tagsToSave.producer"></textarea>
                    </div>
                  </td>
                </tr>
                <tr>
                  <th>Genre</th>
                  <td>
                    <div>
                      <textarea spellcheck="false" type="text" v-model="activeDzrsTrackObject.tags.genre" readonly></textarea>
                    </div>
                  </td>
                  <td>
                    <div>
                      <textarea spellcheck="false" type="text" :class="{ 'tag-accent-text': activeDzrsTrackObject.tagsToSave.genre !== activeDzrsTrackObject.tags.genre }" v-model="activeDzrsTrackObject.tagsToSave.genre"></textarea>
                    </div>
                  </td>
                </tr>
                <tr :set="(len = `${Math.floor(activeDzrsTrackObject.tags.length / 60)}:${(activeDzrsTrackObject.tags.length % 60).toString().padStart(2, '0')}`)">
                  <th>Length</th>
                  <td>
                    <div>
                      <textarea spellcheck="false" type="text" v-model="len" readonly></textarea>
                    </div>
                  </td>
                  <td>
                    <div>
                      <textarea spellcheck="false" type="text" v-model="len" readonly></textarea>
                    </div>
                  </td>
                </tr>
                <tr style="height: 300px">
                  <th>Lyrics</th>
                  <td>
                    <div>
                      <textarea spellcheck="false" type="text" v-model="activeDzrsTrackObject.tags.lyrics" readonly></textarea>
                    </div>
                  </td>
                  <td>
                    <div>
                      <textarea spellcheck="false" type="text" :class="{ 'tag-accent-text': activeDzrsTrackObject.tagsToSave.lyrics !== activeDzrsTrackObject.tags.lyrics }" v-model="activeDzrsTrackObject.tagsToSave.lyrics"></textarea>
                    </div>
                  </td>
                </tr>
                <tr style="height: 80px">
                  <th>Copyright</th>
                  <td>
                    <div>
                      <textarea spellcheck="false" type="text" v-model="activeDzrsTrackObject.tags.copyright" readonly></textarea>
                    </div>
                  </td>
                  <td>
                    <div>
                      <textarea spellcheck="false" type="text" :class="{ 'tag-accent-text': activeDzrsTrackObject.tagsToSave.copyright !== activeDzrsTrackObject.tags.copyright }" v-model="activeDzrsTrackObject.tagsToSave.copyright"></textarea>
                    </div>
                  </td>
                </tr>
                <tr>
                  <th style="height: 80px">Description</th>
                  <td>
                    <div>
                      <textarea spellcheck="false" type="text" v-model="activeDzrsTrackObject.tags.description" readonly></textarea>
                    </div>
                  </td>
                  <td>
                    <div>
                      <textarea spellcheck="false" type="text" :class="{ 'tag-accent-text': activeDzrsTrackObject.tagsToSave.description !== activeDzrsTrackObject.tags.description }" v-model="activeDzrsTrackObject.tagsToSave.description"></textarea>
                    </div>
                  </td>
                </tr>
                <tr>
                  <th>Track Number</th>
                  <td>
                    <div>
                      <textarea spellcheck="false" type="text" v-model="activeDzrsTrackObject.tags.trackNumber" readonly></textarea>
                    </div>
                  </td>
                  <td>
                    <div>
                      <textarea spellcheck="false" type="text" :class="{ 'tag-accent-text': activeDzrsTrackObject.tagsToSave.trackNumber !== activeDzrsTrackObject.tags.trackNumber }" v-model="activeDzrsTrackObject.tagsToSave.trackNumber"></textarea>
                    </div>
                  </td>
                </tr>
                <tr>
                  <th>Total Tracks</th>
                  <td>
                    <div>
                      <textarea spellcheck="false" type="text" v-model="activeDzrsTrackObject.tags.trackTotal" readonly></textarea>
                    </div>
                  </td>
                  <td>
                    <div>
                      <textarea spellcheck="false" type="text" :class="{ 'tag-accent-text': activeDzrsTrackObject.tagsToSave.trackTotal !== activeDzrsTrackObject.tags.trackTotal }" v-model="activeDzrsTrackObject.tagsToSave.trackTotal"></textarea>
                    </div>
                  </td>
                </tr>
                <tr>
                  <th>Disk Number</th>
                  <td>
                    <div>
                      <textarea spellcheck="false" type="text" v-model="activeDzrsTrackObject.tags.diskNumber" readonly></textarea>
                    </div>
                  </td>
                  <td>
                    <div>
                      <textarea spellcheck="false" type="text" :class="{ 'tag-accent-text': activeDzrsTrackObject.tagsToSave.diskNumber !== activeDzrsTrackObject.tags.diskNumber }" v-model="activeDzrsTrackObject.tagsToSave.diskNumber"></textarea>
                    </div>
                  </td>
                </tr>
                <tr>
                  <th>Total Disks</th>
                  <td>
                    <div>
                      <textarea spellcheck="false" type="text" v-model="activeDzrsTrackObject.tags.diskTotal" readonly></textarea>
                    </div>
                  </td>
                  <td>
                    <div>
                      <textarea spellcheck="false" type="text" :class="{ 'tag-accent-text': activeDzrsTrackObject.tagsToSave.diskTotal !== activeDzrsTrackObject.tags.diskTotal }" v-model="activeDzrsTrackObject.tagsToSave.diskTotal"></textarea>
                    </div>
                  </td>
                </tr>
                <tr>
                  <th>Date</th>
                  <td>
                    <div>
                      <textarea spellcheck="false" type="text" v-model="activeDzrsTrackObject.tags.date" readonly></textarea>
                    </div>
                  </td>
                  <td>
                    <div>
                      <textarea spellcheck="false" type="text" :class="{ 'tag-accent-text': activeDzrsTrackObject.tagsToSave.date !== activeDzrsTrackObject.tags.date }" v-model="activeDzrsTrackObject.tagsToSave.date"></textarea>
                    </div>
                  </td>
                </tr>
                <tr>
                  <th>Year</th>
                  <td>
                    <div>
                      <textarea spellcheck="false" type="text" v-model="activeDzrsTrackObject.tags.year" readonly></textarea>
                    </div>
                  </td>
                  <td>
                    <div>
                      <textarea spellcheck="false" type="text" :class="{ 'tag-accent-text': activeDzrsTrackObject.tagsToSave.year !== activeDzrsTrackObject.tags.year }" v-model="activeDzrsTrackObject.tagsToSave.year"></textarea>
                    </div>
                  </td>
                </tr>
                <tr>
                  <th>Original Date</th>
                  <td>
                    <div>
                      <textarea spellcheck="false" type="text" v-model="activeDzrsTrackObject.tags.originalDate" readonly></textarea>
                    </div>
                  </td>
                  <td>
                    <div>
                      <textarea spellcheck="false" type="text" :class="{ 'tag-accent-text': activeDzrsTrackObject.tagsToSave.originalDate !== activeDzrsTrackObject.tags.originalDate }" v-model="activeDzrsTrackObject.tagsToSave.originalDate"></textarea>
                    </div>
                  </td>
                </tr>
                <tr>
                  <th>Comment</th>
                  <td>
                    <div>
                      <textarea spellcheck="false" type="text" v-model="activeDzrsTrackObject.tags.comment" readonly></textarea>
                    </div>
                  </td>
                  <td>
                    <div>
                      <textarea spellcheck="false" type="text" :class="{ 'tag-accent-text': activeDzrsTrackObject.tagsToSave.comment !== activeDzrsTrackObject.tags.comment }" v-model="activeDzrsTrackObject.tagsToSave.comment"></textarea>
                    </div>
                  </td>
                </tr>
                <tr>
                  <th>Label</th>
                  <td>
                    <div>
                      <textarea spellcheck="false" type="text" v-model="activeDzrsTrackObject.tags.label" readonly></textarea>
                    </div>
                  </td>
                  <td>
                    <div>
                      <textarea spellcheck="false" type="text" :class="{ 'tag-accent-text': activeDzrsTrackObject.tagsToSave.label !== activeDzrsTrackObject.tags.label }" v-model="activeDzrsTrackObject.tagsToSave.label"></textarea>
                    </div>
                  </td>
                </tr>
                <tr>
                  <th>Organization</th>
                  <td>
                    <div>
                      <textarea spellcheck="false" type="text" v-model="activeDzrsTrackObject.tags.organization" readonly></textarea>
                    </div>
                  </td>
                  <td>
                    <div>
                      <textarea spellcheck="false" type="text" :class="{ 'tag-accent-text': activeDzrsTrackObject.tagsToSave.organization !== activeDzrsTrackObject.tags.organization }" v-model="activeDzrsTrackObject.tagsToSave.organization"></textarea>
                    </div>
                  </td>
                </tr>
                <tr>
                  <th>Barcode</th>
                  <td>
                    <div>
                      <textarea spellcheck="false" type="text" v-model="activeDzrsTrackObject.tags.barcode" readonly></textarea>
                    </div>
                  </td>
                  <td>
                    <div>
                      <textarea spellcheck="false" type="text" :class="{ 'tag-accent-text': activeDzrsTrackObject.tagsToSave.barcode !== activeDzrsTrackObject.tags.barcode }" v-model="activeDzrsTrackObject.tagsToSave.barcode"></textarea>
                    </div>
                  </td>
                </tr>
                <tr>
                  <th>ISRC</th>
                  <td>
                    <div>
                      <textarea spellcheck="false" type="text" v-model="activeDzrsTrackObject.tags.isrc" readonly></textarea>
                    </div>
                  </td>
                  <td>
                    <div>
                      <textarea spellcheck="false" type="text" :class="{ 'tag-accent-text': activeDzrsTrackObject.tagsToSave.isrc !== activeDzrsTrackObject.tags.isrc }" v-model="activeDzrsTrackObject.tagsToSave.isrc"></textarea>
                    </div>
                  </td>
                </tr>
                <tr>
                  <th>BPM</th>
                  <td>
                    <div>
                      <textarea spellcheck="false" type="text" v-model="activeDzrsTrackObject.tags.bpm" readonly></textarea>
                    </div>
                  </td>
                  <td>
                    <div>
                      <textarea spellcheck="false" type="text" :class="{ 'tag-accent-text': activeDzrsTrackObject.tagsToSave.bpm !== activeDzrsTrackObject.tags.bpm }" v-model="activeDzrsTrackObject.tagsToSave.bpm"></textarea>
                    </div>
                  </td>
                </tr>
                <tr>
                  <th>Parental Advisory</th>
                  <td>
                    <div>
                      <textarea spellcheck="false" type="text" v-model="activeDzrsTrackObject.tags.explicit" readonly></textarea>
                    </div>
                  </td>
                  <td>
                    <div>
                      <textarea spellcheck="false" type="text" :class="{ 'tag-accent-text': activeDzrsTrackObject.tagsToSave.explicit !== activeDzrsTrackObject.tags.explicit }" v-model="activeDzrsTrackObject.tagsToSave.explicit"></textarea>
                    </div>
                  </td>
                </tr>
                <tr>
                  <th>RG Track Gain</th>
                  <td>
                    <div>
                      <textarea spellcheck="false" type="text" v-model="activeDzrsTrackObject.tags.replaygainTrackGain" readonly></textarea>
                    </div>
                  </td>
                  <td>
                    <div>
                      <textarea spellcheck="false" type="text" :class="{ 'tag-accent-text': activeDzrsTrackObject.tagsToSave.replaygainTrackGain !== activeDzrsTrackObject.tags.replaygainTrackGain }" v-model="activeDzrsTrackObject.tagsToSave.replaygainTrackGain"></textarea>
                    </div>
                  </td>
                </tr>
                <tr>
                  <th>RG Track Peak</th>
                  <td>
                    <div>
                      <textarea spellcheck="false" type="text" v-model="activeDzrsTrackObject.tags.replaygainTrackPeak" readonly></textarea>
                    </div>
                  </td>
                  <td>
                    <div>
                      <textarea spellcheck="false" type="text" :class="{ 'tag-accent-text': activeDzrsTrackObject.tagsToSave.replaygainTrackPeak !== activeDzrsTrackObject.tags.replaygainTrackPeak }" v-model="activeDzrsTrackObject.tagsToSave.replaygainTrackPeak"></textarea>
                    </div>
                  </td>
                </tr>
                <tr>
                  <th>RG Album Gain</th>
                  <td>
                    <div>
                      <textarea spellcheck="false" type="text" v-model="activeDzrsTrackObject.tags.replaygainAlbumGain" readonly></textarea>
                    </div>
                  </td>
                  <td>
                    <div>
                      <textarea spellcheck="false" type="text" :class="{ 'tag-accent-text': activeDzrsTrackObject.tagsToSave.replaygainAlbumGain !== activeDzrsTrackObject.tags.replaygainAlbumGain }" v-model="activeDzrsTrackObject.tagsToSave.replaygainAlbumGain"></textarea>
                    </div>
                  </td>
                </tr>
                <tr>
                  <th>RG Album Peak</th>
                  <td>
                    <div>
                      <textarea spellcheck="false" type="text" v-model="activeDzrsTrackObject.tags.replaygainAlbumPeak" readonly></textarea>
                    </div>
                  </td>
                  <td>
                    <div>
                      <textarea spellcheck="false" type="text" :class="{ 'tag-accent-text': activeDzrsTrackObject.tagsToSave.replaygainAlbumPeak !== activeDzrsTrackObject.tags.replaygainAlbumPeak }" v-model="activeDzrsTrackObject.tagsToSave.replaygainAlbumPeak"></textarea>
                    </div>
                  </td>
                </tr>
                <tr>
                  <th>Encoder</th>
                  <td>
                    <div>
                      <textarea spellcheck="false" type="text" v-model="activeDzrsTrackObject.tags.encoder" readonly></textarea>
                    </div>
                  </td>
                  <td>
                    <div>
                      <textarea spellcheck="false" type="text" :class="{ 'tag-accent-text': activeDzrsTrackObject.tagsToSave.encoder !== activeDzrsTrackObject.tags.encoder }" v-model="activeDzrsTrackObject.tagsToSave.encoder"></textarea>
                    </div>
                  </td>
                </tr>
                <tr>
                  <th style="border-top: 1px solid var(--color-bg-2)" colspan="3">Other Tags</th>
                </tr>
                <tr v-for="(extraTag, i) in activeDzrsTrackObject.tags.extraTags" :key="i">
                  <th>{{ extraTag[0] }}</th>
                  <td>
                    <div>
                      <textarea spellcheck="false" type="text" v-model="extraTag[1]" readonly></textarea>
                    </div>
                  </td>
                  <td>
                    <div>
                      <textarea spellcheck="false" type="text" :class="{ 'tag-accent-text': extraTag[1] !== activeDzrsTrackObject.tagsToSave.extraTags[i][1] }" v-model="activeDzrsTrackObject.tagsToSave.extraTags[i][1]"></textarea>
                    </div>
                  </td>
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
.directory-panel,
.tags-panel,
.source-panel {
  flex-grow: 1;
  display: flex;
  flex-direction: column;
}

.source-panel {
  flex-basis: 0px;
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

.header-content span {
  user-select: none;
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

.directory-panel tbody tr:hover,
.tags-panel tbody tr:hover,
.selected-file {
  background-color: var(--color-hover);
}

.greyed-file {
  color: #b8b8b8;
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
  resize: none;
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

.sources-item {
  border-radius: 4px;
  transition: background-color 0.2s ease;
}

.sources-item:hover {
  background-color: var(--color-bg-2);
}

.sources-item .icon-check {
  opacity: 0%;
  transition: opacity 0.2s ease;
}

.sources-item:hover .icon-check {
  opacity: 100%;
  transition: opacity 0.1s ease;
}

.sources-item-cover {
  position: relative;
}

.sources-item-cover img {
  display: block;
  margin-bottom: auto;
  margin-top: auto;
  border-radius: 4px;
  user-select: none;
  opacity: 100%;
  transition: opacity 0.2s ease;
}

.sources-item-cover:hover img {
  opacity: 50%;
}

.sources-item-cover .icon-link {
  opacity: 0%;
  transition: opacity 0.2s ease;
}

.sources-item-cover:hover .icon-link {
  opacity: 100%;
  transition: opacity 0.2s ease;
}

.sources-item-cover .icon-link {
  top: 25%;
  right: 25%;
  position: absolute;
  cursor: pointer;
  user-select: none;
}

@media (max-width: 1300px) {
  .sources-item-text-col {
    font-size: 0.88em;
  }

  .sources-item p {
    padding-left: 5px !important;
  }
}

@media (max-width: 1150px) {
  .header-content span {
    display: none;
  }

  .sources-item-text-head {
    display: none;
  }
}

@media (max-width: 1000px) {
  .sources-item-text-col:nth-of-type(2) {
    flex-grow: 1;
  }
}

.sources-item p {
  max-width: 10px;
  padding-left: 15px;
  flex-grow: 1;
  margin-top: 0px;
  margin-bottom: 0px;
  text-wrap: nowrap;
}

.sources-item p span {
  opacity: 0.5;
  font-style: italic;
  user-select: none;
}

.icon-check {
  cursor: pointer;
}

.img-container,
.img-container * {
  width: 30px;
}

.tag-accent-text {
  color: #998140;
}

.expanded {
  display: flex;
  width: 200px;
}

::-webkit-scrollbar {
  width: 8px;
}
</style>

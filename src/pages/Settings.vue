<script setup>
import { IconFolder, IconFolderFilled, IconTextSize, IconCheck, IconFileFilled, IconBookmarksFilled, IconList } from "@tabler/icons-vue";

import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { open } from "@tauri-apps/api/dialog";
import { appWindow } from "@tauri-apps/api/window";
import { downloadDir } from "@tauri-apps/api/path";
import { writeText } from "@tauri-apps/api/clipboard";

import SettingsGroup from "../components/SettingsGroup.vue";
import HeaderBar from "../components/HeaderBar.vue";

import { appConfig, filterColumnsDownload, tagSeparators } from "../globals";

const downloadInputValue = ref(appConfig.downloadPath);
const fileTemplateInput = ref(null);
const fileTemplateValue = ref(appConfig.fileTemplate);
const localFilesInputValue = ref(appConfig.directoryViewPath);

async function setDownloadPath() {
  const defaultPath = await downloadDir()
    .then((result) => result)
    .catch((err) => appWindow.emit("notification-add", { type: "Error", origin: "setDownloadPath", msg: err }));
  const path = await open({ defaultPath: defaultPath, directory: true, multiple: false })
    .then((result) => result)
    .catch((err) => appWindow.emit("notification-add", { type: "Error", origin: "setDownloadPath", msg: err }));
  if (path !== null) {
    await invoke("config_set", { key: "download_path", value: path }).catch((err) => appWindow.emit("notification-add", { type: "Error", origin: "setDownloadPath", msg: err }));
    appConfig.downloadPath = path;
    downloadInputValue.value = path;
    appWindow.emit("instant-notification-add", { type: "Info", origin: "Settings", msg: "Setting Updated!" });
  }
}

async function saveFileTemplate() {
  await invoke("config_set", { key: "file_template", value: fileTemplateInput.value.value }).catch((err) => appWindow.emit("notification-add", { type: "Error", origin: "saveFileTemplate", msg: err }));
  appConfig.fileTemplate = fileTemplateInput.value.value;
  appWindow.emit("instant-notification-add", { type: "Info", origin: "Settings", msg: "Setting Updated!" });
}

// Updates a single config entry in both backend and frontend, the underlying command will persist the change into the config file
async function updateConfig(key, value) {
  await invoke("config_set", { key: key, value: value }).catch((err) => appWindow.emit("notification-add", { type: "Error", origin: "updateConfig", msg: err }));
  appConfig[key] = value;
}

// Copies to the clipboard the clicked element's innerHTML
async function copyEventTargetToClipboard(event) {
  await writeText(event.target.innerHTML).catch((err) => appWindow.emit("notification-add", { type: "Error", origin: "copyEventTargetToClipboard", msg: err }));
}

async function setLocalFilesPath() {
  const path = await open({ directory: true, multiple: false })
    .then((result) => result)
    .catch((err) => appWindow.emit("notification-add", { type: "Error", origin: "setLocalFilesPath", msg: err }));
  if (path !== null) {
    await invoke("config_set", { key: "directory_view_path", value: path }).catch((err) => appWindow.emit("notification-add", { type: "Error", origin: "setLocalFilesPath", msg: err }));
    appConfig.directoryViewPath = path;
    localFilesInputValue.value = path;
    appWindow.emit("instant-notification-add", { type: "Info", origin: "Settings", msg: "Setting Updated!" });
  }
}
</script>

<template>
  <HeaderBar />
  <div class="container" style="overflow-y: auto">
    <div class="column">
      <SettingsGroup :body-as-column="true" class="group-download">
        <template #head>
          <IconFolderFilled size="30" class="icon setting-icon" />
          <h1>Download</h1>
        </template>
        <template #body>
          <p>Download Path</p>
          <div class="row">
            <input :value="downloadInputValue" type="text" placeholder="Open..." style="flex-grow: 1" />
            <button style="margin-left: 15px" @click="setDownloadPath">
              <IconFolder class="icon clickable-effect" />
            </button>
          </div>
        </template>
      </SettingsGroup>
      <SettingsGroup :body-as-column="true" class="group-file-template">
        <template #head>
          <IconTextSize size="30" class="icon setting-icon" />
          <h1>File Template</h1>
        </template>
        <template #body>
          <p style="font-size: 1em">Downloaded files will be saved with the following name</p>
          <div class="row">
            <input style="flex-grow: 1" type="text" spellcheck="false" placeholder="File name template..." :value="fileTemplateValue" ref="fileTemplateInput" />
            <button @click.prevent="saveFileTemplate" style="margin-left: 10px">
              <IconCheck class="icon clickable-effect" />
            </button>
          </div>
          <div class="row" style="margin-top: 10px; padding-left: 10px; padding-right: 10px; justify-content: flex-start; flex-wrap: wrap">
            <p>Available Variables:</p>
            <p v-for="item in filterColumnsDownload" :key="item.id" class="clickable-effect" v-tooltip="{ content: 'Copied!', triggers: ['click'], hideTriggers: ['hover'] }" @click="copyEventTargetToClipboard">
              {{ `%${item.key}%` }}
            </p>
          </div>
        </template>
      </SettingsGroup>
      <SettingsGroup :body-as-column="true" class="group-local-files">
        <template #head>
          <IconFileFilled size="30" class="icon setting-icon" />
          <h1>Local Files</h1>
        </template>
        <template #body>
          <p>Default Directory</p>
          <div class="row">
            <input :value="localFilesInputValue" type="text" placeholder="Open..." style="flex-grow: 1" />
            <button style="margin-left: 15px" @click="setLocalFilesPath">
              <IconFolder class="icon clickable-effect" />
            </button>
          </div>
        </template>
      </SettingsGroup>
      <SettingsGroup :body-as-column="true" class="group-tags">
        <template #head>
          <IconBookmarksFilled size="30" class="icon setting-icon" />
          <h1>Deezer Tags</h1>
        </template>
        <template #body>
          <div class="frame" style="padding: 15px">
            <div class="row" style="flex-grow: 1">
              <div class="column" style="flex-basis: 50%; align-self: flex-start">
                <div class="row">
                  <input @input="(e) => updateConfig('tag_dz_title', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagDzTitle" />
                  <span>Title</span>
                </div>
                <div class="row">
                  <input @input="(e) => updateConfig('tag_dz_artist', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagDzArtist" />
                  <span>Artist</span>
                </div>
                <div class="row">
                  <input @input="(e) => updateConfig('tag_dz_album', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagDzAlbum" />
                  <span>Album</span>
                </div>
                <div class="row">
                  <input @input="(e) => updateConfig('tag_dz_track_number', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagDzTrackNumber" />
                  <span>Track Number</span>
                </div>
                <div class="row">
                  <input @input="(e) => updateConfig('tag_dz_track_total', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagDzTrackTotal" />
                  <span>Track Total</span>
                </div>
                <div class="row">
                  <input @input="(e) => updateConfig('tag_dz_disk_number', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagDzDiskNumber" />
                  <span>Disk Number</span>
                </div>
                <div class="row">
                  <input @input="(e) => updateConfig('tag_dz_disk_total', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagDzDiskTotal" />
                  <span>Disk Total</span>
                </div>
                <div class="row">
                  <input @input="(e) => updateConfig('tag_dz_album_artist', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagDzAlbumArtist" />
                  <span>Album Artist</span>
                </div>
                <div class="row">
                  <input @input="(e) => updateConfig('tag_dz_genre', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagDzGenre" />
                  <span>Genre</span>
                </div>
                <div class="row">
                  <input @input="(e) => updateConfig('tag_dz_year', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagDzYear" />
                  <span>Year</span>
                </div>
                <div class="row">
                  <input @input="(e) => updateConfig('tag_dz_date', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagDzDate" />
                  <span>Date</span>
                </div>
                <div class="row">
                  <input @input="(e) => updateConfig('tag_dz_original_date', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagDzOriginalDate" />
                  <span>Original Date</span>
                </div>
                <div class="row">
                  <input @input="(e) => updateConfig('tag_dz_itunesadvisory', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagDzItunesadvisory" />
                  <span>Explicit Lyrics</span>
                </div>
              </div>
              <div class="column" style="justify-content: flex-start; align-self: flex-start">
                <div class="row">
                  <input @input="(e) => updateConfig('tag_dz_isrc', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagDzIsrc" />
                  <span>ISRC</span>
                </div>
                <div class="row">
                  <input @input="(e) => updateConfig('tag_dz_length', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagDzLength" />
                  <span>Track Length</span>
                </div>
                <div class="row">
                  <input @input="(e) => updateConfig('tag_dz_barcode', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagDzBarcode" />
                  <span>Album Barcode (UPC)</span>
                </div>
                <div class="row">
                  <input @input="(e) => updateConfig('tag_dz_bpm', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagDzBpm" />
                  <span>BPM</span>
                </div>
                <div class="row">
                  <input @input="(e) => updateConfig('tag_dz_explicit', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagDzExplicit" />
                  <span>Parental Advisory Rating</span>
                </div>
                <div class="row">
                  <input @input="(e) => updateConfig('tag_dz_replaygain_track_gain', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagDzReplaygainTrackGain" />
                  <span>Replay Gain</span>
                </div>
                <div class="row">
                  <input @input="(e) => updateConfig('tag_dz_label', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagDzLabel" />
                  <span>Label</span>
                </div>
                <div class="row">
                  <input @input="(e) => updateConfig('tag_dz_organization', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagDzOrganization" />
                  <span>Organization</span>
                </div>
                <div class="row">
                  <input @input="(e) => updateConfig('tag_dz_lyrics', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagDzLyrics" />
                  <span>Lyrics</span>
                </div>
                <div class="row">
                  <input @input="(e) => updateConfig('tag_dz_copyright', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagDzCopyright" />
                  <span>Copyright</span>
                </div>
                <div class="row">
                  <input @input="(e) => updateConfig('tag_dz_composer', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagDzComposer" />
                  <span>Composer</span>
                </div>
                <div class="row">
                  <input @input="(e) => updateConfig('tag_dz_performer', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagDzPerformer" />
                  <span>Performer</span>
                </div>
                <div class="row">
                  <input @input="(e) => updateConfig('tag_dz_source_id', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagDzSourceId" />
                  <span>Deezer Song ID</span>
                </div>
              </div>
            </div>
          </div>
        </template>
      </SettingsGroup>
      <SettingsGroup :body-as-column="true" class="group-other">
        <template #head>
          <IconList size="30" class="icon setting-icon" />
          <h1>Other</h1>
        </template>
        <template #body>
          <div class="row" style="justify-content: start; margin-bottom: 16px">
            <span style="margin-right: 8px">Tags Separator</span>
            <select
              name="select-tag-separator"
              :value="appConfig.tagSeparator"
              @change="
                (e) => {
                  updateConfig('tag_separator', e.target.value);
                  appWindow.emit('instant-notification-add', { type: 'Info', origin: 'Settings', msg: 'Setting Updated!' });
                }
              ">
              <option v-for="(sep, i) in tagSeparators" :key="i" :value="sep">"{{ sep }}"</option>
            </select>
          </div>
          <div class="frame" style="padding: 15px; margin-bottom: 15px">
            <div class="row" style="justify-content: flex-start">
              <input @input="(e) => updateConfig('overwrite_downloads', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.overwriteDownloads" />
              <span style="margin-left: 8px">Overwrite existing files when downloading</span>
            </div>
            <div class="row" style="justify-content: flex-start; margin-top: 10px">
              <input @input="(e) => updateConfig('tag_prefer_sync_lyrics', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagPreferSyncLyrics" />
              <span style="margin-left: 8px">Prefer synchronized lyrics when available</span>
            </div>
            <div class="row" style="justify-content: flex-start; margin-top: 10px">
              <input @input="(e) => updateConfig('tag_fetch_with_filename', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagFetchWithFilename" />
              <span style="margin-left: 8px">Use filename for fetching when tags are missing</span>
            </div>
            <div class="row" style="justify-content: flex-start; margin-top: 10px">
              <input @input="(e) => updateConfig('tag_date_as_year', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagDateAsYear" />
              <span style="margin-left: 8px">Retrieve DATE with YYYY format</span>
            </div>
            <div class="row" style="justify-content: flex-start; margin-top: 10px">
              <input @input="(e) => updateConfig('tag_originaldate_as_year', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagOriginaldateAsYear" />
              <span style="margin-left: 8px">Retrieve ORIGINALDATE with YYYY format</span>
            </div>
          </div>
          <div class="frame" style="padding: 15px">
            <p style="text-align: start; margin-bottom: 10px; margin-top: 0px">Add Padding to the Following Tags:</p>
            <div class="row" style="justify-content: start">
              <div class="column" style="align-items: start; flex-basis: 50%; gap: 4px">
                <div class="row">
                  <input @input="(e) => updateConfig('tag_pad_track', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagPadTrack" />
                  <span>Track</span>
                </div>
                <div class="row">
                  <input @input="(e) => updateConfig('tag_pad_track_total', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagPadTrackTotal" />
                  <span>Track Total</span>
                </div>
              </div>
              <div class="column" style="align-items: start; gap: 4px">
                <div class="row">
                  <input @input="(e) => updateConfig('tag_pad_disk', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagPadDisk" />
                  <span>Disk</span>
                </div>
                <div class="row">
                  <input @input="(e) => updateConfig('tag_pad_disk_total', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagPadDiskTotal" />
                  <span>Disk Total</span>
                </div>
              </div>
            </div>
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

.group-download p {
  text-align: left;
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

.group-local-files {
  text-align: left;
}

.group-tags .row {
  justify-content: flex-start;
}

.group-tags .column .row {
  align-items: center;
}

.group-tags .column .row:not(:first-child) {
  margin-top: 10px;
}

.group-other .row {
  align-items: center;
}

.row input + span {
  margin-left: 8px;
}
</style>

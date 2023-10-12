<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { open } from "@tauri-apps/api/dialog";
import { downloadDir } from "@tauri-apps/api/path";
import { writeText } from '@tauri-apps/api/clipboard';
import { IconFolder, IconFolderFilled, IconTextSize, IconCheck, IconFileFilled, IconBookmarksFilled, IconList } from "@tabler/icons-vue";
import SettingsGroup from "../components/SettingsGroup.vue";

import { appConfig, filterColumnsDownload, globalEmitter, tagSeparators } from "../helpers";

const downloadInputValue = ref(appConfig.downloadPath);
const fileTemplateInput = ref(null);
const fileTemplateValue = ref(appConfig.fileTemplate);
const localFilesInputValue = ref(appConfig.directoryViewPath);

async function setDownloadPath() {
  const defaultPath = await downloadDir()
    .then((result) => result)
    .catch((err) => globalEmitter.emit("notification-add", { type: "Error", origin: "setDownloadPath", msg: err }));
  const path = await open({ defaultPath: defaultPath, directory: true, multiple: false})
    .then((result) => result)
    .catch((err) => globalEmitter.emit("notification-add", { type: "Error", origin: "setDownloadPath", msg: err })); 
  if (path !== null) {
    await invoke("update_config", { key: "download_path", value: path })
      .then((_) => "")
      .catch((err) => globalEmitter.emit("notification-add", { type: "Error", origin: "setDownloadPath", msg: err }));
    appConfig.downloadPath = path;
    downloadInputValue.value = path;
    globalEmitter.emit("instant-notification-add", { type: "Info", origin: "Settings", msg: "Setting Updated!" });
  };
};

async function saveFileTemplate() {
  await invoke("update_config", { key: "file_template", value: fileTemplateInput.value.value })
    .then((_) => "")
    .catch((err) => globalEmitter.emit("notification-add", { type: "Error", origin: "saveFileTemplate", msg: err }));
  appConfig.fileTemplate = fileTemplateInput.value.value
  globalEmitter.emit("instant-notification-add", { type: "Info", origin: "Settings", msg: "Setting Updated!" });
}

async function updateConfig(key, value) {
  await invoke("update_config", { key: key, value: value })
    .then((_) => "")
    .catch((err) => globalEmitter.emit("notification-add", { type: "Error", origin: "updateConfig", msg: err }));
}

async function copyEventTargetToClipboard(event) {
  await writeText(event.target.innerHTML)
    .then((_) => "")
    .catch((err) => globalEmitter.emit("notification-add", { type: "Error", origin: "copyEventTargetToClipboard", msg: err }));
};

async function setLocalFilesPath() {
  const path = await open({ directory: true, multiple: false})
    .then((result) => result)
    .catch((err) => globalEmitter.emit("notification-add", { type: "Error", origin: "setLocalFilesPath", msg: err })); 
  if (path !== null) {
    await invoke("update_config", { key: "directory_view_path", value: path })
      .then((_) => "")
      .catch((err) => globalEmitter.emit("notification-add", { type: "Error", origin: "setLocalFilesPath", msg: err }));
    appConfig.directoryViewPath = path;
    localFilesInputValue.value = path;
    globalEmitter.emit("instant-notification-add", { type: "Info", origin: "Settings", msg: "Setting Updated!" });
  };
}

</script>

<template>
  <div class="container" style="overflow-y: auto;">
    <div class="column">
      <SettingsGroup :body-as-column="true" class="group-download">
        <template #head>
          <IconFolderFilled size="30" class="icon setting-icon"/>
          <h1>Download</h1>
        </template>
        <template #body>
          <p>Download Path</p>
          <div class="row">
            <input :value="downloadInputValue" type="text" placeholder="Open..." style="flex-grow: 1;">
            <button style="margin-left: 15px;" @click="setDownloadPath">
              <IconFolder color="var(--color-text)"/>
            </button>
          </div>
          <div class="row" style="justify-content: flex-start; padding-top: 20px;">
            <input @input="e => updateConfig('overwrite_downloads', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.overwriteDownloads">
            <p style="margin-left: 8px;">Overwrite Existing Files</p>
          </div>
        </template>
      </SettingsGroup>
      <SettingsGroup :body-as-column="true" class="group-file-template">
        <template #head>
          <IconTextSize size="30" class="icon setting-icon"/>
          <h1>File Template</h1>
        </template>
        <template #body>
          <p style="font-size: 1.0em;">Downloaded files will be saved with the following name</p>
          <div class="row">
            <input style="flex-grow: 1;" type="text" spellcheck="false" placeholder="File name template..." :value="fileTemplateValue" ref="fileTemplateInput">
            <button @click.prevent="saveFileTemplate" style="margin-left: 10px;">
              <IconCheck color="var(--color-text)"/>
            </button>
          </div>
          <div class="row" style="margin-top: 10px; padding-left: 10px; padding-right: 10px; justify-content: flex-start; flex-wrap: wrap;">
            <p>Available Variables:</p>
            <p v-for="item in filterColumnsDownload" :key="item.id" @click="copyEventTargetToClipboard">{{ `%${item.key}%` }}</p>
          </div>
        </template>
      </SettingsGroup>
      <SettingsGroup :body-as-column="true" class="group-local-files">
        <template #head>
          <IconFileFilled size="30" class="icon setting-icon"/>
          <h1>Local Files</h1>
        </template>
        <template #body>
          <p>Default Directory</p>
          <div class="row">
            <input :value="localFilesInputValue" type="text" placeholder="Open..." style="flex-grow: 1;">
            <button style="margin-left: 15px;" @click="setLocalFilesPath">
              <IconFolder color="var(--color-text)"/>
            </button>
          </div>
        </template>
      </SettingsGroup>
      <SettingsGroup :body-as-column="true" class="group-tags">
        <template #head>
          <IconBookmarksFilled size="30" class="icon setting-icon"/>
          <h1>Deezer Tags</h1>
        </template>
        <template #body>
          <div class="row" style="flex-grow: 1;">
            <div class="column" style="flex-basis: 50%; align-self: flex-start;">
              <div class="row">
                <input @input="e => updateConfig('tag_dz_title', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagDzTitle">
                <span>Title</span>
              </div>
              <div class="row">
                <input @input="e => updateConfig('tag_dz_artist', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagDzArtist">
                <span>Artist</span>
              </div>
              <div class="row">
                <input @input="e => updateConfig('tag_dz_album', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagDzAlbum">
                <span>Album</span>
              </div>
              <div class="row">
                <input @input="e => updateConfig('tag_dz_track_number', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagDzTrackNumber">
                <span>Track Number</span>
              </div>
              <div class="row">
                <input @input="e => updateConfig('tag_dz_track_total', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagDzTrackTotal">
                <span>Track Total</span>
              </div>
              <div class="row">
                <input @input="e => updateConfig('tag_dz_disc_number', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagDzDiscNumber">
                <span>Disc Number</span>
              </div>
              <div class="row">
                <input @input="e => updateConfig('tag_dz_disc_total', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagDzDiscTotal">
                <span>Disc Total</span>
              </div>
              <div class="row">
                <input @input="e => updateConfig('tag_dz_album_artist', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagDzAlbumArtist">
                <span>Album Artist</span>
              </div>
              <div class="row">
                <input @input="e => updateConfig('tag_dz_genre', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagDzGenre">
                <span>Genre</span>
              </div>
              <div class="row">
                <input @input="e => updateConfig('tag_dz_year', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagDzYear">
                <span>Year</span>
              </div>
              <div class="row">
                <input @input="e => updateConfig('tag_dz_date', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagDzDate">
                <span>Date</span>
              </div>
              <div class="row">
                <input @input="e => updateConfig('tag_dz_itunesadvisory', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagDzItunesadvisory">
                <span>Explicit Lyrics</span>
              </div>
            </div>
            <div class="column" style="justify-content: flex-start; align-self: flex-start;">
              <div class="row">
                <input @input="e => updateConfig('tag_dz_isrc', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagDzIsrc">
                <span>ISRC</span>
              </div>
              <div class="row">
                <input @input="e => updateConfig('tag_dz_length', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagDzLength">
                <span>Track Length</span>
              </div>
              <div class="row">
                <input @input="e => updateConfig('tag_dz_barcode', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagDzBarcode">
                <span>Album Barcode (UPC)</span>
              </div>
              <div class="row">
                <input @input="e => updateConfig('tag_dz_bpm', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagDzBpm">
                <span>BPM</span>
              </div>
              <div class="row">
                <input @input="e => updateConfig('tag_dz_replaygain_track_gain', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagDzReplaygainTrackGain">
                <span>Replay Gain</span>
              </div>
              <div class="row">
                <input @input="e => updateConfig('tag_dz_label', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagDzLabel">
                <span>Label</span>
              </div>
              <div class="row">
                <input @input="e => updateConfig('tag_dz_lyrics', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagDzLyrics">
                <span>Lyrics</span>
              </div>
              <div class="row">
                <input @input="e => updateConfig('tag_dz_copyright', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagDzCopyright">
                <span>Copyright</span>
              </div>
              <div class="row">
                <input @input="e => updateConfig('tag_dz_composer', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagDzComposer">
                <span>Composer</span>
              </div>
              <div class="row">
                <input @input="e => updateConfig('tag_dz_performer', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagDzPerformer">
                <span>Performer</span>
              </div>
              <div class="row">
                <input @input="e => updateConfig('tag_dz_source_id', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagDzSourceId">
                <span>Deezer Song ID</span>
              </div>
            </div>
          </div>
        </template>
      </SettingsGroup>
      <SettingsGroup :body-as-column="true" class="group-other">
        <template #head>
          <IconList size="30" class="icon setting-icon"/>
          <h1>Other</h1>
        </template>
        <template #body>
          <div class="row" style="justify-content: start;">
            <span style="margin-right: 8px;">Separate Retrieved Tags Using</span>
            <select name="select-tag-separator" :value="appConfig.tagSeparator" @change="e => {updateConfig('tag_separator', e.target.value); globalEmitter.emit('instant-notification-add', { type: 'Info', origin: 'Settings', msg: 'Setting Updated!' });}">
              <option v-for="(sep, i) in tagSeparators" :key="i" :value="sep">"{{ sep }}"</option>
            </select>
          </div>
          <div class="row" style="justify-content: flex-start;">
            <input @input="e => updateConfig('tag_prefer_sync_lyrics', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagPreferSyncLyrics">
            <p style="margin-left: 8px;">Prefer Synchronized Lyrics when Available</p>
          </div>
          <p style="text-align: start; margin-bottom: 4px; margin-top: 4px;">Add Padding to the Following Tags:</p>
          <div class="row" style="justify-content: start;">
            <div class="column" style="align-items: start; flex-basis: 50%; gap: 4px;">
              <div class="row">
                <input @input="e => updateConfig('tag_pad_track', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagPadTrack">
                <span>Track</span>
              </div>
              <div class="row">
                <input @input="e => updateConfig('tag_pad_track_total', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagPadTrackTotal">
                <span>Track Total</span>
              </div>
              <div class="row">
                <span style="margin-right: 8px;">Character</span>
                <input @change="e => updateConfig('tag_pad_track_char', e.target.value)" type="text" maxlength="1" class="input-number" :value="appConfig.tagPadTrackChar">
              </div>
            </div>
            <div class="column" style="align-items: start; gap: 4px;">
              <div class="row">
                <input @input="e => updateConfig('tag_pad_disc', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagPadDisc">
                <span>Disc</span>
              </div>
              <div class="row">
                <input @input="e => updateConfig('tag_pad_disc_total', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagPadDiscTotal">
                <span>Disc Total</span>
              </div>
              <div class="row">
                <span style="margin-right: 8px;">Character</span>
                <input @change="e => updateConfig('tag_pad_disc_char', e.target.value)" type="text" maxlength="1" class="input-number" :value="appConfig.tagPadDiscChar">
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

.input-number {
  width: 20px;
  padding: 1px 5px;
  border: 0px;
  border-radius: 2px;
  text-align: center;
}

</style>

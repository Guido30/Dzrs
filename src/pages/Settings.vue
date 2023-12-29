<script setup>
import { IconNotes, IconBrandStorybook, IconBrandDiscordFilled, IconFolder, IconWorldDownload, IconTextSize, IconCheck, IconFileFilled, IconBookmarksFilled, IconList } from "@tabler/icons-vue";

import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { open } from "@tauri-apps/api/dialog";
import { appWindow } from "@tauri-apps/api/window";
import { downloadDir, homeDir } from "@tauri-apps/api/path";
import { writeText } from "@tauri-apps/api/clipboard";
import { camelCase } from "lodash";

import SettingsGroup from "../components/SettingsGroup.vue";
import HeaderBar from "../components/HeaderBar.vue";

import { appConfig, filterColumnsDownload, tagSeparators } from "../globals";

const fileTemplateInput = ref(null);

async function discordLogin() {
  if (appConfig.discordStoreCredentials) {
    await updateBackendConfig("discord_email", appConfig.discordEmail);
    await updateBackendConfig("discord_password", appConfig.discordPassword);
  } else {
    await updateBackendConfig("discord_email", "");
    await updateBackendConfig("discord_password", "");
  }
  if (appConfig.discordEmail && appConfig.discordPassword) {
    const token = await invoke("discord_token", { email: appConfig.discordEmail, password: appConfig.discordPassword })
      .then((t) => t)
      .catch((err) => appWindow.emit("notification-add", { type: "Error", origin: "Discord", msg: err }));
    if (token) {
      await updateConfig("discord_token", token);
      await invoke("discord_authenticate", { token: token })
        .then(() => appWindow.emit("instant-notification-add", { type: "Info", origin: "Discord", msg: "Discord Login Successfull!" }))
        .catch((err) => appWindow.emit("notification-add", { type: "Error", origin: "Discord", msg: err }));
    }
  }
}

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
    appWindow.emit("instant-notification-add", { type: "Info", origin: "Settings", msg: "Setting Updated!" });
  }
}

async function setSlavartdlPath() {
  const defaultPath = await homeDir()
    .then((result) => result)
    .catch((err) => appWindow.emit("notification-add", { type: "Error", origin: "setSlavartdlPath", msg: err }));
  const path = await open({ defaultPath: defaultPath, directory: false, multiple: false })
    .then((result) => result)
    .catch((err) => appWindow.emit("notification-add", { type: "Error", origin: "setSlavartdlPath", msg: err }));
  if (path !== null) {
    await invoke("config_set", { key: "slavartdl_path", value: path }).catch((err) => appWindow.emit("notification-add", { type: "Error", origin: "setSlavartdlPath", msg: err }));
    appConfig.slavartdlPath = path;
    appWindow.emit("instant-notification-add", { type: "Info", origin: "Settings", msg: "Setting Updated!" });
  }
}

async function saveFileTemplate() {
  await invoke("config_set", { key: "file_template", value: fileTemplateInput.value.value }).catch((err) => appWindow.emit("notification-add", { type: "Error", origin: "saveFileTemplate", msg: err }));
  appConfig.fileTemplate = fileTemplateInput.value.value;
  appWindow.emit("instant-notification-add", { type: "Info", origin: "Settings", msg: "Setting Updated!" });
}

// Updates a single config entry in backend ONLY, the underlying command will persist the change into the config file
async function updateBackendConfig(key, value) {
  await invoke("config_set", { key: key, value: value }).catch((err) => appWindow.emit("notification-add", { type: "Error", origin: "updateBackendConfig", msg: err }));
}

// Updates a single config entry in both backend and frontend, the underlying command will persist the change into the config file
async function updateConfig(key, value) {
  await invoke("config_set", { key: key, value: value }).catch((err) => appWindow.emit("notification-add", { type: "Error", origin: "updateConfig", msg: err }));
  appConfig[camelCase(key)] = value;
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
    appWindow.emit("instant-notification-add", { type: "Info", origin: "Settings", msg: "Setting Updated!" });
  }
}

function downloadSourceToggleStyle(e) {
  const groupEl = e.target.parentNode.nextElementSibling;
  const spanEl = e.target.nextElementSibling;
  if (e.target.checked) {
    groupEl.classList.remove("disabled-el");
    spanEl.innerHTML = "Enabled";
  } else {
    groupEl.classList.add("disabled-el");
    spanEl.innerHTML = "Disabled";
  }
}
</script>

<template>
  <HeaderBar>
    <div class="row header-content" style="justify-content: start">
      <button style="padding: 2px 8px" @click="invoke('browse_cmd', { path: appConfig._path.replace('config.json', '') })">
        <div class="row clickable-effect" style="color: var(--color-text)">
          <IconNotes size="20" class="icon" style="margin-right: 3px" />
          <span>Open Config</span>
        </div>
      </button>
    </div>
  </HeaderBar>
  <div class="container" style="overflow-y: auto">
    <div class="column">
      <SettingsGroup :body-as-column="true" class="group-download">
        <template #head>
          <IconWorldDownload size="30" class="icon setting-icon" />
          <h1>Download</h1>
        </template>
        <template #body>
          <div class="row" style="margin-bottom: 5px">
            <p style="margin: auto 0px; flex-basis: 150px">Download Path</p>
            <input :value="appConfig.downloadPath" type="text" placeholder="Open..." style="flex-grow: 1" />
            <button style="margin-left: 15px" @click="setDownloadPath">
              <IconFolder size="18px" class="icon clickable-effect" />
            </button>
          </div>
          <p style="font-size: 1.3em; text-align: left; margin-bottom: 0px">Alternative Downloading Methods</p>
          <div class="discord-div column">
            <div class="row" style="position: absolute; left: 0px; top: 0px; margin: 15px; z-index: 1">
              <input
                type="checkbox"
                class="checkbox"
                style="height: 20px"
                @input="
                  (e) => {
                    updateBackendConfig('discord_enabled', String(e.target.checked));
                    downloadSourceToggleStyle(e);
                  }
                "
                :checked="appConfig.discordEnabled" />
              <span style="font-style: italic; font-size: 1.2em">{{ appConfig.discordEnabled ? "Enabled" : "Disabled" }}</span>
            </div>
            <div class="column" :class="{ 'disabled-el': !appConfig.discordEnabled }">
              <div class="row" style="font-size: 1.5em; margin-bottom: 4px">
                <IconBrandDiscordFilled class="icon" size="30px" />
                <p style="margin: auto 0px; padding-left: 5px">Discord (NOT IMPLEMENTED)</p>
              </div>
              <div class="row" style="gap: 10px; margin-top: 5px">
                <input type="text" style="flex-grow: 1" placeholder="..." :value="appConfig.discordToken" readonly />
                <button @click="updateConfig('discord_token', '')">Clear Token</button>
              </div>
              <div class="row" style="justify-content: space-between; gap: 10px">
                <form class="form column">
                  <p style="text-align: center; font-size: 1.2em; margin-bottom: 5px; margin-right: 0px">Login</p>
                  <input type="email" placeholder="Email..." v-model="appConfig.discordEmail" />
                  <input type="password" placeholder="Password..." v-model="appConfig.discordPassword" />
                  <div class="row" style="justify-content: space-between">
                    <div class="row">
                      <input style="height: 15px" @input="(e) => updateBackendConfig('discord_store_credentials', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.discordStoreCredentials" />
                      <span style="margin-left: 8px; margin-top: auto; margin-bottom: auto">Store credentials unencrypted</span>
                    </div>
                    <div class="row">
                      <button type="submit" @click.prevent="discordLogin">Refresh</button>
                    </div>
                  </div>
                </form>
                <div class="row" style="flex-grow: 1; padding-right: 30px">
                  <div class="column" style="justify-content: start; flex-grow: 1; margin-top: 15px">
                    <p style="text-align: center; font-size: 1.2em; margin-bottom: 5px; margin-right: 0px; margin-top: 0px">Discord IDS</p>
                    <div class="row" style="margin-bottom: 5px; gap: 5px">
                      <p style="margin: auto 0px; flex-basis: 80px">Channel ID</p>
                      <input type="text" style="flex-grow: 1" placeholder="..." v-model="appConfig.discordChannelId" />
                    </div>
                    <div class="row" style="gap: 5px; margin-bottom: 5px">
                      <p style="margin: auto 0px; flex-basis: 80px">Bot ID</p>
                      <input type="text" style="flex-grow: 1" placeholder="..." v-model="appConfig.discordBotId" />
                    </div>
                    <div class="row" style="justify-content: end">
                      <button
                        @click="
                          updateBackendConfig('discord_channel_id', String(appConfig.discordChannelId));
                          updateBackendConfig('discord_bot_id', String(appConfig.discordBotId));
                          appWindow.emit('instant-notification-add', { type: 'Info', origin: 'Settings', msg: 'Setting Updated!' });
                        ">
                        Save Ids
                      </button>
                    </div>
                  </div>
                </div>
              </div>
              <p style="font-style: italic">Note: Its recommended to use a secondary account and join only the slavart server</p>
            </div>
          </div>
          <div class="slavartdl-div column">
            <div class="row" style="position: absolute; left: 0px; top: 0px; margin: 15px; z-index: 1">
              <input
                type="checkbox"
                class="checkbox"
                style="height: 20px"
                @input="
                  (e) => {
                    updateBackendConfig('slavartdl_enabled', String(e.target.checked));
                    downloadSourceToggleStyle(e);
                  }
                "
                :checked="appConfig.slavartdlEnabled" />
              <span style="font-style: italic; font-size: 1.2em">{{ appConfig.slavartdlEnabled ? "Enabled" : "Disabled" }}</span>
            </div>
            <div class="column" :class="{ 'disabled-el': !appConfig.slavartdlEnabled }">
              <div class="row" style="font-size: 1.5em; margin-bottom: 4px">
                <IconBrandStorybook class="icon" size="30px" />
                <p style="margin: auto 0px; padding-left: 5px">SlavartDL</p>
              </div>
              <div>
                <div class="row" style="margin-top: 5px">
                  <p style="margin: auto 0px; flex-basis: 150px">SlavartDL Path</p>
                  <input :value="appConfig.slavartdlPath" type="text" placeholder="Select..." style="flex-grow: 1" />
                  <button style="margin-left: 15px" @click="setSlavartdlPath">
                    <IconFolder size="18px" class="icon clickable-effect" />
                  </button>
                </div>
                <p style="font-style: italic">Note: SlavartDL needs to be manually configured!</p>
              </div>
            </div>
          </div>
        </template>
      </SettingsGroup>
      <SettingsGroup :body-as-column="true" class="group-local-files">
        <template #head>
          <IconFileFilled size="30" class="icon setting-icon" />
          <h1>Local Files</h1>
        </template>
        <template #body>
          <div class="row">
            <p style="margin: auto 0px; flex-basis: 150px">Default Directory</p>
            <input :value="appConfig.directoryViewPath" type="text" placeholder="Open..." style="flex-grow: 1" />
            <button style="margin-left: 15px" @click="setLocalFilesPath">
              <IconFolder size="20px" class="icon clickable-effect" />
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
            <input style="flex-grow: 1" type="text" spellcheck="false" placeholder="File name template..." :value="appConfig.fileTemplate" ref="fileTemplateInput" />
            <button @click.prevent="saveFileTemplate" style="margin-left: 10px">
              <IconCheck size="18px" class="icon clickable-effect" />
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
                  <input @input="(e) => updateBackendConfig('tag_dz_title', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagDzTitle" />
                  <span>Title</span>
                </div>
                <div class="row">
                  <input @input="(e) => updateBackendConfig('tag_dz_artist', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagDzArtist" />
                  <span>Artist</span>
                </div>
                <div class="row">
                  <input @input="(e) => updateBackendConfig('tag_dz_album', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagDzAlbum" />
                  <span>Album</span>
                </div>
                <div class="row">
                  <input @input="(e) => updateBackendConfig('tag_dz_track_number', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagDzTrackNumber" />
                  <span>Track Number</span>
                </div>
                <div class="row">
                  <input @input="(e) => updateBackendConfig('tag_dz_track_total', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagDzTrackTotal" />
                  <span>Track Total</span>
                </div>
                <div class="row">
                  <input @input="(e) => updateBackendConfig('tag_dz_disk_number', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagDzDiskNumber" />
                  <span>Disk Number</span>
                </div>
                <div class="row">
                  <input @input="(e) => updateBackendConfig('tag_dz_disk_total', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagDzDiskTotal" />
                  <span>Disk Total</span>
                </div>
                <div class="row">
                  <input @input="(e) => updateBackendConfig('tag_dz_album_artist', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagDzAlbumArtist" />
                  <span>Album Artist</span>
                </div>
                <div class="row">
                  <input @input="(e) => updateBackendConfig('tag_dz_genre', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagDzGenre" />
                  <span>Genre</span>
                </div>
                <div class="row">
                  <input @input="(e) => updateBackendConfig('tag_dz_year', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagDzYear" />
                  <span>Year</span>
                </div>
                <div class="row">
                  <input @input="(e) => updateBackendConfig('tag_dz_date', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagDzDate" />
                  <span>Date</span>
                </div>
                <div class="row">
                  <input @input="(e) => updateBackendConfig('tag_dz_original_date', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagDzOriginalDate" />
                  <span>Original Date</span>
                </div>
                <div class="row">
                  <input @input="(e) => updateBackendConfig('tag_dz_itunesadvisory', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagDzItunesadvisory" />
                  <span>Explicit Lyrics</span>
                </div>
              </div>
              <div class="column" style="justify-content: flex-start; align-self: flex-start">
                <div class="row">
                  <input @input="(e) => updateBackendConfig('tag_dz_isrc', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagDzIsrc" />
                  <span>ISRC</span>
                </div>
                <div class="row">
                  <input @input="(e) => updateBackendConfig('tag_dz_length', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagDzLength" />
                  <span>Track Length</span>
                </div>
                <div class="row">
                  <input @input="(e) => updateBackendConfig('tag_dz_barcode', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagDzBarcode" />
                  <span>Album Barcode (UPC)</span>
                </div>
                <div class="row">
                  <input @input="(e) => updateBackendConfig('tag_dz_bpm', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagDzBpm" />
                  <span>BPM</span>
                </div>
                <div class="row">
                  <input @input="(e) => updateBackendConfig('tag_dz_explicit', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagDzExplicit" />
                  <span>Parental Advisory Rating</span>
                </div>
                <div class="row">
                  <input @input="(e) => updateBackendConfig('tag_dz_replaygain_track_gain', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagDzReplaygainTrackGain" />
                  <span>Replay Gain</span>
                </div>
                <div class="row">
                  <input @input="(e) => updateBackendConfig('tag_dz_label', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagDzLabel" />
                  <span>Label</span>
                </div>
                <div class="row">
                  <input @input="(e) => updateBackendConfig('tag_dz_organization', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagDzOrganization" />
                  <span>Organization</span>
                </div>
                <div class="row">
                  <input @input="(e) => updateBackendConfig('tag_dz_lyrics', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagDzLyrics" />
                  <span>Lyrics</span>
                </div>
                <div class="row">
                  <input @input="(e) => updateBackendConfig('tag_dz_copyright', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagDzCopyright" />
                  <span>Copyright</span>
                </div>
                <div class="row">
                  <input @input="(e) => updateBackendConfig('tag_dz_composer', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagDzComposer" />
                  <span>Composer</span>
                </div>
                <div class="row">
                  <input @input="(e) => updateBackendConfig('tag_dz_performer', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagDzPerformer" />
                  <span>Performer</span>
                </div>
                <div class="row">
                  <input @input="(e) => updateBackendConfig('tag_dz_source_id', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagDzSourceId" />
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
                  updateBackendConfig('tag_separator', e.target.value);
                  appWindow.emit('instant-notification-add', { type: 'Info', origin: 'Settings', msg: 'Setting Updated!' });
                }
              ">
              <option v-for="(sep, i) in tagSeparators" :key="i" :value="sep">"{{ sep }}"</option>
            </select>
          </div>
          <div class="frame" style="padding: 15px; margin-bottom: 15px">
            <div class="row" style="justify-content: flex-start">
              <input @input="(e) => updateBackendConfig('overwrite_downloads', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.overwriteDownloads" />
              <span style="margin-left: 8px">Overwrite existing files when downloading</span>
            </div>
            <div class="row" style="justify-content: flex-start; margin-top: 10px">
              <input @input="(e) => updateBackendConfig('tag_prefer_sync_lyrics', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagPreferSyncLyrics" />
              <span style="margin-left: 8px">Prefer synchronized lyrics when available</span>
            </div>
            <div class="row" style="justify-content: flex-start; margin-top: 10px">
              <input @input="(e) => updateBackendConfig('tag_fetch_with_filename', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagFetchWithFilename" />
              <span style="margin-left: 8px">Use filename for fetching when tags are missing</span>
            </div>
            <div class="row" style="justify-content: flex-start; margin-top: 10px">
              <input @input="(e) => updateBackendConfig('tag_date_as_year', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagDateAsYear" />
              <span style="margin-left: 8px">Retrieve DATE with YYYY format</span>
            </div>
            <div class="row" style="justify-content: flex-start; margin-top: 10px">
              <input @input="(e) => updateBackendConfig('tag_originaldate_as_year', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagOriginaldateAsYear" />
              <span style="margin-left: 8px">Retrieve ORIGINALDATE with YYYY format</span>
            </div>
          </div>
          <div class="frame" style="padding: 15px">
            <p style="text-align: start; margin-bottom: 10px; margin-top: 0px">Add Padding to the Following Tags:</p>
            <div class="row" style="justify-content: start">
              <div class="column" style="align-items: start; flex-basis: 50%; gap: 4px">
                <div class="row">
                  <input @input="(e) => updateBackendConfig('tag_pad_track', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagPadTrack" />
                  <span>Track</span>
                </div>
                <div class="row">
                  <input @input="(e) => updateBackendConfig('tag_pad_track_total', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagPadTrackTotal" />
                  <span>Track Total</span>
                </div>
              </div>
              <div class="column" style="align-items: start; gap: 4px">
                <div class="row">
                  <input @input="(e) => updateBackendConfig('tag_pad_disk', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagPadDisk" />
                  <span>Disk</span>
                </div>
                <div class="row">
                  <input @input="(e) => updateBackendConfig('tag_pad_disk_total', String(e.target.checked))" type="checkbox" class="checkbox" :checked="appConfig.tagPadDiskTotal" />
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

input {
  height: 1.7em;
  padding: 0.2em 1em;
}

button {
  padding: 0.2em 1em;
}

.group-download p {
  text-align: left;
}

.group-download .discord-div,
.group-download .slavartdl-div {
  position: relative;
  border-radius: 20px;
  margin: 20px;
  padding: 10px 20px;
  box-shadow: 6px 6px 5px var(--color-shadow);
}

.group-download .discord-div {
  background-color: #383fa188;
}

.group-download .slavartdl-div {
  background-color: #41425288;
}

.group-download .discord-div input,
.group-download .discord-div button {
  background-color: #10122e88;
}

.group-download .discord-div input:focus,
.group-download .discord-div button:hover {
  outline: none !important;
  border: 1px solid #515fff88;
}

.group-download .discord-div .form {
  width: 50%;
  margin: 0px;
  margin-top: 10px;
}

.group-download .discord-div .form p {
  min-width: 100px;
  margin-top: auto;
  margin-bottom: auto;
  margin-left: 0px;
  margin-right: 8px;
}

.group-download .discord-div .form input {
  flex-grow: 1;
  margin-bottom: 5px;
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

.disabled-el {
  opacity: 0.5;
}

.disabled-el * {
  pointer-events: none;
}
</style>

import { invoke } from "@tauri-apps/api/tauri";
import { ref } from "vue";

export const appConfig = JSON.parse(await invoke("config_get").then((res) => res), (key, value) => {
  if (value === "true" || value === "false") {
    return value === "true";
  }
  if (/^\d+$/.test(value)) {
    return parseInt(value);
  }
  return value;
});

export const filterColumnsDownload = ref([
  { key: "title", label: "Title", config: "", readonly: true, enabled: true, width: 20 },
  { key: "album", label: "Album", config: "", readonly: true, enabled: true, width: 15 },
  { key: "artist", label: "Artist", config: "", readonly: true, enabled: true, width: 15 },
  { key: "genre", label: "Genre", config: "filter_download_genre", readonly: false, enabled: appConfig.filterDownloadGenre, width: 10 },
  { key: "duration", label: "Duration", config: "", readonly: true, enabled: true, width: 6 },
  { key: "date", label: "Date", config: "filter_download_date", readonly: false, enabled: appConfig.filterDownloadDate, width: 10 },
  { key: "composer", label: "Composer", config: "filter_download_composer", readonly: false, enabled: appConfig.filterDownloadComposer, width: 8 },
  { key: "isrc", label: "Isrc", config: "filter_download_isrc", readonly: false, enabled: appConfig.filterDownloadIsrc, width: 8 },
  { key: "copyright", label: "Copyright", config: "filter_download_copyright", readonly: false, enabled: appConfig.filterDownloadCopyright, width: 15 },
  { key: "bitDepth", label: "Bit Depth", config: "filter_download_bitdepth", readonly: false, enabled: appConfig.filterDownloadBitdepth, width: 5 },
  { key: "samplingRate", label: "Sampling", config: "filter_download_samplingrate", readonly: false, enabled: appConfig.filterDownloadSamplingrate, width: 8 },
]);

export const filterColumnsDirView = ref([
  { key: "filename", label: "Filename", config: "", readonly: true, enabled: true },
  { key: "size", label: "Size", config: "", readonly: true, enabled: true },
  { key: "extension", label: "Extension", config: "filter_dirview_extension", readonly: false, enabled: appConfig.filterDirviewExtension },
  { key: "tagStatus", label: "Status", config: "", readonly: true, enabled: true },
]);

export const tagSeparators = [";", "; ", "/", "/ ", " / ", ",", ", ", " , "];
export const defaultDzrsTrackObject = await invoke("tracks_object").then((res) => res);

// Headerbar components share the same state using these globals
// Multiple Headerbar components are required to assign them specific templates into the slot based on the active page
// with a single header at the root of the app this would not be possibile
export const activePageNameGlobal = ref("Main");
export const showNotificationsGlobal = ref(false);
export const showNotificationDotGlobal = ref(false);

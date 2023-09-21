import { invoke } from "@tauri-apps/api/tauri";
import { ref } from "vue";
import mitt from "mitt";

export function parseFileName(fileData, template) {
  const fileName = template
    .replaceAll("%title%", fileData.title)
    .replaceAll("%album%", fileData.albumTitle)
    .replaceAll("%artist%", fileData.artist)
    .replaceAll("%genre%", fileData.genre)
    .replaceAll(
      "%duration%",
      `${Math.floor(fileData.duration / 60)}.${(fileData.duration % 60)
        .toString()
        .padStart(2, "0")}`
    )
    .replaceAll("%date%", fileData.date)
    .replaceAll("%composer%", fileData.composer)
    .replaceAll("%isrc%", fileData.isrc)
    .replaceAll("%copyright%", fileData.copyright)
    .replaceAll("%bitDepth%", fileData.bitDepth)
    .replaceAll("%samplingRate%", fileData.samplingRate)
    .replaceAll(/[<>:"\/\\|?*]/g, " ");
  return fileName;
}

export async function openFileBrowser(path) {
  await invoke("open_explorer", { path: path });
}

const config = JSON.parse(
  await invoke("get_config_values")
    .then((res) => res)
    .catch((_) => {}),
  (key, value) => {
    if (value === "true" || value === "false") {
      return value === "true";
    }
    if (/^\d+$/.test(value)) {
      return parseInt(value);
    }
    return value;
  }
);

export const filterColumnsDownload = ref([
  { key: "title", label: "Title", config: "", readonly: true, enabled: true },
  { key: "album", label: "Album", config: "", readonly: true, enabled: true },
  { key: "artist", label: "Artist", config: "", readonly: true, enabled: true },
  {
    key: "genre",
    label: "Genre",
    config: "filter_download_genre",
    readonly: false,
    enabled: config.filterDownloadGenre,
  },
  {
    key: "duration",
    label: "Duration",
    config: "",
    readonly: true,
    enabled: true,
  },
  {
    key: "date",
    label: "Date",
    config: "filter_download_date",
    readonly: false,
    enabled: config.filterDownloadDate,
  },
  {
    key: "composer",
    label: "Composer",
    config: "filter_download_composer",
    readonly: false,
    enabled: config.filterDownloadComposer,
  },
  {
    key: "isrc",
    label: "Isrc",
    config: "filter_download_isrc",
    readonly: false,
    enabled: config.filterDownloadIsrc,
  },
  {
    key: "copyright",
    label: "Copyright",
    config: "filter_download_copyright",
    readonly: false,
    enabled: config.filterDownloadCopyright,
  },
  {
    key: "bitDepth",
    label: "Bit Depth",
    config: "filter_download_bitdepth",
    readonly: false,
    enabled: config.filterDownloadBitdepth,
  },
  {
    key: "samplingRate",
    label: "Sampling",
    config: "filter_download_samplingrate",
    readonly: false,
    enabled: config.filterDownloadSamplingrate,
  },
]);

export const filterColumnsDirView = ref([
  {
    key: "filename",
    label: "Filename",
    config: "",
    readonly: true,
    enabled: true,
  },
  {
    key: "size",
    label: "Size",
    config: "",
    readonly: true,
    enabled: true,
  },
  {
    key: "extension",
    label: "Extension",
    config: "filter_dirview_extension",
    readonly: false,
    enabled: config.filterDirviewExtension,
  },
  {
    key: "tagStatus",
    label: "Status",
    config: "",
    readonly: true,
    enabled: true,
  },
]);

export const appConfig = config;
export const globalEmitter = mitt();

export default {};

import { invoke } from "@tauri-apps/api/tauri";
import { reactive } from "vue";
import mitt from "mitt";

export function parseFileName(fileData, template) {
  const fileName = template
    .replaceAll("%title%", fileData.title)
    .replaceAll("%album%", fileData.album_title)
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
    .replaceAll("%bitDepth%", fileData.bit_depth)
    .replaceAll("%samplingRate%", fileData.sampling_rate)
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

export const filterColumnsDownload = reactive([
  { key: "title", label: "Title", config: "", readonly: true, enabled: true },
  { key: "album", label: "Album", config: "", readonly: true, enabled: true },
  { key: "artist", label: "Artist", config: "", readonly: true, enabled: true },
  {
    key: "genre",
    label: "Genre",
    config: "filter_download_genre",
    readonly: false,
    enabled: config.filter_download_genre,
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
    enabled: config.filter_download_date,
  },
  {
    key: "composer",
    label: "Composer",
    config: "filter_download_composer",
    readonly: false,
    enabled: config.filter_download_composer,
  },
  {
    key: "isrc",
    label: "Isrc",
    config: "filter_download_isrc",
    readonly: false,
    enabled: config.filter_download_isrc,
  },
  {
    key: "copyright",
    label: "Copyright",
    config: "filter_download_copyright",
    readonly: false,
    enabled: config.filter_download_copyright,
  },
  {
    key: "bitDepth",
    label: "Bit Depth",
    config: "filter_download_bitdepth",
    readonly: false,
    enabled: config.filter_download_bitdepth,
  },
  {
    key: "samplingRate",
    label: "Sampling",
    config: "filter_download_samplingrate",
    readonly: false,
    enabled: config.filter_download_samplingrate,
  },
]);

export const filterColumnsDirView = reactive([
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
    enabled: config.filter_dirview_extension,
  },
  {
    key: "tagStatus",
    label: "Status",
    config: "",
    readonly: true,
    enabled: true,
  },
]);

export const fileIconPaths = {
  flac: "assets/icon-flac.png",
  mp3: "assets/icon-mp3.png",
  default: "assets/icon-file.png",
};

export const appConfig = config;
export const globalEmitter = mitt();

export default {};

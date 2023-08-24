import { invoke } from "@tauri-apps/api/tauri";
import { reactive } from "vue";

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
    .replaceAll("%samplingRate%", fileData.sampling_rate);
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

export const filterColumns = reactive([
  { key: "title", label: "Title", config: "", readonly: true, enabled: true },
  { key: "album", label: "Album", config: "", readonly: true, enabled: true },
  { key: "artist", label: "Artist", config: "", readonly: true, enabled: true },
  {
    key: "genre",
    label: "Genre",
    config: "filter_column_genre",
    readonly: false,
    enabled: config.filter_column_genre,
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
    config: "filter_column_date",
    readonly: false,
    enabled: config.filter_column_date,
  },
  {
    key: "composer",
    label: "Composer",
    config: "filter_column_composer",
    readonly: false,
    enabled: config.filter_column_composer,
  },
  {
    key: "isrc",
    label: "Isrc",
    config: "filter_column_isrc",
    readonly: false,
    enabled: config.filter_column_isrc,
  },
  {
    key: "copyright",
    label: "Copyright",
    config: "filter_column_copyright",
    readonly: false,
    enabled: config.filter_column_copyright,
  },
  {
    key: "bitDepth",
    label: "Bit Depth",
    config: "filter_column_bitdepth",
    readonly: false,
    enabled: config.filter_column_bitdepth,
  },
  {
    key: "samplingRate",
    label: "Sampling",
    config: "filter_column_samplingrate",
    readonly: false,
    enabled: config.filter_column_samplingrate,
  },
]);

export const appConfig = config;

export default {};

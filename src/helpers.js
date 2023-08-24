import { invoke } from "@tauri-apps/api/tauri";

export function parseFileName(fileData, template) {
  const fileName = template
    .replaceAll("%title%", fileData.title)
    .replaceAll("%album%", fileData.album_title)
    .replaceAll("%artist%", fileData.performer_name);
  return fileName;
}

export async function openFileBrowser(path) {
  await invoke("open_explorer", { path: path });
}

const fileTemplateVars = ["%title%", "%artist%", "%album%"];

const config = JSON.parse(
  await invoke("get_config_values")
    .then((res) => res)
    .catch((_) => {})
);

config.fileTemplateVars = fileTemplateVars;

export const appConfig = config;

export default {};

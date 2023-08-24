import { invoke } from "@tauri-apps/api/tauri";

export function parseFileName(fileData, template) {
  const fileName = template
    .replace("%title%", fileData.title)
    .replace("%album%", fileData.album_title)
    .replace("%artist%", fileData.performer_name);
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

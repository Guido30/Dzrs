import { invoke } from "@tauri-apps/api/tauri";

const response = await invoke("get_config_values")
  .then((res) => res)
  .catch((_) => "");

const appConfig = JSON.parse(response);

export default appConfig;

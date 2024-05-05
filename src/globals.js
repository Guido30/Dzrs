import { invoke } from "@tauri-apps/api/tauri";
import { ref, reactive } from "vue";

export const appConfig = reactive(
  JSON.parse(await invoke("config_get").then((res) => res), (key, value) => {
    if (value === "true" || value === "false") {
      return value === "true";
    }
    if (/^\d+$/.test(value)) {
      return parseInt(value);
    }
    return value;
  })
);

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

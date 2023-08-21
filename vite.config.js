import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import topLevelAwait from "vite-plugin-top-level-await";

export default defineConfig(async () => ({
  plugins: [
    vue(),
    topLevelAwait({
      promiseExportName: "__tla",
      promiseImportName: (i) => `__tla_${i}`,
    }),
  ],
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
  },
  envPrefix: ["VITE_", "TAURI_"],
}));

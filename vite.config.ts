import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";

// Tauri 开发时固定端口，构建产物输出到 dist/
export default defineConfig({
  plugins: [vue()],
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    watch: { ignored: ["**/src-tauri/**"] },
  },
  envPrefix: ["VITE_", "TAURI_"],
  build: {
    target: "es2021",
    minify: "esbuild",
    sourcemap: false,
    outDir: "dist",
    emptyOutDir: true,
  },
});

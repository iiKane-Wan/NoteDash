import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";

// Tauri 要求前端开发服务器使用固定端口，便于 Rust 侧 devUrl 指向
export default defineConfig({
  plugins: [vue()],
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    watch: {
      // 后端源码变动由 Tauri 自己监听，避免 Vite 重复刷新
      ignored: ["**/src-tauri/**"],
    },
  },
  envPrefix: ["VITE_", "TAURI_"],
  build: {
    // WebView2 基于 Chromium，可放心使用现代语法
    target: "chrome105",
    minify: "esbuild",
    sourcemap: false,
  },
});

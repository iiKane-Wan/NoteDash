import { ref } from "vue";
import { getCurrentWindow } from "@tauri-apps/api/window";
import type { ThemeMode } from "../api/settings";

const mode = ref<ThemeMode>("system");
const isDark = ref(false);

const media = window.matchMedia("(prefers-color-scheme: dark)");

/**
 * 同步原生窗口主题，让 Windows 标题栏跟随应用主题。
 * system 时传 null，交还系统决定（系统切换时会自动跟随）。
 */
function syncWindowTheme(next: ThemeMode, dark: boolean) {
  try {
    const theme = next === "system" ? null : dark ? "dark" : "light";
    getCurrentWindow()
      .setTheme(theme)
      .catch(() => {
        /* 权限缺失等情况忽略，不影响网页内主题 */
      });
  } catch {
    /* 非 Tauri 环境（纯浏览器调试）忽略 */
  }
}

/** 计算最终明暗并写入 html[data-theme]，全局 CSS 变量与原生标题栏随之切换 */
export function applyTheme(next: ThemeMode) {
  mode.value = next;
  const dark = next === "dark" || (next === "system" && media.matches);
  isDark.value = dark;
  document.documentElement.dataset.theme = dark ? "dark" : "light";
  syncWindowTheme(next, dark);
}

// 跟随系统时，系统明暗变化要实时响应
media.addEventListener("change", () => {
  if (mode.value === "system") applyTheme("system");
});

export function useTheme() {
  return { mode, isDark, applyTheme };
}

// 首屏先按系统主题渲染，避免出现无主题状态的闪烁
applyTheme("system");

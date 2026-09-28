import { ref } from "vue";
import type { ThemeMode } from "../api/settings";

const mode = ref<ThemeMode>("system");
const isDark = ref(false);

const media = window.matchMedia("(prefers-color-scheme: dark)");

/** 计算最终明暗并写入 html[data-theme]，全局 CSS 变量随之切换 */
export function applyTheme(next: ThemeMode) {
  mode.value = next;
  const dark = next === "dark" || (next === "system" && media.matches);
  isDark.value = dark;
  document.documentElement.dataset.theme = dark ? "dark" : "light";
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
import { ref } from "vue";
import * as api from "../api/settings";
import type { Settings, ThemeMode } from "../api/settings";
import { applyTheme } from "./useTheme";
import { showToast } from "./useToast";

const settings = ref<Settings>({
  autostart: false,
  globalShortcut: "",
  hideShortcut: "Escape",
  closeToTray: true,
  theme: "system",
});

/** 首次读取设置并应用主题 */
export async function loadSettings() {
  try {
    const loaded = await api.getSettings();
    settings.value = loaded;
    applyTheme(loaded.theme);
  } catch (err) {
    showToast(`读取设置失败：${err}`, "error");
  }
}

/**
 * 乐观更新：先刷新界面再落库，失败则整体回滚并弹出后端返回的提示
 * （例如快捷键被占用、开机自启写入失败）。
 */
async function patch(values: Partial<Settings>) {
  const previous = settings.value;
  const next: Settings = { ...previous, ...values };
  settings.value = next;
  applyTheme(next.theme);

  try {
    const saved = await api.updateSettings(next);
    settings.value = saved;
    applyTheme(saved.theme);
  } catch (err) {
    settings.value = previous;
    applyTheme(previous.theme);
    showToast(String(err), "error");
  }
}

/** 主题切换：界面即时刷新 */
function setTheme(mode: ThemeMode) {
  return patch({ theme: mode });
}

function setAutostart(enabled: boolean) {
  return patch({ autostart: enabled });
}

function setCloseToTray(enabled: boolean) {
  return patch({ closeToTray: enabled });
}

function setShortcut(value: string) {
  return patch({ globalShortcut: value });
}

function setHideShortcut(value: string) {
  return patch({ hideShortcut: value });
}

export function useSettings() {
  return {
    settings,
    setTheme,
    setAutostart,
    setCloseToTray,
    setShortcut,
    setHideShortcut,
  };
}
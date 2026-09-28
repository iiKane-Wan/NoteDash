import { invoke } from "@tauri-apps/api/core";

export type ThemeMode = "light" | "dark" | "system";

/** 默认全局快捷键，需与 Rust 侧 settings::DEFAULT_SHORTCUT 保持一致 */
export const DEFAULT_SHORTCUT = "Ctrl+Shift+N";

/** 默认隐藏主界面快捷键，需与 Rust 侧 settings::DEFAULT_HIDE_SHORTCUT 保持一致 */
export const DEFAULT_HIDE_SHORTCUT = "Escape";

/** 应用设置，字段与 Rust 侧 Settings 一一对应 */
export interface Settings {
  autostart: boolean;
  globalShortcut: string;
  /** 隐藏主界面快捷键（应用内生效） */
  hideShortcut: string;
  closeToTray: boolean;
  theme: ThemeMode;
}

/** 读取全部设置 */
export function getSettings(): Promise<Settings> {
  return invoke<Settings>("get_settings");
}

/**
 * 保存设置并即时应用。
 * 若全局快捷键被占用或格式非法，会抛错（消息可直接展示给用户），且不会落库。
 */
export function updateSettings(settings: Settings): Promise<Settings> {
  return invoke<Settings>("update_settings", { settings });
}
import { onBeforeUnmount, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { eventToShortcut } from "../utils/shortcut";
import { hasOpenModal } from "./useModalLayer";
import { useSettings } from "./useSettings";
import { showToast } from "./useToast";

/**
 * 隐藏主界面快捷键（应用内生效，依赖 WebView 焦点）。
 * 采用捕获阶段监听，保证在 Vditor 编辑区内部也能触发。
 */
export function useHideShortcut() {
  const { settings } = useSettings();

  async function hideMainWindow() {
    try {
      await invoke("hide_main_window");
    } catch (err) {
      showToast(`隐藏主界面失败：${err}`, "error");
    }
  }

  function onKeydown(event: KeyboardEvent) {
    const shortcut = settings.value.hideShortcut;
    if (!shortcut) return;

    // 正在录制快捷键时，按键归录制框处理
    const target = event.target as HTMLElement | null;
    if (target?.closest("[data-capturing]")) return;

    // 面板打开时让位于面板（Esc 优先关闭面板），不隐藏窗口
    if (hasOpenModal()) return;

    if (eventToShortcut(event, true) !== shortcut) return;

    event.preventDefault();
    void hideMainWindow();
  }

  onMounted(() => {
    window.addEventListener("keydown", onKeydown, true);
  });

  onBeforeUnmount(() => {
    window.removeEventListener("keydown", onKeydown, true);
  });
}

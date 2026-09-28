<script setup lang="ts">
import { onBeforeUnmount, onMounted, ref, watch } from "vue";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import MarkdownEditor from "./components/MarkdownEditor.vue";
import NoteList from "./components/NoteList.vue";
import SearchBar from "./components/SearchBar.vue";
import SettingsPanel from "./components/SettingsPanel.vue";
import ToastHost from "./components/ToastHost.vue";
import { useHideShortcut } from "./composables/useHideShortcut";
import { useNotes } from "./composables/useNotes";
import { loadSettings } from "./composables/useSettings";
import { showToast } from "./composables/useToast";

const {
  notes,
  current,
  keyword,
  saving,
  refresh,
  selectNote,
  addNote,
  removeNote,
  applyEdit,
  flushSave,
} = useNotes();

const showSettings = ref(false);
let searchTimer: number | undefined;
let unlisten: UnlistenFn | undefined;

// 注册应用内「隐藏主界面」快捷键监听
useHideShortcut();

// 搜索输入防抖 200ms，避免频繁请求后端
watch(keyword, () => {
  if (searchTimer) window.clearTimeout(searchTimer);
  searchTimer = window.setTimeout(() => void refresh(), 200);
});

// 窗口关闭前尽力落盘未保存内容
function onBeforeUnload() {
  void flushSave();
}

onMounted(async () => {
  void refresh();
  void loadSettings();
  window.addEventListener("beforeunload", onBeforeUnload);

  // 重复启动时，后端会聚焦本窗口并广播该事件
  unlisten = await listen("app://already-running", () => {
    showToast("应用已在运行，已为你打开主窗口");
  });
});

onBeforeUnmount(() => {
  window.removeEventListener("beforeunload", onBeforeUnload);
  unlisten?.();
});
</script>

<template>
  <div class="app">
    <aside class="sidebar">
      <div class="sidebar__head">
        <h1 class="brand">
          NoteDash<span class="brand__dot" />
        </h1>
        <p class="brand__sub">即时灵感速记</p>
      </div>

      <div class="sidebar__tools">
        <SearchBar />
        <button class="sidebar__new" type="button" @click="addNote">
          <span class="sidebar__new-plus">+</span>
          新建记录
        </button>
      </div>

      <NoteList
        :notes="notes"
        :active-id="current?.id ?? null"
        @select="selectNote"
        @remove="removeNote"
      />

      <footer class="sidebar__foot">
        <span>{{ notes.length }} 条记录</span>
        <button class="sidebar__settings" type="button" @click="showSettings = true">
          <svg viewBox="0 0 24 24" aria-hidden="true">
            <circle
              cx="12"
              cy="12"
              r="3.2"
              fill="none"
              stroke="currentColor"
              stroke-width="1.7"
            />
            <path
              d="M19.4 12c0-.4-.03-.8-.1-1.2l2-1.5-2-3.4-2.3.9a7.3 7.3 0 0 0-2-1.2L14.7 3h-3.4l-.3 2.6a7.3 7.3 0 0 0-2 1.2l-2.3-.9-2 3.4 2 1.5a7.4 7.4 0 0 0 0 2.4l-2 1.5 2 3.4 2.3-.9a7.3 7.3 0 0 0 2 1.2l.3 2.6h3.4l.3-2.6a7.3 7.3 0 0 0 2-1.2l2.3.9 2-3.4-2-1.5c.06-.4.1-.8.1-1.2Z"
              fill="none"
              stroke="currentColor"
              stroke-width="1.5"
              stroke-linejoin="round"
            />
          </svg>
          设置
        </button>
      </footer>
    </aside>

    <main class="stage">
      <MarkdownEditor
        v-if="current"
        :note="current"
        :saving="saving"
        @edit="applyEdit"
      />
      <div v-else class="stage__empty">
        <p class="stage__empty-glyph">”</p>
        <p class="stage__empty-title">选中左侧记录，或新建一条灵感</p>
        <p class="stage__empty-hint">
          正文支持 Markdown 即时渲染 · 右上角「大纲」可快速跳转标题
        </p>
      </div>
    </main>

    <SettingsPanel v-if="showSettings" @close="showSettings = false" />
    <ToastHost />
  </div>
</template>

<style scoped>
.app {
  display: grid;
  /* 左三分之一 / 右三分之二 */
  grid-template-columns: minmax(280px, 1fr) 2fr;
  /* 锁定行高：否则内容会把 auto 行撑开，导致整体溢出视口、内部区域反而滚不动 */
  grid-template-rows: minmax(0, 1fr);
  height: 100vh;
  overflow: hidden;
}

.sidebar {
  display: flex;
  flex-direction: column;
  min-width: 0;
  /* 约束自身高度，保证列表只在本列内部滚动 */
  min-height: 0;
  overflow: hidden;
  border-right: 1px solid var(--line);
  background: var(--paper);
}

.sidebar__head {
  padding: 20px 16px 4px;
}

.brand {
  display: flex;
  align-items: baseline;
  gap: 3px;
  margin: 0;
  font-family: var(--font-serif);
  font-size: 21px;
  font-weight: 700;
  letter-spacing: 0.02em;
  color: var(--ink);
}

.brand__dot {
  width: 6px;
  height: 6px;
  border-radius: 50%;
  background: var(--accent);
}

.brand__sub {
  margin: 5px 0 0;
  font-size: 11.5px;
  letter-spacing: 0.22em;
  color: var(--ink-faint);
}

.sidebar__tools {
  display: flex;
  flex-direction: column;
  gap: 8px;
  padding: 14px 16px 10px;
}

.sidebar__new {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  height: 34px;
  border: 1px solid var(--line-strong);
  border-radius: 8px;
  background: transparent;
  font-family: var(--font-sans);
  font-size: 13px;
  color: var(--ink-soft);
  cursor: pointer;
  transition: color 0.16s ease, border-color 0.16s ease, background 0.16s ease;
}

.sidebar__new:hover {
  color: var(--accent-contrast);
  border-color: var(--accent);
  background: var(--accent);
}

.sidebar__new-plus {
  font-size: 15px;
  line-height: 1;
}

.sidebar__foot {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 8px 12px 8px 16px;
  border-top: 1px solid var(--line);
  font-size: 11px;
  letter-spacing: 0.08em;
  color: var(--ink-faint);
}

.sidebar__settings {
  display: inline-flex;
  align-items: center;
  gap: 5px;
  height: 26px;
  padding: 0 9px;
  border: none;
  border-radius: 7px;
  background: transparent;
  font-family: var(--font-sans);
  font-size: 11.5px;
  letter-spacing: 0.06em;
  color: var(--ink-soft);
  cursor: pointer;
  transition: color 0.15s ease, background 0.15s ease;
}

.sidebar__settings svg {
  width: 14px;
  height: 14px;
}

.sidebar__settings:hover {
  color: var(--accent);
  background: var(--accent-soft);
}

.stage {
  position: relative;
  min-width: 0;
  /* 约束高度，编辑器内容不得撑开本列 */
  min-height: 0;
  overflow: hidden;
  background: var(--paper);
}

.stage__empty {
  height: 100%;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  text-align: center;
  padding: 0 40px;
}

.stage__empty-glyph {
  margin: 0 0 4px;
  font-family: var(--font-serif);
  font-size: 84px;
  line-height: 0.6;
  color: var(--line-strong);
}

.stage__empty-title {
  margin: 22px 0 0;
  font-family: var(--font-serif);
  font-size: 17px;
  font-weight: 600;
  color: var(--ink-soft);
}

.stage__empty-hint {
  margin: 10px 0 0;
  font-size: 12.5px;
  color: var(--ink-faint);
}
</style>

import { ref } from "vue";
import * as api from "../api/notes";
import type { Note, NoteSummary } from "../api/notes";

/** 模块级单例状态：整个应用共用一份笔记状态 */
const notes = ref<NoteSummary[]>([]);
const current = ref<Note | null>(null);
const keyword = ref("");
const saving = ref(false);

let saveTimer: number | undefined;

/** 拉取列表（带关键词过滤） */
async function refresh() {
  notes.value = await api.listNotes(keyword.value.trim() || undefined);
}

/** 选中笔记；切换前先落盘未保存内容，避免丢失 */
async function selectNote(id: number) {
  if (current.value?.id === id) return;
  await flushSave();
  current.value = await api.getNote(id);
}

/** 新建笔记并直接进入编辑 */
async function addNote() {
  await flushSave();
  current.value = await api.createNote();
  await refresh();
}

/** 删除笔记；若删除的是当前项，同时清空编辑区 */
async function removeNote(id: number) {
  if (saveTimer) {
    window.clearTimeout(saveTimer);
    saveTimer = undefined;
  }
  await api.deleteNote(id);
  if (current.value?.id === id) {
    current.value = null;
  }
  await refresh();
}

/** 防抖保存：停止输入 500ms 后写库 */
function scheduleSave() {
  if (saveTimer) window.clearTimeout(saveTimer);
  saveTimer = window.setTimeout(() => void flushSave(), 500);
}

/** 立即保存；只同步列表项标题与时间，不重排列表，避免编辑时列表跳动 */
async function flushSave() {
  if (saveTimer) {
    window.clearTimeout(saveTimer);
    saveTimer = undefined;
  }
  const note = current.value;
  if (!note) return;
  saving.value = true;
  try {
    const saved = await api.updateNote(note.id, note.title, note.content);
    const item = notes.value.find((n) => n.id === saved.id);
    if (item) {
      item.title = saved.title;
      item.updatedAt = saved.updatedAt;
    }
  } finally {
    saving.value = false;
  }
}

/** 编辑区内容变更入口：先更新本地状态，再触发防抖保存 */
function applyEdit(title: string, content: string) {
  if (!current.value) return;
  current.value.title = title;
  current.value.content = content;
  scheduleSave();
}

export function useNotes() {
  return {
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
  };
}
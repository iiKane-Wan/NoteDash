import { invoke } from "@tauri-apps/api/core";

/** 左侧列表项 */
export interface NoteSummary {
  id: number;
  title: string;
  updatedAt: string;
}

/** 右侧编辑区完整笔记 */
export interface Note {
  id: number;
  title: string;
  content: string;
  createdAt: string;
  updatedAt: string;
}

/** 查询列表，keyword 为空时返回全部 */
export function listNotes(keyword?: string): Promise<NoteSummary[]> {
  return invoke<NoteSummary[]>("list_notes", { keyword: keyword ?? null });
}

/** 读取单条笔记 */
export function getNote(id: number): Promise<Note | null> {
  return invoke<Note | null>("get_note", { id });
}

/** 新建空笔记，返回新记录 */
export function createNote(): Promise<Note> {
  return invoke<Note>("create_note");
}

/** 保存标题与正文 */
export function updateNote(
  id: number,
  title: string,
  content: string,
): Promise<NoteSummary> {
  return invoke<NoteSummary>("update_note", { id, title, content });
}

/** 删除笔记 */
export function deleteNote(id: number): Promise<void> {
  return invoke<void>("delete_note", { id });
}
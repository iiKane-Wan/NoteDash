<script setup lang="ts">
import { ref } from "vue";
import type { NoteSummary } from "../api/notes";
import { formatTime } from "../utils/time";

defineProps<{
  notes: NoteSummary[];
  activeId: number | null;
}>();

const emit = defineEmits<{
  select: [id: number];
  remove: [id: number];
}>();

/** 二次确认删除：首次点击进入待确认态，3 秒内再点才真正删除 */
const pendingId = ref<number | null>(null);
let pendingTimer: number | undefined;

function onDelete(id: number) {
  if (pendingId.value === id) {
    pendingId.value = null;
    emit("remove", id);
    return;
  }
  pendingId.value = id;
  if (pendingTimer) window.clearTimeout(pendingTimer);
  pendingTimer = window.setTimeout(() => {
    pendingId.value = null;
  }, 3000);
}
</script>

<template>
  <ul class="list">
    <li v-if="notes.length === 0" class="list__empty">
      还没有记录，点上方「新建」开始吧
    </li>

    <li
      v-for="note in notes"
      :key="note.id"
      class="list__item"
      :class="{ 'is-active': note.id === activeId }"
      @click="emit('select', note.id)"
    >
      <div class="list__body">
        <p class="list__title">{{ note.title.trim() || "未命名灵感" }}</p>
        <p class="list__time">{{ formatTime(note.updatedAt) }}</p>
      </div>

      <button
        class="list__delete"
        :class="{ 'is-pending': pendingId === note.id }"
        type="button"
        :title="pendingId === note.id ? '再次点击确认删除' : '删除'"
        @click.stop="onDelete(note.id)"
      >
        <template v-if="pendingId === note.id">确认?</template>
        <template v-else>×</template>
      </button>
    </li>
  </ul>
</template>

<style scoped>
.list {
  list-style: none;
  margin: 0;
  padding: 4px 0 12px;
  overflow-y: auto;
  flex: 1;
  /* 显式声明最小高度，确保内容变多时只在本区域内滚动，不撑高侧栏 */
  min-height: 0;
}

.list__empty {
  padding: 32px 16px;
  text-align: center;
  font-size: 12.5px;
  line-height: 1.7;
  color: var(--ink-faint);
}

.list__item {
  position: relative;
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 9px 12px 9px 16px;
  cursor: pointer;
  border-left: 3px solid transparent;
  transition: background 0.15s ease, border-color 0.15s ease;
}

.list__item::after {
  content: "";
  position: absolute;
  left: 16px;
  right: 12px;
  bottom: 0;
  height: 1px;
  background: var(--line);
  opacity: 0.7;
}

.list__item:last-child::after {
  display: none;
}

.list__item:hover {
  background: var(--hover-bg);
}

.list__item.is-active {
  background: var(--paper-soft);
  border-left-color: var(--accent);
}

.list__item.is-active .list__title {
  color: var(--accent);
}

.list__body {
  flex: 1;
  min-width: 0;
}

.list__title {
  margin: 0;
  font-family: var(--font-serif);
  font-size: 14.5px;
  font-weight: 600;
  color: var(--ink);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  transition: color 0.15s ease;
}

.list__time {
  margin: 3px 0 0;
  font-size: 11px;
  letter-spacing: 0.02em;
  color: var(--ink-faint);
}

.list__delete {
  flex: none;
  min-width: 22px;
  height: 22px;
  padding: 0 6px;
  border: 1px solid transparent;
  border-radius: 6px;
  background: transparent;
  color: var(--ink-faint);
  font-size: 14px;
  line-height: 1;
  opacity: 0;
  cursor: pointer;
  transition: opacity 0.15s ease, color 0.15s ease, background 0.15s ease;
}

.list__item:hover .list__delete {
  opacity: 1;
}

.list__delete:hover {
  color: var(--accent);
  background: var(--accent-soft);
}

.list__delete.is-pending {
  opacity: 1;
  border-color: var(--accent);
  color: var(--accent-contrast);
  background: var(--accent);
  font-size: 11px;
}
</style>
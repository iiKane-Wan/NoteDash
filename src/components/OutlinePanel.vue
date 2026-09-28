<script setup lang="ts">
import { onBeforeUnmount, onMounted } from "vue";
import { useModalLayer } from "../composables/useModalLayer";
import type { OutlineItem } from "../utils/outline";

defineProps<{ items: OutlineItem[] }>();

const emit = defineEmits<{
  close: [];
  jump: [item: OutlineItem];
}>();

// 登记为模态层，使全局隐藏快捷键让位于面板处理
useModalLayer();

/** Esc 关闭面板；捕获阶段监听，避免被编辑器内部逻辑拦截 */
function onEscape(event: KeyboardEvent) {
  if (event.key !== "Escape") return;
  const target = event.target as HTMLElement | null;
  if (target?.closest("[data-capturing]")) return;
  emit("close");
}

onMounted(() => window.addEventListener("keydown", onEscape, true));
onBeforeUnmount(() => window.removeEventListener("keydown", onEscape, true));
</script>

<template>
  <!-- 透明遮罩：点击面板外任意位置关闭 -->
  <div class="outline-mask" @click.self="emit('close')">
    <aside class="outline">
      <header class="outline__head">
        <span class="outline__label">大纲</span>
        <button class="outline__close" type="button" title="关闭" @click="emit('close')">
          ×
        </button>
      </header>

      <p v-if="items.length === 0" class="outline__empty">
        当前内容还没有标题<br />用 # 号创建标题试试
      </p>

      <ul v-else class="outline__list">
        <li v-for="item in items" :key="item.id">
          <button
            class="outline__item"
            :class="`outline__item--h${item.level}`"
            :style="{ paddingLeft: `${12 + (item.level - 1) * 14}px` }"
            type="button"
            @click="emit('jump', item)"
          >
            {{ item.text }}
          </button>
        </li>
      </ul>
    </aside>
  </div>
</template>

<style scoped>
.outline-mask {
  position: absolute;
  inset: 0;
  z-index: 30;
  background: rgba(36, 32, 26, 0.06);
}

.outline {
  position: absolute;
  top: 12px;
  right: 12px;
  bottom: 12px;
  width: 268px;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  border: 1px solid var(--line-strong);
  border-radius: 12px;
  background: var(--paper-soft);
  box-shadow: var(--shadow);
  animation: outline-in 0.18s ease-out;
}

@keyframes outline-in {
  from {
    opacity: 0;
    transform: translateX(14px);
  }
  to {
    opacity: 1;
    transform: translateX(0);
  }
}

.outline__head {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 14px 10px;
  border-bottom: 1px solid var(--line);
}

.outline__label {
  font-family: var(--font-serif);
  font-size: 14px;
  font-weight: 700;
  letter-spacing: 0.14em;
  color: var(--ink);
}

.outline__close {
  width: 22px;
  height: 22px;
  border: none;
  border-radius: 6px;
  background: transparent;
  color: var(--ink-faint);
  font-size: 16px;
  line-height: 1;
  cursor: pointer;
  transition: color 0.15s ease, background 0.15s ease;
}

.outline__close:hover {
  color: var(--accent);
  background: var(--accent-soft);
}

.outline__empty {
  margin: 0;
  padding: 28px 16px;
  text-align: center;
  font-size: 12.5px;
  line-height: 1.9;
  color: var(--ink-faint);
}

.outline__list {
  list-style: none;
  margin: 0;
  padding: 6px 0;
  overflow-y: auto;
}

.outline__item {
  display: block;
  width: 100%;
  padding-top: 6px;
  padding-bottom: 6px;
  padding-right: 12px;
  border: none;
  background: transparent;
  text-align: left;
  font-family: var(--font-sans);
  font-size: 13px;
  line-height: 1.5;
  color: var(--ink-soft);
  cursor: pointer;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  transition: color 0.15s ease, background 0.15s ease;
}

.outline__item:hover {
  color: var(--accent);
  background: var(--accent-soft);
}

.outline__item--h1 {
  font-weight: 700;
  color: var(--ink);
}

.outline__item--h2 {
  font-weight: 600;
  color: var(--ink);
}

.outline__item--h3,
.outline__item--h4,
.outline__item--h5,
.outline__item--h6 {
  font-weight: 400;
}
</style>
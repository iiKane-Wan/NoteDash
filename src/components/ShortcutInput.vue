<script setup lang="ts">
import { ref } from "vue";
import { eventToShortcut } from "../utils/shortcut";

const props = withDefaults(
  defineProps<{
    modelValue: string;
    /** 允许把 Esc 录为主键；为 false 时 Esc 仅用于取消录制 */
    allowEscape?: boolean;
  }>(),
  { allowEscape: false },
);

const emit = defineEmits<{
  change: [value: string];
}>();

const capturing = ref(false);

function onKeydown(event: KeyboardEvent) {
  if (!capturing.value) return;
  // 捕获期间拦截按键，避免触发编辑器或面板的默认行为
  event.preventDefault();
  event.stopPropagation();

  // 未允许 Esc 时，Esc 仅用于取消录制
  if (event.code === "Escape" && !props.allowEscape) {
    capturing.value = false;
    return;
  }

  const combo = eventToShortcut(event, props.allowEscape);
  if (!combo) return;

  emit("change", combo);
  capturing.value = false;
}
</script>

<template>
  <button
    class="shortcut"
    :class="{ 'is-capturing': capturing }"
    :data-capturing="capturing ? 'true' : undefined"
    type="button"
    @click="capturing = true"
    @keydown="onKeydown"
    @blur="capturing = false"
  >
    <template v-if="capturing">请按下快捷键…</template>
    <template v-else-if="modelValue">{{ modelValue }}</template>
    <template v-else>未设置</template>
  </button>
</template>

<style scoped>
.shortcut {
  flex: 1;
  min-width: 0;
  height: 32px;
  padding: 0 12px;
  border: 1px solid var(--line-strong);
  border-radius: 8px;
  background: var(--paper-soft);
  font-family: var(--font-sans);
  font-size: 12.5px;
  letter-spacing: 0.04em;
  color: var(--ink);
  text-align: left;
  cursor: pointer;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  transition: border-color 0.16s ease, box-shadow 0.16s ease, color 0.16s ease;
}

.shortcut:hover {
  border-color: var(--ink-faint);
}

.shortcut.is-capturing {
  border-color: var(--accent);
  color: var(--accent);
  box-shadow: 0 0 0 3px var(--accent-soft);
}
</style>

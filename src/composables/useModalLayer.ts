import { onBeforeUnmount, onMounted, ref } from "vue";

/** 当前打开的模态层数量（设置面板、大纲面板等） */
const modalDepth = ref(0);

/**
 * 在模态组件内调用，挂载/卸载时自动增减计数。
 * 隐藏主界面快捷键据此让位于面板处理（例如 Esc 优先关闭面板）。
 */
export function useModalLayer() {
  onMounted(() => {
    modalDepth.value += 1;
  });
  onBeforeUnmount(() => {
    modalDepth.value = Math.max(0, modalDepth.value - 1);
  });
}

export function hasOpenModal(): boolean {
  return modalDepth.value > 0;
}

import { ref } from "vue";

export type ToastKind = "info" | "success" | "error";

export interface ToastItem {
  id: number;
  text: string;
  kind: ToastKind;
}

const toasts = ref<ToastItem[]>([]);
let seq = 0;

/** 弹出轻量提示，默认 2.8 秒后自动消失 */
export function showToast(text: string, kind: ToastKind = "info", duration = 2800) {
  const id = ++seq;
  toasts.value = [...toasts.value, { id, text, kind }];
  window.setTimeout(() => {
    toasts.value = toasts.value.filter((item) => item.id !== id);
  }, duration);
}

export function useToast() {
  return { toasts };
}
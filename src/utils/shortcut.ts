/**
 * 快捷键解析：把浏览器 KeyboardEvent 转成应用内统一格式的字符串。
 * 录制（ShortcutInput）与触发（useHideShortcut）共用同一套规则，保证格式一致。
 */

const MODIFIER_CODES = new Set([
  "ControlLeft",
  "ControlRight",
  "ShiftLeft",
  "ShiftRight",
  "AltLeft",
  "AltRight",
  "MetaLeft",
  "MetaRight",
]);

/** 非字母数字键映射，与后端 global-hotkey 支持的别名保持一致 */
const NAMED_KEYS: Record<string, string> = {
  Space: "Space",
  Enter: "Enter",
  Tab: "Tab",
  Backquote: "`",
  Minus: "-",
  Equal: "=",
  BracketLeft: "[",
  BracketRight: "]",
  Backslash: "\\",
  Semicolon: ";",
  Quote: "'",
  Comma: ",",
  Period: ".",
  Slash: "/",
  ArrowUp: "Up",
  ArrowDown: "Down",
  ArrowLeft: "Left",
  ArrowRight: "Right",
  Home: "Home",
  End: "End",
  PageUp: "PageUp",
  PageDown: "PageDown",
  Insert: "Insert",
  Delete: "Delete",
};

export function isModifierCode(code: string): boolean {
  return MODIFIER_CODES.has(code);
}

/** 将 KeyboardEvent.code 转成主键名；无法识别时返回 null */
export function mainKey(code: string): string | null {
  if (/^Key[A-Z]$/.test(code)) return code.slice(3);
  if (/^Digit[0-9]$/.test(code)) return code.slice(5);
  if (/^F([1-9]|1[0-9]|2[0-4])$/.test(code)) return code;
  if (code === "Escape") return "Escape";
  return NAMED_KEYS[code] ?? null;
}

/**
 * 把按键事件转成 "Ctrl+Shift+N" / "Escape" 形式的字符串。
 * allowEscape 为 false 时忽略 Esc（由调用方用于取消录制）。
 */
export function eventToShortcut(
  event: KeyboardEvent,
  allowEscape = false,
): string | null {
  if (MODIFIER_CODES.has(event.code)) return null;
  if (event.code === "Escape" && !allowEscape) return null;

  const key = mainKey(event.code);
  if (!key) return null;

  const parts: string[] = [];
  if (event.ctrlKey) parts.push("Ctrl");
  if (event.altKey) parts.push("Alt");
  if (event.shiftKey) parts.push("Shift");
  if (event.metaKey) parts.push("Super");
  parts.push(key);

  return parts.join("+");
}
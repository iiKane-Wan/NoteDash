<script setup lang="ts">
import { onBeforeUnmount, onMounted } from "vue";
import { DEFAULT_HIDE_SHORTCUT, DEFAULT_SHORTCUT } from "../api/settings";
import type { ThemeMode } from "../api/settings";
import { useModalLayer } from "../composables/useModalLayer";
import { useSettings } from "../composables/useSettings";
import ShortcutInput from "./ShortcutInput.vue";

const emit = defineEmits<{
  close: [];
}>();

const {
  settings,
  setTheme,
  setAutostart,
  setCloseToTray,
  setShortcut,
  setHideShortcut,
} = useSettings();

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

const THEMES: { value: ThemeMode; label: string }[] = [
  { value: "light", label: "浅色" },
  { value: "dark", label: "深色" },
  { value: "system", label: "跟随系统" },
];
</script>

<template>
  <div class="mask" @click.self="emit('close')">
    <section class="panel">
      <header class="panel__head">
        <h2 class="panel__title">设置</h2>
        <button class="panel__close" type="button" title="关闭" @click="emit('close')">
          ×
        </button>
      </header>

      <div class="panel__body">
        <!-- 开机自启 -->
        <div class="item">
          <div class="item__info">
            <p class="item__title">开机自启</p>
            <p class="item__desc">登录 Windows 后自动启动 NoteDash</p>
          </div>
          <button
            class="switch"
            :class="{ 'is-on': settings.autostart }"
            type="button"
            role="switch"
            :aria-checked="settings.autostart"
            @click="setAutostart(!settings.autostart)"
          >
            <span class="switch__knob" />
          </button>
        </div>

        <!-- 全局快捷键 -->
        <div class="item item--stack">
          <div class="item__info">
            <p class="item__title">打开主页面快捷键</p>
            <p class="item__desc">在任意程序中按下即可唤起主窗口，Esc 取消录制</p>
          </div>
          <div class="item__control">
            <ShortcutInput
              :model-value="settings.globalShortcut"
              @change="setShortcut"
            />
            <button
              class="ghost-btn"
              type="button"
              @click="setShortcut(DEFAULT_SHORTCUT)"
            >
              重置
            </button>
            <button
              class="ghost-btn"
              type="button"
              :disabled="!settings.globalShortcut"
              @click="setShortcut('')"
            >
              清除
            </button>
          </div>
        </div>

        <!-- 隐藏主界面快捷键 -->
        <div class="item item--stack">
          <div class="item__info">
            <p class="item__title">隐藏主界面快捷键</p>
            <p class="item__desc">
              焦点在软件内时按下即隐藏窗口，可从托盘菜单或全局快捷键唤回
            </p>
          </div>
          <div class="item__control">
            <ShortcutInput
              :model-value="settings.hideShortcut"
              allow-escape
              @change="setHideShortcut"
            />
            <button
              class="ghost-btn"
              type="button"
              @click="setHideShortcut(DEFAULT_HIDE_SHORTCUT)"
            >
              重置
            </button>
            <button
              class="ghost-btn"
              type="button"
              :disabled="!settings.hideShortcut"
              @click="setHideShortcut('')"
            >
              清除
            </button>
          </div>
        </div>

        <!-- 关闭保留托盘 -->
        <div class="item">
          <div class="item__info">
            <p class="item__title">关闭窗口后保留托盘</p>
            <p class="item__desc">点击关闭按钮时隐藏到托盘，可从托盘菜单退出</p>
          </div>
          <button
            class="switch"
            :class="{ 'is-on': settings.closeToTray }"
            type="button"
            role="switch"
            :aria-checked="settings.closeToTray"
            @click="setCloseToTray(!settings.closeToTray)"
          >
            <span class="switch__knob" />
          </button>
        </div>

        <!-- 主题 -->
        <div class="item item--stack">
          <div class="item__info">
            <p class="item__title">主题</p>
            <p class="item__desc">切换后整个界面与编辑器立即刷新</p>
          </div>
          <div class="segmented">
            <button
              v-for="theme in THEMES"
              :key="theme.value"
              class="segmented__item"
              :class="{ 'is-active': settings.theme === theme.value }"
              type="button"
              @click="setTheme(theme.value)"
            >
              {{ theme.label }}
            </button>
          </div>
        </div>
      </div>
    </section>
  </div>
</template>

<style scoped>
.mask {
  position: fixed;
  inset: 0;
  z-index: 60;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 24px;
  background: var(--mask);
  backdrop-filter: blur(2px);
}

.panel {
  width: 100%;
  max-width: 430px;
  max-height: 100%;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  border: 1px solid var(--line-strong);
  border-radius: 14px;
  background: var(--paper-soft);
  box-shadow: var(--shadow);
  animation: panel-in 0.18s ease-out;
}

@keyframes panel-in {
  from {
    opacity: 0;
    transform: translateY(10px) scale(0.99);
  }
  to {
    opacity: 1;
    transform: translateY(0) scale(1);
  }
}

.panel__head {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px 20px 14px;
  border-bottom: 1px solid var(--line);
}

.panel__title {
  margin: 0;
  font-family: var(--font-serif);
  font-size: 16px;
  font-weight: 700;
  letter-spacing: 0.16em;
  color: var(--ink);
}

.panel__close {
  width: 26px;
  height: 26px;
  border: none;
  border-radius: 7px;
  background: transparent;
  color: var(--ink-faint);
  font-size: 17px;
  line-height: 1;
  cursor: pointer;
  transition: color 0.15s ease, background 0.15s ease;
}

.panel__close:hover {
  color: var(--accent);
  background: var(--accent-soft);
}

.panel__body {
  padding: 6px 20px 18px;
  overflow-y: auto;
}

.item {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 15px 0;
  border-bottom: 1px solid var(--line);
}

.item:last-child {
  border-bottom: none;
  padding-bottom: 4px;
}

.item--stack {
  flex-direction: column;
  align-items: stretch;
  gap: 10px;
}

.item__info {
  flex: 1;
  min-width: 0;
}

.item__title {
  margin: 0;
  font-size: 13.5px;
  font-weight: 600;
  color: var(--ink);
}

.item__desc {
  margin: 4px 0 0;
  font-size: 11.5px;
  line-height: 1.6;
  color: var(--ink-faint);
}

.item__control {
  display: flex;
  align-items: center;
  gap: 8px;
}

/* 开关 */
.switch {
  flex: none;
  position: relative;
  width: 42px;
  height: 24px;
  padding: 0;
  border: 1px solid var(--line-strong);
  border-radius: 999px;
  background: transparent;
  cursor: pointer;
  transition: background 0.18s ease, border-color 0.18s ease;
}

.switch.is-on {
  border-color: var(--accent);
  background: var(--accent);
}

.switch__knob {
  position: absolute;
  top: 3px;
  left: 3px;
  width: 16px;
  height: 16px;
  border-radius: 50%;
  background: var(--ink-faint);
  transition: transform 0.18s ease, background 0.18s ease;
}

.switch.is-on .switch__knob {
  transform: translateX(18px);
  background: var(--accent-contrast);
}

/* 次级按钮 */
.ghost-btn {
  flex: none;
  height: 32px;
  padding: 0 12px;
  border: 1px solid var(--line-strong);
  border-radius: 8px;
  background: transparent;
  font-family: var(--font-sans);
  font-size: 12.5px;
  color: var(--ink-soft);
  cursor: pointer;
  transition: color 0.16s ease, border-color 0.16s ease, background 0.16s ease;
}

.ghost-btn:hover:not(:disabled) {
  color: var(--accent);
  border-color: var(--accent);
  background: var(--accent-soft);
}

.ghost-btn:disabled {
  opacity: 0.4;
  cursor: not-allowed;
}

/* 主题分段选择 */
.segmented {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 4px;
  padding: 3px;
  border: 1px solid var(--line);
  border-radius: 9px;
  background: var(--paper);
}

.segmented__item {
  height: 30px;
  border: none;
  border-radius: 6px;
  background: transparent;
  font-family: var(--font-sans);
  font-size: 12.5px;
  color: var(--ink-soft);
  cursor: pointer;
  transition: color 0.15s ease, background 0.15s ease;
}

.segmented__item:hover {
  color: var(--ink);
}

.segmented__item.is-active {
  color: var(--accent-contrast);
  background: var(--accent);
}
</style>
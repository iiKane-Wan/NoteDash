<script setup lang="ts">
import { onBeforeUnmount, onMounted, ref, watch } from "vue";
import Vditor from "vditor";
import "vditor/dist/index.css";
import type { Note } from "../api/notes";
import { useTheme } from "../composables/useTheme";
import { collectOutline, type OutlineItem } from "../utils/outline";
import OutlinePanel from "./OutlinePanel.vue";

const props = defineProps<{
  note: Note;
  saving: boolean;
}>();

const emit = defineEmits<{
  edit: [title: string, content: string];
}>();

const { isDark } = useTheme();

const editorRef = ref<HTMLDivElement | null>(null);
const showOutline = ref(false);
const outlineItems = ref<OutlineItem[]>([]);
/** Vditor 的编辑区根节点，用于提取标题与滚动 */
let irRoot: HTMLElement | null = null;

let vditor: Vditor | null = null;
/** 程序写入内容期间置位，避免 setValue 触发的 input 回调被当成用户编辑 */
let loading = false;

function setContent(content: string) {
  if (!vditor) return;
  loading = true;
  vditor.setValue(content, true);
  window.setTimeout(() => {
    loading = false;
  }, 0);
}

/** 主题切换时同步 Vditor：内置明暗类、内容主题与代码高亮主题 */
function syncVditorTheme() {
  if (!vditor) return;
  vditor.setTheme(
    isDark.value ? "dark" : "classic",
    isDark.value ? "dark" : "light",
    isDark.value ? "github-dark" : "github",
  );
}

/** 标题输入：同步正文一起提交，触发父级防抖保存 */
function onTitleInput(event: Event) {
  const value = (event.target as HTMLInputElement).value;
  emit("edit", value, vditor?.getValue() ?? props.note.content);
}

function openOutline() {
  if (!irRoot) return;
  outlineItems.value = collectOutline(irRoot);
  showOutline.value = true;
}

/** 点击大纲项：滚动到对应标题并关闭面板 */
function jumpTo(item: OutlineItem) {
  showOutline.value = false;
  item.el.scrollIntoView({ behavior: "smooth", block: "start" });
}

onMounted(() => {
  vditor = new Vditor(editorRef.value as HTMLDivElement, {
    // ir = 即时渲染（所见即所得），输入 Markdown 语法立刻呈现格式
    mode: "ir",
    theme: isDark.value ? "dark" : "classic",
    height: "100%",
    width: "100%",
    // 资源指向 public/vditor（本地离线），不使用默认 CDN
    cdn: "/vditor",
    preview: {
      theme: {
        current: isDark.value ? "dark" : "light",
        path: "/vditor/dist/css/content-theme",
      },
      hljs: { style: isDark.value ? "github-dark" : "github" },
    },
    cache: { enable: false },
    outline: { enable: false, position: "left" },
    resize: { enable: false },
    counter: { enable: false },
    toolbar: [
      "headings",
      "bold",
      "italic",
      "strike",
      "|",
      "list",
      "ordered-list",
      "check",
      "quote",
      "|",
      "code",
      "inline-code",
      "link",
      "table",
      "|",
      "undo",
      "redo",
      "|",
      "fullscreen",
    ],
    input: (value) => {
      if (loading) return;
      emit("edit", props.note.title, value);
    },
    after: () => {
      irRoot = editorRef.value?.querySelector<HTMLElement>(".vditor-ir") ?? null;
      setContent(props.note.content);
    },
  });
});

// 切换笔记：重置编辑器内容并关闭可能开着的大纲面板
watch(
  () => props.note.id,
  () => {
    showOutline.value = false;
    setContent(props.note.content);
  },
);

// 主题实时切换
watch(isDark, () => syncVditorTheme());

onBeforeUnmount(() => {
  vditor?.destroy();
  vditor = null;
});
</script>

<template>
  <section class="editor">
    <header class="editor__head">
      <input
        class="editor__title"
        type="text"
        :value="note.title"
        placeholder="给这条灵感起个标题…"
        spellcheck="false"
        @input="onTitleInput"
      />

      <div class="editor__meta">
        <span class="editor__state" :class="{ 'is-saving': saving }">
          {{ saving ? "保存中" : "已保存" }}
        </span>
        <button class="editor__outline-btn" type="button" @click="openOutline">
          <svg viewBox="0 0 16 16" aria-hidden="true">
            <path
              d="M2 3.5h12M4.5 8h9.5M7 12.5h7"
              fill="none"
              stroke="currentColor"
              stroke-width="1.4"
              stroke-linecap="round"
            />
          </svg>
          大纲
        </button>
      </div>
    </header>

    <div class="editor__body">
      <div ref="editorRef" class="editor__vditor"></div>
      <OutlinePanel
        v-if="showOutline"
        :items="outlineItems"
        @close="showOutline = false"
        @jump="jumpTo"
      />
    </div>
  </section>
</template>

<style scoped>
.editor {
  display: flex;
  flex-direction: column;
  height: 100%;
  min-width: 0;
}

.editor__head {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 14px 20px 12px;
  border-bottom: 1px solid var(--line);
  background: var(--paper-soft);
}

.editor__title {
  flex: 1;
  min-width: 0;
  border: none;
  outline: none;
  background: transparent;
  font-family: var(--font-serif);
  font-size: 21px;
  font-weight: 700;
  letter-spacing: 0.01em;
  color: var(--ink);
}

.editor__title::placeholder {
  color: var(--ink-faint);
  font-weight: 400;
}

.editor__meta {
  display: flex;
  align-items: center;
  gap: 12px;
  flex: none;
}

.editor__state {
  font-size: 11.5px;
  letter-spacing: 0.06em;
  color: var(--ink-faint);
  transition: color 0.2s ease;
}

.editor__state.is-saving {
  color: var(--accent);
}

.editor__outline-btn {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  height: 30px;
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

.editor__outline-btn svg {
  width: 14px;
  height: 14px;
}

.editor__outline-btn:hover {
  color: var(--accent);
  border-color: var(--accent);
  background: var(--accent-soft);
}

.editor__body {
  position: relative;
  flex: 1;
  min-height: 0;
  background: var(--paper);
}

.editor__vditor {
  height: 100%;
}

/* 抹平 Vditor 默认外观：全部配色改为引用应用主题变量，
   这样浅色/深色/跟随系统切换时编辑器会同步刷新，不再出现白色割裂感 */
.editor__vditor :deep(.vditor) {
  border: none;
  border-radius: 0;
  background: transparent;
  --border-color: var(--line);
  --second-color: var(--hover-bg);
  --panel-background-color: var(--paper-soft);
  --panel-shadow: var(--shadow);
  --toolbar-background-color: var(--paper-soft);
  --toolbar-icon-color: var(--ink-soft);
  --toolbar-icon-hover-color: var(--accent);
  --textarea-background-color: var(--paper);
  --textarea-text-color: var(--ink);
  --heading-border-color: var(--line);
  --blockquote-color: var(--ink-soft);
  --ir-heading-color: var(--ink);
  --ir-title-color: var(--ink-faint);
  --count-background-color: var(--hover-bg);
  --resize-icon-color: var(--ink-faint);
  --resize-background-color: var(--paper-soft);
  --resize-hover-background-color: var(--accent);
  --resize-hover-icon-color: #fff;
}

.editor__vditor :deep(.vditor-toolbar) {
  border-bottom: 1px solid var(--line);
  padding-left: 14px;
}

/* 内容区背景一律透明，统一透出 .editor__body 的主题底色 */
.editor__vditor :deep(.vditor-content),
.editor__vditor :deep(.vditor-ir),
.editor__vditor :deep(.vditor-sv),
.editor__vditor :deep(.vditor-wysiwyg),
.editor__vditor :deep(.vditor-preview),
.editor__vditor :deep(.vditor-reset) {
  background-color: transparent;
}

.editor__vditor :deep(.vditor-reset) {
  font-family: var(--font-sans);
  color: var(--ink);
}

/* 标题沿用衬线，与整体气质一致 */
.editor__vditor :deep(.vditor-ir .vditor-reset > h1),
.editor__vditor :deep(.vditor-ir .vditor-reset > h2),
.editor__vditor :deep(.vditor-ir .vditor-reset > h3) {
  font-family: var(--font-serif);
}

/* 引用与代码块背景跟随主题 */
.editor__vditor :deep(.vditor-reset blockquote) {
  border-left: 3px solid var(--line-strong);
  color: var(--ink-soft);
}

.editor__vditor :deep(.vditor-reset pre) {
  background-color: var(--code-bg);
}

.editor__vditor :deep(.vditor-reset code:not(.hljs)) {
  background-color: var(--code-bg);
  color: var(--ink);
}

.editor__vditor :deep(.vditor-reset pre code) {
  background-color: transparent;
}

/* ir 模式下的 Markdown 标记符号弱化 */
.editor__vditor :deep(.vditor-ir__marker) {
  color: var(--ink-faint);
}
</style>
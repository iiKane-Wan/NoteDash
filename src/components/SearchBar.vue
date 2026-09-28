<script setup lang="ts">
import { useNotes } from "../composables/useNotes";

const { keyword } = useNotes();
</script>

<template>
  <div class="search">
    <svg class="search__icon" viewBox="0 0 16 16" aria-hidden="true">
      <circle cx="7" cy="7" r="4.6" fill="none" stroke="currentColor" stroke-width="1.4" />
      <path d="M10.6 10.6 14 14" stroke="currentColor" stroke-width="1.4" stroke-linecap="round" />
    </svg>
    <input
      v-model="keyword"
      class="search__input"
      type="text"
      placeholder="搜索标题或内容"
      spellcheck="false"
    />
    <button
      v-if="keyword"
      class="search__clear"
      type="button"
      title="清空搜索"
      @click="keyword = ''"
    >
      ×
    </button>
  </div>
</template>

<style scoped>
.search {
  position: relative;
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 0 10px;
  height: 34px;
  border: 1px solid var(--line);
  border-radius: 8px;
  background: var(--paper-soft);
  transition: border-color 0.18s ease, box-shadow 0.18s ease;
}

.search:focus-within {
  border-color: var(--accent);
  box-shadow: 0 0 0 3px var(--accent-soft);
}

.search__icon {
  width: 14px;
  height: 14px;
  flex: none;
  color: var(--ink-faint);
}

.search__input {
  flex: 1;
  min-width: 0;
  border: none;
  outline: none;
  background: transparent;
  font-family: var(--font-sans);
  font-size: 13px;
  color: var(--ink);
}

.search__input::placeholder {
  color: var(--ink-faint);
}

.search__clear {
  flex: none;
  width: 18px;
  height: 18px;
  line-height: 1;
  border: none;
  border-radius: 50%;
  background: transparent;
  color: var(--ink-faint);
  font-size: 15px;
  cursor: pointer;
  transition: color 0.15s ease, background 0.15s ease;
}

.search__clear:hover {
  color: var(--accent);
  background: var(--accent-soft);
}
</style>
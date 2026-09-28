<script setup lang="ts">
import { useToast } from "../composables/useToast";

const { toasts } = useToast();
</script>

<template>
  <div class="toast-host">
    <TransitionGroup name="toast">
      <div
        v-for="item in toasts"
        :key="item.id"
        class="toast"
        :class="`toast--${item.kind}`"
      >
        {{ item.text }}
      </div>
    </TransitionGroup>
  </div>
</template>

<style scoped>
.toast-host {
  position: fixed;
  top: 18px;
  left: 50%;
  transform: translateX(-50%);
  z-index: 90;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
  pointer-events: none;
}

.toast {
  max-width: 420px;
  padding: 9px 16px;
  border: 1px solid var(--line-strong);
  border-radius: 9px;
  background: var(--paper-soft);
  box-shadow: var(--shadow);
  font-size: 12.5px;
  line-height: 1.5;
  color: var(--ink);
}

.toast--error {
  border-color: var(--accent);
  color: var(--accent);
}

.toast--success {
  border-color: var(--line-strong);
}

.toast-enter-active,
.toast-leave-active {
  transition: opacity 0.2s ease, transform 0.2s ease;
}

.toast-enter-from,
.toast-leave-to {
  opacity: 0;
  transform: translateY(-8px);
}
</style>
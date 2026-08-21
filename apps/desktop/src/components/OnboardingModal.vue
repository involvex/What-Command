<script setup lang="ts">
import { ref } from "vue";

const props = defineProps<{
  visible: boolean;
}>();

const emit = defineEmits<{
  (e: "update:visible", val: boolean): void;
}>();

const step = ref(1);

function finish() {
  localStorage.setItem("wc_onboarded", "true");
  emit("update:visible", false);
}
</script>

<template>
  <div v-if="props.visible" class="onboarding-backdrop">
    <div class="onboarding-modal card">
      <div class="onboarding-header">
        <span class="badge badge--primary">Welcome to What Command</span>
        <span class="muted text-xs">Step {{ step }} of 3</span>
      </div>

      <!-- Step 1: Welcome & Overview -->
      <div v-if="step === 1" class="onboarding-body">
        <h2 class="t-title-md">Your Offline Terminal Companion</h2>
        <p class="muted">
          What Command gives you instant access to thousands of tldr pages, AI command
          generation, and a safe educational terminal sandbox.
        </p>
        <ul class="feature-list">
          <li>🔍 Instant fuzzy search & Cmd/Ctrl+K global palette</li>
          <li>💻 Simulated terminal playground with safety guardrails</li>
          <li>🤖 Multi-turn AI chat supporting cloud gateways & local GGUF models</li>
        </ul>
      </div>

      <!-- Step 2: AI & Local GGUF Setup -->
      <div v-if="step === 2" class="onboarding-body">
        <h2 class="t-title-md">Choose Your AI Power</h2>
        <p class="muted">
          You can use cloud providers like OpenCode Zen or Kilo Gateway, or load local
          GGUF models (e.g. Gemma 2B) for fully offline, private operation.
        </p>
        <div class="tip-box">
          💡 You can configure API keys and GGUF model weights anytime in the
          <strong>More & Settings</strong> tab.
        </div>
      </div>

      <!-- Step 3: Keyboard Shortcuts -->
      <div v-if="step === 3" class="onboarding-body">
        <h2 class="t-title-md">Power Shortcuts</h2>
        <div class="shortcut-table">
          <div class="shortcut-row">
            <kbd class="mono">Cmd / Ctrl + K</kbd>
            <span>Open Global Command Palette</span>
          </div>
          <div class="shortcut-row">
            <kbd class="mono">Esc</kbd>
            <span>Close Overlays</span>
          </div>
          <div class="shortcut-row">
            <kbd class="mono">Swipe Left/Right</kbd>
            <span>Switch Mobile Navigation Tabs</span>
          </div>
        </div>
      </div>

      <div class="onboarding-footer">
        <button
          v-if="step > 1"
          class="btn btn--ghost btn--sm"
          type="button"
          @click="step--"
        >
          Back
        </button>
        <div class="flex-1"></div>
        <button
          v-if="step < 3"
          class="btn btn--primary btn--sm"
          type="button"
          @click="step++"
        >
          Next
        </button>
        <button v-else class="btn btn--primary btn--sm" type="button" @click="finish">
          Get Started
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.onboarding-backdrop {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.65);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  padding: var(--space-4);
}
.onboarding-modal {
  width: 100%;
  max-width: 520px;
  display: flex;
  flex-direction: column;
  gap: var(--space-4);
  background: var(--color-surface);
  border: var(--border-width) solid var(--color-border);
  box-shadow: var(--shadow-lg);
}
.onboarding-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
}
.onboarding-body {
  display: flex;
  flex-direction: column;
  gap: var(--space-3);
}
.feature-list {
  display: flex;
  flex-direction: column;
  gap: var(--space-2);
  padding-left: var(--space-4);
  color: var(--color-text-secondary);
}
.tip-box {
  background: var(--color-surface-hover);
  padding: var(--space-3);
  border-radius: var(--radius-sm);
  border: var(--border-width) solid var(--color-border);
  font-size: var(--text-body-sm-size);
}
.shortcut-table {
  display: flex;
  flex-direction: column;
  gap: var(--space-2);
}
.shortcut-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--space-2);
  background: var(--color-surface-hover);
  border-radius: var(--radius-sm);
}
kbd {
  background: var(--color-surface);
  padding: 2px 6px;
  border-radius: 4px;
  border: var(--border-width) solid var(--color-border);
  font-size: 11px;
}
.onboarding-footer {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  margin-top: var(--space-2);
}
.muted {
  color: var(--color-text-muted);
  margin: 0;
}
.text-xs {
  font-size: 11px;
}
.flex-1 {
  flex: 1;
}
</style>

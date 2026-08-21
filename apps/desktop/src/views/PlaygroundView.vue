<script setup lang="ts">
import { computed, ref, watch } from "vue";
import ParameterModal from "../components/ParameterModal.vue";
import TerminalEmulator from "../components/TerminalEmulator.vue";
import { usePlaygroundStore } from "../stores/playground";
import { extractParams, copyToClipboard } from "../composables/useTauri";

const playground = usePlaygroundStore();

const paramModalOpen = ref(false);
const detectedParams = ref<string[]>([]);
const copied = ref(false);

const hasParams = computed(() => {
  return (
    detectedParams.value.length > 0 || Object.keys(playground.variables).length > 0
  );
});

watch(
  () => playground.command,
  async (newValue) => {
    try {
      detectedParams.value = await extractParams(newValue);
    } catch {
      detectedParams.value = [];
    }
  },
  { immediate: true },
);

function openParamModal() {
  paramModalOpen.value = true;
}

function onParamsSubmit(values: Record<string, string>) {
  playground.variables = values;
  playground.simulate();
}

async function copyCurrentCommand() {
  await copyToClipboard(playground.command);
  copied.value = true;
  setTimeout(() => {
    copied.value = false;
  }, 2000);
}
</script>

<template>
  <section class="playground">
    <div class="playground__header">
      <div>
        <h1 class="t-headline-md">Terminal Playground</h1>
        <p class="muted">
          Educational simulation only. Destructive commands are blocked for safety.
        </p>
      </div>
      <button
        class="btn btn--primary btn--sm"
        type="button"
        title="Copy raw command to clipboard for real terminal execution"
        @click="copyCurrentCommand"
      >
        {{ copied ? "Copied!" : "📋 Copy for Real Shell" }}
      </button>
    </div>

    <div class="editor-wrapper">
      <textarea
        v-model="playground.command"
        class="input playground__editor mono"
        rows="5"
        spellcheck="false"
        placeholder="Enter command or select from Browse..."
      />
    </div>

    <div class="playground__actions">
      <button class="btn btn--primary" type="button" @click="playground.simulate()">
        ▶ Try in Sandbox
      </button>
      <button
        v-if="hasParams"
        class="btn btn--ghost"
        type="button"
        @click="openParamModal"
      >
        Parameters
      </button>
      <button
        class="btn btn--ghost"
        type="button"
        @click="playground.clearTranscript()"
      >
        Clear Output
      </button>
    </div>

    <!-- Enhanced Blocked / Warning Banner -->
    <div v-if="playground.lastResult?.blocked" class="danger-banner card">
      <span class="danger-icon">⛔</span>
      <div class="danger-content">
        <strong>Execution Blocked (Safety Guardrail)</strong>
        <p>{{ playground.lastResult.explanation }}</p>
        <p class="text-xs muted">
          What Command simulator protects you from accidental data loss. Use the copy
          button above if you explicitly intend to run this in your secure shell.
        </p>
      </div>
    </div>

    <TerminalEmulator :lines="playground.transcript" />

    <ParameterModal
      v-if="playground.currentCommand"
      v-model:model-value="paramModalOpen"
      :command="playground.currentCommand"
      @submit="onParamsSubmit"
    />
  </section>
</template>

<style scoped>
.playground {
  display: flex;
  flex-direction: column;
  gap: var(--space-4);
}
.playground__header {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: var(--space-3);
  flex-wrap: wrap;
}
.playground__editor {
  width: 100%;
  min-height: 100px;
  font-family: var(--font-mono);
  resize: vertical;
}
.playground__actions {
  display: flex;
  gap: var(--space-2);
  flex-wrap: wrap;
}
.muted {
  color: var(--color-text-secondary);
  margin: 0;
}
.text-xs {
  font-size: 11px;
}
.danger-banner {
  display: flex;
  gap: var(--space-3);
  background: rgba(198, 40, 40, 0.08);
  border-color: rgba(198, 40, 40, 0.3);
  padding: var(--space-3);
}
.danger-icon {
  font-size: 24px;
}
.danger-content {
  display: flex;
  flex-direction: column;
  gap: 4px;
}
.danger-content strong {
  color: var(--color-danger, #c62828);
}
.danger-content p {
  margin: 0;
  color: var(--color-text);
}
</style>

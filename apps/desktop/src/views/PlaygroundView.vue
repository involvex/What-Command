<script setup lang="ts">
import { computed, ref, watch } from "vue";
import ParameterModal from "../components/ParameterModal.vue";
import TerminalEmulator from "../components/TerminalEmulator.vue";
import { usePlaygroundStore } from "../stores/playground";
import { extractParams } from "../composables/useTauri";

const playground = usePlaygroundStore();

const paramModalOpen = ref(false);
const detectedParams = ref<string[]>([]);

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
</script>

<template>
  <section class="playground">
    <h1 class="t-headline-md">Playground</h1>
    <p class="muted">Try commands in a simulated terminal — no real shell execution.</p>
    <textarea
      v-model="playground.command"
      class="input playground__editor mono"
      rows="6"
      spellcheck="false"
    />
    <div class="playground__actions">
      <button class="btn btn--primary" type="button" @click="playground.simulate()">
        Try
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
        Clear
      </button>
    </div>
    <TerminalEmulator :lines="playground.transcript" />
    <p v-if="playground.lastResult?.blocked" class="warn">
      {{ playground.lastResult.explanation }}
    </p>

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
.playground__editor {
  width: 100%;
  min-height: 120px;
  font-family: var(--font-mono);
  resize: vertical;
}
.playground__actions {
  display: flex;
  gap: var(--space-2);
}
.muted {
  color: var(--color-text-secondary);
  margin: 0;
}
.warn {
  color: var(--color-warning);
  margin: 0;
}
</style>

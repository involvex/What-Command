<script setup lang="ts">
import { computed, onMounted, ref, watch } from "vue";
import type { Command, Param } from "../types";
import { extractParams, updateCommandParams } from "../composables/useTauri";

const props = withDefaults(
  defineProps<{
    command: Command;
    modelValue: boolean;
  }>(),
  { modelValue: false },
);

const emit = defineEmits<{
  (e: "update:modelValue", v: boolean): void;
  (e: "submit", values: Record<string, string>): void;
}>();

const loading = ref(false);
const extractedKeys = ref<string[]>([]);
const values = ref<Record<string, string>>({});

const show = computed(() => props.modelValue);

const resolvedParams = computed(() => {
  if (props.command.params && props.command.params.length > 0) {
    return props.command.params;
  }
  return extractedKeys.value.map<Param>((key) => ({
    key,
    label: key,
    type: "string",
  }));
});

const paramEntries = computed(() =>
  resolvedParams.value.map((p) => ({
    param: p,
    value: values.value[p.key] ?? p.default ?? "",
  })),
);

const canSubmit = computed(() => {
  return resolvedParams.value.every(
    (p) => values.value[p.key] !== undefined && values.value[p.key] !== "",
  );
});

watch(show, (v) => {
  if (v) {
    void loadParams();
  }
});

async function loadParams() {
  if (props.command.params && props.command.params.length > 0) {
    extractedKeys.value = props.command.params.map((p) => p.key);
  } else {
    loading.value = true;
    try {
      extractedKeys.value = await extractParams(props.command.command);
    } catch {
      extractedKeys.value = [];
    } finally {
      loading.value = false;
    }
  }
}

async function onSubmit() {
  if (!canSubmit.value) return;
  const result: Record<string, string> = {};
  for (const p of resolvedParams.value) {
    result[p.key] = values.value[p.key] ?? "";
  }
  emit("submit", result);
  await saveParams();
  emit("update:modelValue", false);
}

async function saveParams() {
  if (extractedKeys.value.length === 0) return;
  const params: Param[] = extractedKeys.value.map((key) => ({
    key,
    label: key,
    type: "string",
  }));
  try {
    await updateCommandParams(props.command.id, params);
  } catch {
    /* non-critical */
  }
}

function onKeydown(e: KeyboardEvent) {
  if (e.key === "Escape") {
    emit("update:modelValue", false);
  }
}

onMounted(() => {
  if (show.value) {
    void loadParams();
  }
});
</script>

<template>
  <teleport to="body">
    <transition name="fade">
      <div
        v-if="modelValue"
        class="param-modal__backdrop"
        tabindex="-1"
        @click="emit('update:modelValue', false)"
        @keydown="onKeydown"
      >
        <transition name="scale">
          <div v-show="modelValue" class="param-modal" @click.stop @keydown="onKeydown">
            <header class="param-modal__header">
              <h2>Fill Parameters</h2>
              <button
                class="btn btn--ghost btn--icon btn--sm"
                type="button"
                aria-label="Close"
                @click="emit('update:modelValue', false)"
              >
                ✕
              </button>
            </header>

            <div class="param-modal__body">
              <pre class="param-modal__command mono">{{ command.command }}</pre>

              <div v-if="loading" class="param-modal__loading">
                Extracting parameters...
              </div>

              <template v-else>
                <div
                  v-for="entry in paramEntries"
                  :key="entry.param.key"
                  class="param-modal__field"
                >
                  <label class="param-modal__label">
                    {{ entry.param.label }}
                  </label>
                  <input
                    v-if="entry.param.type !== 'choice'"
                    v-model="values[entry.param.key]"
                    class="input input--sm"
                    :type="entry.param.type === 'number' ? 'number' : 'text'"
                    :placeholder="entry.param.default ?? entry.param.key"
                  />
                  <select
                    v-else-if="entry.param.choices && entry.param.choices.length"
                    v-model="values[entry.param.key]"
                    class="input input--sm"
                  >
                    <option
                      v-for="choice in entry.param.choices"
                      :key="choice"
                      :value="choice"
                    >
                      {{ choice }}
                    </option>
                  </select>
                  <span v-else class="param-modal__no-choices">
                    No choices available
                  </span>
                </div>

                <div
                  v-if="resolvedParams.length === 0 && extractedKeys.length === 0"
                  class="param-modal__empty"
                >
                  No parameters detected in this command.
                </div>
              </template>
            </div>

            <footer class="param-modal__footer">
              <button
                class="btn btn--ghost btn--sm"
                type="button"
                @click="emit('update:modelValue', false)"
              >
                Cancel
              </button>
              <button
                class="btn btn--primary btn--sm"
                type="button"
                :disabled="!canSubmit"
                @click="onSubmit"
              >
                Apply &amp; Run
              </button>
            </footer>
          </div>
        </transition>
      </div>
    </transition>
  </teleport>
</template>

<style scoped>
.param-modal__backdrop {
  position: fixed;
  inset: 0;
  background: var(--color-overlay);
  display: flex;
  align-items: center;
  justify-content: center;
  padding: var(--space-4);
  z-index: 1000;
}

.param-modal {
  background: var(--color-elevated);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  max-width: 520px;
  width: 100%;
  display: flex;
  flex-direction: column;
  max-height: 80vh;
}

.param-modal__header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--space-4);
  border-bottom: 1px solid var(--color-border);
}

.param-modal__header h2 {
  font-size: var(--text-headline-md-size);
  font-weight: var(--text-headline-md-weight);
  margin: 0;
}

.param-modal__body {
  padding: var(--space-4);
  overflow-y: auto;
  flex: 1;
}

.param-modal__command {
  margin: 0 0 var(--space-4);
  padding: var(--space-3);
  background: var(--color-background);
  border-radius: var(--radius-sm);
  font-size: var(--text-mono-sm-size);
  word-break: break-word;
}

.param-modal__field {
  display: flex;
  flex-direction: column;
  gap: var(--space-2);
  margin-bottom: var(--space-3);
}

.param-modal__field:last-child {
  margin-bottom: 0;
}

.param-modal__label {
  font-size: var(--text-label-sm-size);
  font-weight: var(--text-label-sm-weight);
  letter-spacing: var(--text-label-sm-tracking);
  color: var(--color-text-secondary);
}

.param-modal__loading,
.param-modal__empty,
.param-modal__no-choices {
  color: var(--color-text-secondary);
  font-size: var(--text-body-sm-size);
  text-align: center;
  padding: var(--space-4);
}

.param-modal__footer {
  display: flex;
  justify-content: flex-end;
  gap: var(--space-2);
  padding: var(--space-4);
  border-top: 1px solid var(--color-border);
}

.input {
  background: var(--color-background);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-sm);
  padding: var(--space-2);
  font-family: var(--font-mono);
  font-size: var(--text-body-md-size);
  color: var(--color-text-primary);
  outline: none;
  transition: border-color 0.2s ease;
}

.input:focus {
  border-color: var(--color-primary);
}

.input--sm {
  font-size: var(--text-body-sm-size);
  padding: var(--space-1) var(--space-2);
}

.btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  border: 1px solid transparent;
  border-radius: var(--radius-sm);
  padding: var(--space-2) var(--space-4);
  font-size: var(--text-body-sm-size);
  font-weight: var(--text-label-sm-weight);
  cursor: pointer;
  transition:
    background-color 0.2s ease,
    border-color 0.2s ease;
}

.btn--ghost {
  background: transparent;
  border-color: var(--color-border);
  color: var(--color-text-secondary);
}

.btn--ghost:hover {
  background: var(--color-background);
}

.btn--primary {
  background: var(--color-primary);
  color: var(--color-text-inverse);
}

.btn--primary:hover:not(:disabled) {
  background: var(--color-primary-hover);
}

.btn--primary:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn--icon {
  padding: var(--space-1);
}

.btn--sm {
  font-size: var(--text-label-sm-size);
  padding: var(--space-1) var(--space-3);
}

.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.2s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}

.scale-enter-active,
.scale-leave-active {
  transition:
    transform 0.2s ease,
    opacity 0.2s ease;
}

.scale-enter-from,
.scale-leave-to {
  opacity: 0;
  transform: scale(0.95);
}
</style>

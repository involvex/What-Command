<script setup lang="ts">
import { computed, reactive, ref, watch } from "vue";
import type { AppSettings } from "../types";
import { useSettingsStore } from "../stores/settings";
import { pickGgufModelFile, modelIdFromGgufPath } from "../composables/useTauri";

const settingsStore = useSettingsStore();

const providers = [
  { value: "opencode_zen", label: "OpenCode Zen" },
  { value: "kilo_gateway", label: "Kilo Gateway" },
  { value: "local_llm", label: "Local LLM (GGUF / On-Device)" },
  { value: "openai_compat", label: "OpenAI-compatible" },
];

const form = reactive<AppSettings>({
  ai_provider: "opencode_zen",
  ai_model: "mimo-v2.5-free",
  fallback_provider: "local_llm",
  fallback_model: "gemma-2b-it-q4",
  opencode_api_key: "",
  kilo_api_key: "",
  local_model_id: "gemma-2b-it-q4",
  local_model_path: "",
  local_max_tokens: 256,
  openai_compat_base_url: "http://127.0.0.1:8080/v1",
  openai_compat_api_key: "",
  theme: "dark",
  contrast: "normal",
});

const saving = ref(false);
const pickingModel = ref(false);
const saved = ref(false);
const error = ref<string | null>(null);

// Quantization presets for local GGUF models
const quantizationPresets = [
  { id: "gemma-2b-it-q4", name: "Gemma 2B IT (Q4_K_M - ~1.4 GB RAM)", tokens: 256 },
  { id: "gemma-2b-it-q8", name: "Gemma 2B IT (Q8_0 - ~2.6 GB RAM)", tokens: 256 },
  { id: "llama-3-8b-q4", name: "Llama 3 8B (Q4_K_M - ~4.8 GB RAM)", tokens: 512 },
  { id: "phi-3-mini-q4", name: "Phi-3 Mini (Q4_K_M - ~2.2 GB RAM)", tokens: 256 },
];

watch(
  () => settingsStore.settings,
  (s) => {
    if (!s) return;
    Object.assign(form, {
      ...s,
      opencode_api_key: s.opencode_api_key ?? "",
      kilo_api_key: s.kilo_api_key ?? "",
      local_model_path: s.local_model_path ?? "",
      openai_compat_base_url: s.openai_compat_base_url ?? "",
      openai_compat_api_key: s.openai_compat_api_key ?? "",
      local_max_tokens: s.local_max_tokens ?? 256,
      theme: s.theme ?? "dark",
      contrast: s.contrast ?? "normal",
    });
    applyTheme(s.theme ?? "dark", s.contrast ?? "normal");
  },
  { immediate: true },
);

function applyTheme(theme: string, contrast: string) {
  document.documentElement.setAttribute("data-theme", theme);
  document.documentElement.setAttribute("data-contrast", contrast);
}

watch(
  () => [form.theme, form.contrast],
  ([t, c]) => {
    applyTheme(t || "dark", c || "normal");
  },
);

const showOpenCode = computed(
  () =>
    form.ai_provider === "opencode_zen" || form.fallback_provider === "opencode_zen",
);
const showKilo = computed(
  () =>
    form.ai_provider === "kilo_gateway" || form.fallback_provider === "kilo_gateway",
);
const showLocal = computed(
  () => form.ai_provider === "local_llm" || form.fallback_provider === "local_llm",
);
const localModelId = computed(() =>
  form.local_model_path
    ? modelIdFromGgufPath(form.local_model_path)
    : form.local_model_id || "not set",
);

function maskKey(value: string | null | undefined): boolean {
  return Boolean(value && value.length > 4);
}

function selectPreset(preset: (typeof quantizationPresets)[0]) {
  form.local_model_id = preset.id;
  form.local_max_tokens = preset.tokens;
  if (form.ai_provider === "local_llm") {
    form.ai_model = preset.id;
  }
}

async function onSave() {
  saving.value = true;
  saved.value = false;
  error.value = null;
  try {
    const payload: AppSettings = {
      ...form,
      opencode_api_key: form.opencode_api_key || null,
      kilo_api_key: form.kilo_api_key || null,
      local_model_path: form.local_model_path || null,
      openai_compat_base_url: form.openai_compat_base_url || null,
      openai_compat_api_key: form.openai_compat_api_key || null,
      fallback_provider: form.fallback_provider || null,
      fallback_model: form.fallback_model || null,
    };
    await settingsStore.save(payload);
    saved.value = true;
  } catch (e) {
    error.value = e instanceof Error ? e.message : String(e);
  } finally {
    saving.value = false;
  }
}

async function onPickModel() {
  pickingModel.value = true;
  error.value = null;
  try {
    const path = await pickGgufModelFile();
    if (path) {
      form.local_model_path = path;
      const id = modelIdFromGgufPath(path);
      form.local_model_id = id;
      if (form.ai_provider === "local_llm") {
        form.ai_model = id;
      }
      if (form.fallback_provider === "local_llm") {
        form.fallback_model = id;
      }
    }
  } catch (e) {
    error.value = e instanceof Error ? e.message : String(e);
  } finally {
    pickingModel.value = false;
  }
}
</script>

<template>
  <form class="settings-form card" @submit.prevent="onSave">
    <h2 class="t-title-md">AI Settings & On-Device GGUF</h2>
    <p class="muted">
      Configure cloud gateways or select on-device GGUF weights for fully offline AI
      inference.
    </p>

    <label class="field">
      <span class="field__label">Primary provider</span>
      <select v-model="form.ai_provider" class="field__input">
        <option v-for="p in providers" :key="p.value" :value="p.value">
          {{ p.label }}
        </option>
      </select>
    </label>

    <label v-if="form.ai_provider !== 'local_llm'" class="field">
      <span class="field__label">Primary model</span>
      <input
        v-model="form.ai_model"
        class="field__input"
        type="text"
        autocomplete="off"
      />
    </label>

    <!-- On-Device GGUF Selector & Quantization Toggles -->
    <div v-if="showLocal" class="local-llm-section">
      <div class="section-divider"></div>
      <h3 class="t-title-sm">On-Device Local GGUF Configuration</h3>
      <p class="muted text-sm">
        Android feature flag <code>--features local-llm</code> enabled. Select model
        weights or preset quantization.
      </p>

      <div class="preset-grid">
        <button
          v-for="p in quantizationPresets"
          :key="p.id"
          type="button"
          class="btn btn--sm"
          :class="form.local_model_id === p.id ? 'btn--primary' : 'btn--ghost'"
          @click="selectPreset(p)"
        >
          {{ p.id.toUpperCase() }}
        </button>
      </div>

      <div class="field">
        <span class="field__label">Selected Model / VRAM Profile</span>
        <div class="model-status-row">
          <span class="mono model-path-badge">{{ localModelId }}</span>
          <button
            type="button"
            class="btn btn--ghost btn--sm"
            :disabled="pickingModel"
            @click="onPickModel"
          >
            {{ pickingModel ? "Loading..." : "Browse GGUF File…" }}
          </button>
        </div>
        <span v-if="form.local_model_path" class="hint mono break-all">
          Path: {{ form.local_model_path }}
        </span>
      </div>

      <label class="field">
        <span class="field__label">Max Context Tokens</span>
        <input
          v-model.number="form.local_max_tokens"
          class="field__input"
          type="number"
          min="128"
          max="2048"
          step="64"
        />
        <span class="hint"
          >Lower token count preserves memory on Android mobile devices.</span
        >
      </label>
      <div class="section-divider"></div>
    </div>

    <label class="field">
      <span class="field__label">Fallback provider</span>
      <select v-model="form.fallback_provider" class="field__input">
        <option :value="null">None</option>
        <option v-for="p in providers" :key="p.value" :value="p.value">
          {{ p.label }}
        </option>
      </select>
    </label>

    <template v-if="showOpenCode">
      <label class="field">
        <span class="field__label">OpenCode API key</span>
        <input
          v-model="form.opencode_api_key"
          class="field__input"
          type="password"
          autocomplete="off"
          placeholder="sk-…"
        />
        <span v-if="maskKey(settingsStore.settings?.opencode_api_key)" class="hint">
          A key is saved; leave blank to keep unchanged.
        </span>
      </label>
    </template>

    <template v-if="showKilo">
      <label class="field">
        <span class="field__label">Kilo Gateway API key</span>
        <input
          v-model="form.kilo_api_key"
          class="field__input"
          type="password"
          autocomplete="off"
          placeholder="kilo-…"
        />
      </label>
    </template>

    <div class="section-divider"></div>
    <h3 class="t-title-sm">Appearance & Accessibility</h3>

    <div class="field-row">
      <label class="field flex-1">
        <span class="field__label">Theme</span>
        <select v-model="form.theme" class="field__input">
          <option value="dark">Dark (Halo)</option>
          <option value="light">Light</option>
        </select>
      </label>
      <label class="field flex-1">
        <span class="field__label">Contrast</span>
        <select v-model="form.contrast" class="field__input">
          <option value="normal">Normal</option>
          <option value="high">High Contrast</option>
        </select>
      </label>
    </div>

    <div class="actions">
      <button class="btn btn--primary" type="submit" :disabled="saving">
        {{ saving ? "Saving..." : "Save Settings" }}
      </button>
      <span v-if="saved" class="success">Saved successfully!</span>
      <span v-if="error" class="error">{{ error }}</span>
    </div>
  </form>
</template>

<style scoped>
.settings-form {
  display: flex;
  flex-direction: column;
  gap: var(--space-4);
}
.muted {
  color: var(--color-text-muted);
  margin: 0;
  font-size: var(--text-body-sm-size);
}
.text-sm {
  font-size: 12px;
}
.field {
  display: flex;
  flex-direction: column;
  gap: var(--space-1);
}
.field__label {
  font-size: var(--text-body-sm-size);
  font-weight: 500;
}
.field__input {
  padding: var(--space-2);
  background: var(--color-surface-hover);
  border: var(--border-width) solid var(--color-border);
  border-radius: var(--radius-sm);
  color: var(--color-text);
  font-size: var(--text-body-size);
}
.hint {
  font-size: 11px;
  color: var(--color-text-muted);
}
.break-all {
  word-break: break-all;
}
.local-llm-section {
  display: flex;
  flex-direction: column;
  gap: var(--space-3);
  background: var(--color-surface-hover);
  padding: var(--space-3);
  border-radius: var(--radius-md);
  border: var(--border-width) solid var(--color-border);
}
.section-divider {
  height: 1px;
  background: var(--color-border);
  width: 100%;
}
.preset-grid {
  display: flex;
  flex-wrap: wrap;
  gap: var(--space-2);
}
.model-status-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--space-2);
}
.model-path-badge {
  background: var(--color-surface);
  padding: 4px 8px;
  border-radius: var(--radius-sm);
  font-size: 12px;
  border: var(--border-width) solid var(--color-border);
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
.actions {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  margin-top: var(--space-2);
}
.success {
  color: #2e7d32;
  font-size: var(--text-body-sm-size);
}
.error {
  color: var(--color-danger, #c62828);
  font-size: var(--text-body-sm-size);
}
</style>

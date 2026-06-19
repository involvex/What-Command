<script setup lang="ts">
import { computed, reactive, ref, watch } from "vue";
import type { AppSettings } from "../types";
import { useSettingsStore } from "../stores/settings";

const settingsStore = useSettingsStore();

const providers = [
  { value: "opencode_zen", label: "OpenCode Zen" },
  { value: "kilo_gateway", label: "Kilo Gateway" },
  { value: "local_llm", label: "Local LLM (GGUF)" },
  { value: "openai_compat", label: "OpenAI-compatible" },
];

const form = reactive<AppSettings>({
  ai_provider: "opencode_zen",
  ai_model: "gpt-4o-mini",
  fallback_provider: "local_llm",
  fallback_model: "gemma-2b-it-q4",
  opencode_api_key: "",
  kilo_api_key: "",
  local_model_id: "gemma-2b-it-q4",
  local_model_path: "",
  local_max_tokens: 256,
  openai_compat_base_url: "http://127.0.0.1:8080/v1",
  openai_compat_api_key: "",
});

const saving = ref(false);
const saved = ref(false);
const error = ref<string | null>(null);

watch(
  () => settingsStore.settings,
  (s) => {
    if (!s) {
      return;
    }
    Object.assign(form, {
      ...s,
      opencode_api_key: s.opencode_api_key ?? "",
      kilo_api_key: s.kilo_api_key ?? "",
      local_model_path: s.local_model_path ?? "",
      openai_compat_base_url: s.openai_compat_base_url ?? "",
      openai_compat_api_key: s.openai_compat_api_key ?? "",
      local_max_tokens: s.local_max_tokens ?? 256,
    });
  },
  { immediate: true },
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
const showCompat = computed(
  () =>
    form.ai_provider === "openai_compat" || form.fallback_provider === "openai_compat",
);

function maskKey(value: string | null | undefined): boolean {
  return Boolean(value && value.length > 4);
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
</script>

<template>
  <form class="settings-form card" @submit.prevent="onSave">
    <h2 class="t-title-md">AI Settings</h2>
    <p class="muted">
      Keys are stored in ~/.config/what-command/config.toml. Environment variables
      (OPENCODE_API_KEY, KILO_API_KEY, LOCAL_GGUF_PATH) override empty fields at
      startup.
    </p>

    <label class="field">
      <span class="field__label">Primary provider</span>
      <select v-model="form.ai_provider" class="field__input">
        <option v-for="p in providers" :key="p.value" :value="p.value">
          {{ p.label }}
        </option>
      </select>
    </label>

    <label class="field">
      <span class="field__label">Primary model</span>
      <input
        v-model="form.ai_model"
        class="field__input"
        type="text"
        autocomplete="off"
      />
    </label>

    <label class="field">
      <span class="field__label">Fallback provider</span>
      <select v-model="form.fallback_provider" class="field__input">
        <option :value="null">None</option>
        <option v-for="p in providers" :key="p.value" :value="p.value">
          {{ p.label }}
        </option>
      </select>
    </label>

    <label v-if="form.fallback_provider" class="field">
      <span class="field__label">Fallback model</span>
      <input
        v-model="form.fallback_model"
        class="field__input"
        type="text"
        autocomplete="off"
      />
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
          A key is saved; leave blank to keep it unchanged on next save only if you
          re-enter.
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
        />
      </label>
    </template>

    <template v-if="showCompat">
      <label class="field">
        <span class="field__label">OpenAI-compatible base URL</span>
        <input
          v-model="form.openai_compat_base_url"
          class="field__input"
          type="url"
          autocomplete="off"
        />
      </label>
      <label class="field">
        <span class="field__label">OpenAI-compatible API key</span>
        <input
          v-model="form.openai_compat_api_key"
          class="field__input"
          type="password"
          autocomplete="off"
        />
      </label>
    </template>

    <template v-if="showLocal">
      <label class="field">
        <span class="field__label">Local model ID</span>
        <input
          v-model="form.local_model_id"
          class="field__input"
          type="text"
          autocomplete="off"
        />
      </label>
      <label class="field">
        <span class="field__label">GGUF model path</span>
        <input
          v-model="form.local_model_path"
          class="field__input"
          type="text"
          autocomplete="off"
          placeholder="~/.config/what-command/models/model.gguf"
        />
      </label>
      <label class="field">
        <span class="field__label">Max tokens</span>
        <input
          v-model.number="form.local_max_tokens"
          class="field__input"
          type="number"
          min="32"
          max="2048"
        />
      </label>
      <p class="hint">
        Build with <code>cargo build --features local-llm</code> for on-device llama.cpp
        inference.
      </p>
    </template>

    <div class="actions">
      <button type="submit" class="btn btn--primary" :disabled="saving">
        {{ saving ? "Saving…" : "Save settings" }}
      </button>
      <span v-if="saved" class="success">Saved</span>
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
}
.field {
  display: flex;
  flex-direction: column;
  gap: var(--space-2);
}
.field__label {
  font-size: var(--text-body-sm-size);
  font-weight: 500;
  color: var(--color-text-secondary);
}
.field__input {
  padding: var(--space-3);
  border: var(--border-width) solid var(--color-border);
  border-radius: var(--radius-sm);
  background: var(--color-surface);
  color: var(--color-text);
  font: inherit;
}
.hint {
  font-size: var(--text-body-sm-size);
  color: var(--color-text-muted);
}
.actions {
  display: flex;
  align-items: center;
  gap: var(--space-3);
}
.success {
  color: var(--color-success, #2e7d32);
  font-size: var(--text-body-sm-size);
}
.error {
  color: var(--color-danger, #c62828);
  font-size: var(--text-body-sm-size);
}
code {
  font-family: var(--font-mono, monospace);
  font-size: 0.9em;
}
</style>

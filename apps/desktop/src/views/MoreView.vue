<script setup lang="ts">
import { onMounted, ref } from "vue";
import { useCommandsStore } from "../stores/commands";
import { useSettingsStore } from "../stores/settings";
import SettingsForm from "../components/SettingsForm.vue";
import type { Command } from "../types";
import { invoke } from "@tauri-apps/api/core";

const favorites = useCommandsStore();
const settings = useSettingsStore();

const userCommands = ref<Command[]>([]);
const showModal = ref(false);
const formCommand = ref("");
const formDescription = ref("");
const formCategory = ref("custom");
const formDanger = ref(0);
const error = ref<string | null>(null);

async function loadUserCommands() {
  try {
    userCommands.value = await invoke<Command[]>("list_user_commands");
  } catch {
    userCommands.value = [];
  }
}

async function saveCustomCommand() {
  error.value = null;
  if (!formCommand.value.trim()) {
    error.value = "Command string is required.";
    return;
  }
  try {
    await invoke("save_user_command", {
      command: {
        id: `user-${Date.now()}`,
        command: formCommand.value.trim(),
        description: formDescription.value.trim() || "Custom user command",
        category: formCategory.value.trim() || "custom",
        platform: ["common"],
        danger_level: Number(formDanger.value),
        source: "user",
        updated_at: new Date().toISOString(),
        params: [],
      },
    });
    showModal.value = false;
    formCommand.value = "";
    formDescription.value = "";
    await loadUserCommands();
  } catch (e) {
    error.value = e instanceof Error ? e.message : String(e);
  }
}

async function deleteCustomCommand(id: string) {
  try {
    await invoke("delete_user_command", { id });
    await loadUserCommands();
  } catch (e) {
    error.value = e instanceof Error ? e.message : String(e);
  }
}

onMounted(() => {
  void settings.load();
  void loadUserCommands();
});
</script>

<template>
  <section class="more">
    <h1 class="t-headline-md">More & Custom Commands</h1>
    <SettingsForm />

    <!-- Custom Commands Management -->
    <div class="card">
      <div class="section-header">
        <h2 class="t-title-md">My Custom Commands</h2>
        <button
          class="btn btn--primary btn--sm"
          type="button"
          @click="showModal = true"
        >
          + Add Command
        </button>
      </div>
      <p v-if="!userCommands.length" class="muted">
        No custom commands added yet. Create your own team aliases or project scripts.
      </p>
      <ul v-else class="custom-cmd-list">
        <li v-for="cmd in userCommands" :key="cmd.id" class="custom-cmd-item">
          <div class="custom-cmd-info">
            <span class="mono fw-bold">{{ cmd.command }}</span>
            <span class="muted text-sm">{{ cmd.description }}</span>
          </div>
          <button
            class="btn btn--ghost btn--sm text-danger"
            type="button"
            @click="deleteCustomCommand(cmd.id)"
          >
            Delete
          </button>
        </li>
      </ul>
      <span v-if="error" class="error">{{ error }}</span>
    </div>

    <!-- Favorites -->
    <div class="card">
      <h2 class="t-title-md">Favorites</h2>
      <p v-if="!favorites.favorites.length" class="muted">
        Star commands from Browse to save them.
      </p>
      <ul v-else class="fav-list">
        <li v-for="f in favorites.favorites" :key="f.id" class="mono">
          {{ f.command }}
        </li>
      </ul>
    </div>

    <!-- Modal for adding custom command -->
    <div v-if="showModal" class="modal-backdrop">
      <div class="modal card">
        <h3 class="t-title-md">Add Custom Command</h3>
        <label class="field">
          <span class="field__label">Command String</span>
          <input
            v-model="formCommand"
            class="input mono"
            placeholder="e.g. docker compose up -d"
          />
        </label>
        <label class="field">
          <span class="field__label">Description</span>
          <input
            v-model="formDescription"
            class="input"
            placeholder="Start containers in background"
          />
        </label>
        <label class="field">
          <span class="field__label">Category</span>
          <input v-model="formCategory" class="input" placeholder="docker" />
        </label>
        <label class="field">
          <span class="field__label">Danger Level (0-3)</span>
          <select v-model.number="formDanger" class="input">
            <option :value="0">0 - Safe</option>
            <option :value="1">1 - Caution</option>
            <option :value="2">2 - Destructive</option>
            <option :value="3">3 - Critical</option>
          </select>
        </label>
        <div class="modal-actions">
          <button class="btn btn--primary" type="button" @click="saveCustomCommand">
            Save
          </button>
          <button class="btn btn--ghost" type="button" @click="showModal = false">
            Cancel
          </button>
        </div>
      </div>
    </div>
  </section>
</template>

<style scoped>
.more {
  display: flex;
  flex-direction: column;
  gap: var(--space-4);
}
.section-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: var(--space-2);
}
.muted {
  color: var(--color-text-muted);
}
.text-sm {
  font-size: 12px;
}
.text-danger {
  color: var(--color-danger, #c62828);
}
.custom-cmd-list {
  display: flex;
  flex-direction: column;
  gap: var(--space-2);
  list-style: none;
  padding: 0;
  margin: 0;
}
.custom-cmd-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--space-2);
  background: var(--color-surface-hover);
  border-radius: var(--radius-sm);
  border: var(--border-width) solid var(--color-border);
}
.custom-cmd-info {
  display: flex;
  flex-direction: column;
  gap: 2px;
}
.fav-list {
  list-style: none;
  padding: 0;
  margin: 0;
  display: flex;
  flex-direction: column;
  gap: var(--space-2);
}
.modal-backdrop {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 100;
  padding: var(--space-4);
}
.modal {
  width: 100%;
  max-width: 480px;
  display: flex;
  flex-direction: column;
  gap: var(--space-3);
  background: var(--color-surface);
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
.modal-actions {
  display: flex;
  gap: var(--space-2);
  justify-content: flex-end;
  margin-top: var(--space-2);
}
.error {
  color: var(--color-danger, #c62828);
  font-size: var(--text-body-sm-size);
}
</style>

<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import { useRoute, useRouter } from "vue-router";
import {
  getCommand,
  relatedCommands,
  copyToClipboard,
  recordUsage,
} from "../composables/useTauri";
import { usePlaygroundStore } from "../stores/playground";
import { useAiStore } from "../stores/ai";
import type { Command } from "../types";

const route = useRoute();
const router = useRouter();
const playground = usePlaygroundStore();
const ai = useAiStore();

const commandId = computed(() => String(route.params.id || ""));
const command = ref<Command | null>(null);
const related = ref<Command[]>([]);
const activeTab = ref<"overview" | "examples" | "related" | "notes">("overview");
const userNotes = ref("");
const savedNotes = ref(false);
const explanation = ref<string | null>(null);

onMounted(async () => {
  if (commandId.value) {
    try {
      command.value = await getCommand(commandId.value);
      related.value = await relatedCommands(commandId.value, 5);
      const storedNotes = localStorage.getItem(`wc_notes_${commandId.value}`);
      if (storedNotes) userNotes.value = storedNotes;
    } catch {
      command.value = null;
    }
  }
});

async function onCopy() {
  if (!command.value) return;
  await copyToClipboard(command.value.command);
  void recordUsage(command.value.id, "copy");
}

function toPlayground() {
  if (!command.value) return;
  playground.insertCommand(command.value.command, command.value);
  router.push("/playground");
}

async function onExplain() {
  if (!command.value) return;
  activeTab.value = "overview";
  explanation.value = await ai.explain(command.value.command);
  void recordUsage(command.value.id, "explain");
}

function saveNotes() {
  if (!commandId.value) return;
  localStorage.setItem(`wc_notes_${commandId.value}`, userNotes.value);
  savedNotes.value = true;
  setTimeout(() => {
    savedNotes.value = false;
  }, 2000);
}
</script>

<template>
  <section class="command-detail">
    <button class="btn btn--ghost btn--sm mb-2" type="button" @click="router.back()">
      ← Back
    </button>

    <div v-if="!command" class="card">
      <p class="muted">Command not found.</p>
    </div>

    <template v-else>
      <div class="card detail-header">
        <div class="header-top">
          <span class="badge">{{ command.category }}</span>
          <span class="badge badge--muted">{{ command.source }}</span>
          <span v-if="command.danger_level >= 2" class="badge badge--warning">destructive</span>
        </div>
        <pre class="mono detail-code">{{ command.command }}</pre>
        <p class="detail-desc">{{ command.description }}</p>

        <div class="detail-actions">
          <button class="btn btn--primary btn--sm" type="button" @click="onCopy">
            📋 Copy Command
          </button>
          <button class="btn btn--ghost btn--sm" type="button" @click="toPlayground">
            💻 Try in Playground
          </button>
          <button class="btn btn--ghost btn--sm" type="button" @click="onExplain">
            🤖 AI Explain
          </button>
          <a
            class="btn btn--ghost btn--sm text-decoration-none"
            :href="`https://github.com/tldr-pages/tldr/blob/main/pages/common/${command.id}.md`"
            target="_blank"
            rel="noopener noreferrer"
          >
            🔗 Upstream TLDR
          </a>
        </div>
      </div>

      <!-- Tabs -->
      <div class="tabs-header">
        <button
          class="btn btn--sm"
          :class="activeTab === 'overview' ? 'btn--primary' : 'btn--ghost'"
          type="button"
          @click="activeTab = 'overview'"
        >
          Overview
        </button>
        <button
          class="btn btn--sm"
          :class="activeTab === 'examples' ? 'btn--primary' : 'btn--ghost'"
          type="button"
          @click="activeTab = 'examples'"
        >
          Examples & Flags
        </button>
        <button
          class="btn btn--sm"
          :class="activeTab === 'related' ? 'btn--primary' : 'btn--ghost'"
          type="button"
          @click="activeTab = 'related'"
        >
          Related Commands
        </button>
        <button
          class="btn btn--sm"
          :class="activeTab === 'notes' ? 'btn--primary' : 'btn--ghost'"
          type="button"
          @click="activeTab = 'notes'"
        >
          User Notes
        </button>
      </div>

      <!-- Tab Contents -->
      <div class="card tab-body">
        <div v-if="activeTab === 'overview'" class="tab-pane">
          <h3 class="t-title-sm">Summary & AI Analysis</h3>
          <p v-if="explanation" class="explanation-box mono">{{ explanation }}</p>
          <p v-else class="muted">
            Click <strong>AI Explain</strong> above to generate a breakdown of this
            command, flags, and common pitfalls.
          </p>
          <div class="meta-grid">
            <div class="meta-item">
              <span class="muted text-xs">Platforms</span>
              <span class="mono">{{ command.platform.join(", ") }}</span>
            </div>
            <div class="meta-item">
              <span class="muted text-xs">Danger Level</span>
              <span class="mono">Level {{ command.danger_level }}</span>
            </div>
            <div class="meta-item">
              <span class="muted text-xs">Last Updated</span>
              <span class="mono">{{ command.updated_at }}</span>
            </div>
          </div>
        </div>

        <div v-if="activeTab === 'examples'" class="tab-pane">
          <h3 class="t-title-sm">Usage & Variations</h3>
          <p class="muted">
            Execute variants of <code>{{ command.command }}</code> with different flags
            in the terminal simulator.
          </p>
          <pre class="mono sample-box">
# Standard usage
{{ command.command }}

# Verbose / Detailed
{{ command.command }} --help</pre>
        </div>

        <div v-if="activeTab === 'related'" class="tab-pane">
          <h3 class="t-title-sm">Related Commands</h3>
          <ul v-if="related.length" class="related-list">
            <li v-for="rel in related" :key="rel.id" class="related-item">
              <router-link :to="`/command/${rel.id}`" class="mono rel-link">
                {{ rel.command }}
              </router-link>
              <span class="muted text-xs">{{ rel.description }}</span>
            </li>
          </ul>
          <p v-else class="muted">No related commands found.</p>
        </div>

        <div v-if="activeTab === 'notes'" class="tab-pane">
          <h3 class="t-title-sm">Personal Notes</h3>
          <p class="muted text-xs">
            Add custom reminders, project flags, or team notes for this command. Saved
            locally.
          </p>
          <textarea
            v-model="userNotes"
            class="input notes-editor mono"
            rows="6"
            placeholder="Type your notes here..."
          />
          <div class="notes-footer">
            <button class="btn btn--primary btn--sm" type="button" @click="saveNotes">
              Save Notes
            </button>
            <span v-if="savedNotes" class="success text-xs">Saved!</span>
          </div>
        </div>
      </div>
    </template>
  </section>
</template>

<style scoped>
.command-detail {
  display: flex;
  flex-direction: column;
  gap: var(--space-4);
  max-width: 800px;
  margin: 0 auto;
  width: 100%;
}
.detail-header {
  display: flex;
  flex-direction: column;
  gap: var(--space-3);
  background: var(--color-surface);
}
.header-top {
  display: flex;
  gap: var(--space-2);
}
.detail-code {
  margin: 0;
  padding: var(--space-4);
  background: var(--color-background);
  border-radius: var(--radius-sm);
  font-size: var(--text-mono-sm-size);
  overflow-x: auto;
}
.detail-desc {
  margin: 0;
  color: var(--color-text-secondary);
}
.detail-actions {
  display: flex;
  flex-wrap: wrap;
  gap: var(--space-2);
  margin-top: var(--space-2);
}
.tabs-header {
  display: flex;
  gap: var(--space-2);
  flex-wrap: wrap;
}
.tab-body {
  background: var(--color-surface);
  min-height: 200px;
}
.tab-pane {
  display: flex;
  flex-direction: column;
  gap: var(--space-3);
}
.explanation-box {
  background: var(--color-surface-hover);
  padding: var(--space-3);
  border-radius: var(--radius-sm);
  white-space: pre-wrap;
  border: var(--border-width) solid var(--color-border);
}
.meta-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
  gap: var(--space-3);
  margin-top: var(--space-2);
}
.meta-item {
  display: flex;
  flex-direction: column;
  gap: 2px;
  background: var(--color-surface-hover);
  padding: var(--space-2);
  border-radius: var(--radius-sm);
  border: var(--border-width) solid var(--color-border);
}
.sample-box {
  background: var(--color-background);
  padding: var(--space-3);
  border-radius: var(--radius-sm);
  margin: 0;
}
.related-list {
  list-style: none;
  padding: 0;
  margin: 0;
  display: flex;
  flex-direction: column;
  gap: var(--space-2);
}
.related-item {
  display: flex;
  flex-direction: column;
  gap: 2px;
  padding: var(--space-2);
  background: var(--color-surface-hover);
  border-radius: var(--radius-sm);
  border: var(--border-width) solid var(--color-border);
}
.rel-link {
  color: var(--color-primary);
  text-decoration: none;
  font-weight: 500;
}
.rel-link:hover {
  text-decoration: underline;
}
.notes-editor {
  width: 100%;
  resize: vertical;
}
.notes-footer {
  display: flex;
  align-items: center;
  gap: var(--space-2);
}
.success {
  color: var(--color-success);
}
.muted {
  color: var(--color-text-muted);
  margin: 0;
}
.text-xs {
  font-size: 11px;
}
.mb-2 {
  margin-bottom: var(--space-2);
}
</style>

<script setup lang="ts">
import { computed, nextTick, ref, watch } from "vue";
import { useCommandsStore } from "../stores/commands";
import type { Command } from "../types";
import {
  copyToClipboard,
  getRecentCommands,
  getTopCommands,
  recordUsage,
  searchCommands,
} from "../composables/useTauri";

const props = withDefaults(
  defineProps<{
    visible?: boolean;
  }>(),
  { visible: false },
);

const emit = defineEmits<{
  (e: "update:visible", v: boolean): void;
  (e: "select", cmd: Command): void;
}>();

const searchQuery = ref("");
const results = ref<Command[]>([]);
const recentCommands = ref<Command[]>([]);
const topCommands = ref<Command[]>([]);
const loading = ref(false);
const activeIndex = ref(0);
const searchInput = ref<HTMLInputElement>();

const commandsStore = useCommandsStore();

const showRecents = computed(() => searchQuery.value.length === 0);
const displayResults = computed(() => {
  if (searchQuery.value.length === 0) {
    return [];
  }
  return results.value;
});

const allItems = computed(() => {
  if (showRecents.value) {
    return [
      { type: "section", label: "Top Commands" as const, items: topCommands.value },
      { type: "section", label: "Recent" as const, items: recentCommands.value },
      { type: "section", label: "Favorites" as const, items: commandsStore.favorites },
    ];
  }
  return [{ type: "section", label: "Results" as const, items: displayResults.value }];
});

const flatItems = computed(() => allItems.value.flatMap((s) => s.items));

async function doSearch(q: string) {
  if (!q) {
    results.value = [];
    return;
  }
  loading.value = true;
  try {
    results.value = await searchCommands(q, 30);
  } catch {
    results.value = [];
  } finally {
    loading.value = false;
  }
}

watch(
  () => props.visible,
  (v) => {
    if (v) {
      loadSideData();
      searchQuery.value = "";
      results.value = [];
      activeIndex.value = 0;
      nextTick(() => searchInput.value?.focus());
    }
  },
  { immediate: true },
);

async function loadSideData() {
  try {
    recentCommands.value = await getRecentCommands(8);
  } catch {
    recentCommands.value = [];
  }
  try {
    topCommands.value = await getTopCommands(8, 7);
  } catch {
    topCommands.value = [];
  }
}

watch(searchQuery, (q) => {
  activeIndex.value = 0;
  doSearch(q);
});

function handleKeydown(e: KeyboardEvent) {
  if (!props.visible) return;
  const flat = flatItems.value;
  if (e.key === "ArrowDown") {
    e.preventDefault();
    activeIndex.value = (activeIndex.value + 1) % Math.max(flat.length, 1);
  } else if (e.key === "ArrowUp") {
    e.preventDefault();
    activeIndex.value =
      (activeIndex.value - 1 + flat.length) % Math.max(flat.length, 1);
  } else if (e.key === "Enter") {
    e.preventDefault();
    const item = flat[activeIndex.value];
    if (item) {
      handleSelect(item);
    }
  } else if (e.key === "Escape") {
    e.preventDefault();
    emit("update:visible", false);
  }
}

function handleSelect(cmd: Command) {
  emit("select", cmd);
  if (searchQuery.value) {
    searchQuery.value = "";
    results.value = [];
  }
}

function handleCopy(cmd: Command) {
  copyToClipboard(cmd.command);
  void recordUsage(cmd.id, "copy");
}

function isFavorite(cmd: Command) {
  return commandsStore.isFavorite(cmd.id);
}

function toggleFavorite(cmd: Command) {
  commandsStore.toggleFavorite(cmd);
  void (commandsStore.isFavorite(cmd.id)
    ? recordUsage(cmd.id, "favorite")
    : recordUsage(cmd.id, "unfavorite"));
}
</script>

<template>
  <teleport to="body">
    <div
      v-if="visible"
      class="cmd-palette-overlay"
      tabindex="-1"
      @click.self="emit('update:visible', false)"
      @keydown="handleKeydown"
    >
      <div
        class="cmd-palette card"
        role="dialog"
        aria-modal="true"
        aria-label="Command palette"
      >
        <div class="cmd-palette__search">
          <span class="mono">⌘</span>
          <input
            ref="searchInput"
            v-model="searchQuery"
            type="text"
            placeholder="Search commands, frameworks, favorites..."
            autocomplete="off"
            :disabled="loading"
          />
          <span v-if="loading" class="cmd-palette__loading">⏳</span>
          <span v-else class="cmd-palette__hint">Esc to close</span>
        </div>

        <div class="cmd-palette__list">
          <div
            v-for="section of allItems"
            :key="section.label"
            class="cmd-palette__section"
          >
            <div v-if="section.items.length" class="cmd-palette__section-label">
              {{ section.label }}
            </div>
            <div
              v-for="cmd of section.items"
              :key="cmd.id"
              class="cmd-palette__row"
              :class="{
                'cmd-palette__row--active': flatItems.indexOf(cmd) === activeIndex,
                'cmd-palette__row--danger': cmd.danger_level >= 2,
              }"
              @click="handleSelect(cmd)"
              @mouseenter="activeIndex = flatItems.indexOf(cmd)"
            >
              <div class="cmd-palette__cmd mono">{{ cmd.command }}</div>
              <div class="cmd-palette__desc muted">{{ cmd.description }}</div>
              <div class="cmd-palette__meta">
                <span class="badge badge--muted">{{ cmd.category }}</span>
                <span v-if="cmd.danger_level >= 2" class="badge badge--warning"
                  >destructive</span
                >
                <button
                  class="btn btn--ghost btn--sm"
                  type="button"
                  :aria-label="isFavorite(cmd) ? 'Unfavorite' : 'Favorite'"
                  @click.stop="toggleFavorite(cmd)"
                  @mouseenter.stop
                >
                  {{ isFavorite(cmd) ? "★" : "☆" }}
                </button>
                <button
                  class="btn btn--ghost btn--sm"
                  type="button"
                  :aria-label="'Copy ' + cmd.command"
                  @click.stop="handleCopy(cmd)"
                >
                  Copy
                </button>
              </div>
            </div>
          </div>

          <div
            v-if="!loading && flatItems.length === 0 && searchQuery"
            class="cmd-palette__empty"
          >
            No commands found. Try another search term.
          </div>
        </div>
      </div>
    </div>
  </teleport>
</template>

<style scoped>
.cmd-palette-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  background: rgba(0, 0, 0, 0.5);
  backdrop-filter: blur(4px);
  display: flex;
  align-items: flex-start;
  justify-content: center;
  padding-top: 8vh;
  z-index: 1000;
}

.cmd-palette {
  width: 100%;
  max-width: 640px;
  max-height: 72vh;
  display: flex;
  flex-direction: column;
  gap: var(--space-2);
  padding: var(--space-4);
}

.cmd-palette__search {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  background: var(--color-surface);
  border: var(--border-width) solid var(--color-border);
  border-radius: var(--radius-md);
  padding: var(--space-2) var(--space-3);
}

.cmd-palette__search input {
  flex: 1;
  border: none;
  outline: none;
  background: transparent;
  color: var(--color-text);
  font-family: var(--font-mono);
  font-size: var(--text-body-md-size);
}

.cmd-palette__search input::placeholder {
  color: var(--color-text-muted);
}

.cmd-palette__hint {
  font-size: var(--text-body-sm-size);
  color: var(--color-text-muted);
}

.cmd-palette__list {
  overflow-y: auto;
  max-height: 56vh;
}

.cmd-palette__section {
  margin-bottom: var(--space-3);
}

.cmd-palette__section-label {
  font-size: var(--text-label-sm-size);
  text-transform: uppercase;
  letter-spacing: var(--text-label-sm-tracking);
  color: var(--color-text-secondary);
  margin-bottom: var(--space-2);
}

.cmd-palette__row {
  padding: var(--space-2) var(--space-3);
  border-radius: var(--radius-sm);
  cursor: pointer;
  display: flex;
  flex-direction: column;
  gap: var(--space-1);
  transition: background 0.1s ease;
}

.cmd-palette__row:hover,
.cmd-palette__row--active {
  background: var(--color-surface-hover, rgba(255, 255, 255, 0.05));
}

.cmd-palette__row--danger {
  border-left: 3px solid var(--color-warning);
}

.cmd-palette__cmd {
  font-size: var(--text-mono-sm-size);
  color: var(--color-text);
}

.cmd-palette__meta {
  display: flex;
  align-items: center;
  gap: var(--space-1);
  flex-wrap: wrap;
}

.cmd-palette__loading,
.cmd-palette__empty {
  padding: var(--space-6) var(--space-3);
  text-align: center;
  color: var(--color-text-muted);
  font-size: var(--text-body-sm-size);
}
</style>

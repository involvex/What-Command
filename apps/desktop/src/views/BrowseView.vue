<script setup lang="ts">
import { computed, onMounted, ref, watch } from "vue";
import { useCommandsStore } from "../stores/commands";
import { usePlaygroundStore } from "../stores/playground";
import CommandCard from "../components/CommandCard.vue";
import SearchBar from "../components/SearchBar.vue";
import {
  copyToClipboard,
  getRecentCommands,
  recordUsage,
} from "../composables/useTauri";
import { useAiStore } from "../stores/ai";
import type { Command } from "../types";

const commands = useCommandsStore();
const playground = usePlaygroundStore();
const ai = useAiStore();

const recentCommands = ref<Command[]>([]);
const activeTab = ref("search");

const displayItems = computed(() => {
  if (commands.query.length > 0) {
    return commands.results.length ? commands.results : recentCommands.value;
  }
  if (activeTab.value === "favorites") {
    return commands.favorites;
  }
  return recentCommands.value;
});

onMounted(() => {
  if (!commands.query) {
    loadRecent();
    activeTab.value = "recent";
  }
});

watch(
  () => commands.query,
  (q) => {
    if (q.length > 0) {
      activeTab.value = "search";
      commands.search();
    } else {
      activeTab.value = "recent";
    }
  },
);

async function loadRecent() {
  recentCommands.value = await getRecentCommands(10);
}

async function onCopy(cmd: string, id: string) {
  await copyToClipboard(cmd);
  void recordUsage(id, "copy");
}

function toPlayground(cmd: Command) {
  playground.insertCommand(cmd.command, cmd);
  void recordUsage(cmd.id, "playground");
}

async function onExplain(cmd: string, id: string) {
  await ai.explain(cmd);
  void recordUsage(id, "explain");
}

function onFavorite(cmd: Command) {
  commands.toggleFavorite(cmd);
  void recordUsage(cmd.id, commands.isFavorite(cmd.id) ? "favorite" : "unfavorite");
}
</script>

<template>
  <section class="browse">
    <h1 class="t-headline-md">Browse</h1>
    <SearchBar v-model="commands.query" @search="commands.search()" />
    <div v-if="commands.query.length === 0" class="browse__tabs">
      <button
        class="btn btn--ghost btn--sm"
        :class="{ 'btn--active': activeTab === 'recent' }"
        @click="
          activeTab = 'recent';
          loadRecent();
        "
      >
        Recent
      </button>
      <button
        class="btn btn--ghost btn--sm"
        :class="{ 'btn--active': activeTab === 'favorites' }"
        @click="activeTab = 'favorites'"
      >
        Favorites
      </button>
    </div>
    <p v-if="commands.loading && activeTab === 'search'" class="muted">Loading…</p>

    <p
      v-if="
        commands.query.length > 0 && !commands.loading && commands.results.length === 0
      "
      class="muted"
    >
      No results for "{{ commands.query }}". Showing recent commands instead.
    </p>

    <div v-if="displayItems.length === 0 && !commands.loading" class="muted">
      No commands found.
    </div>

    <div class="browse__list">
      <CommandCard
        v-for="cmd in displayItems"
        :key="cmd.id"
        :command="cmd"
        @copy="onCopy(cmd.command, cmd.id)"
        @explain="onExplain(cmd.command, cmd.id)"
        @playground="toPlayground(cmd)"
        @favorite="onFavorite(cmd)"
      />
    </div>
  </section>
</template>

<style scoped>
.browse {
  display: flex;
  flex-direction: column;
  gap: var(--space-4);
}
.browse__list {
  display: flex;
  flex-direction: column;
  gap: var(--space-3);
}
.muted {
  color: var(--color-text-muted);
}
</style>

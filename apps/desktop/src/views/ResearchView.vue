<script setup lang="ts">
import { computed, onMounted, ref, watch } from "vue";
import { useResearchStore } from "../stores/research";
import { useCommandsStore } from "../stores/commands";
import { usePlaygroundStore } from "../stores/playground";
import { useAiStore } from "../stores/ai";
import {
  commandsByFramework,
  copyToClipboard,
  recordUsage,
} from "../composables/useTauri";
import type { Command, CommandPack } from "../types";
import CommandCard from "../components/CommandCard.vue";
import { invoke } from "@tauri-apps/api/core";

const research = useResearchStore();
const commands = useCommandsStore();
const playground = usePlaygroundStore();
const ai = useAiStore();
const frameworkCommands = ref<Command[]>([]);

const activeTab = ref<"frameworks" | "packs">("frameworks");
const packs = ref<CommandPack[]>([]);
const selectedPackId = ref<string | null>(null);
const packCommands = ref<Command[]>([]);

onMounted(async () => {
  research.loadFrameworks();
  try {
    packs.value = await invoke<CommandPack[]>("list_packs");
    if (packs.value.length > 0 && packs.value[0]) {
      selectedPackId.value = packs.value[0].id;
      packCommands.value = packs.value[0].commands;
    }
  } catch {
    packs.value = [];
  }
});

watch(
  () => research.selectedId,
  async (id) => {
    if (id) {
      frameworkCommands.value = await commandsByFramework(id);
    }
  },
);

watch(selectedPackId, (pid) => {
  const p = packs.value.find((pk: CommandPack) => pk.id === pid);
  if (p) {
    packCommands.value = p.commands;
  }
});

const selected = computed(() =>
  research.frameworks.find((f) => f.id === research.selectedId),
);

const selectedPack = computed(() =>
  packs.value.find((p: CommandPack) => p.id === selectedPackId.value),
);

async function onCopy(cmd: string, id: string) {
  await copyToClipboard(cmd);
  void recordUsage(id, "copy");
}

function onFavorite(cmd: Command) {
  commands.toggleFavorite(cmd);
  void recordUsage(cmd.id, commands.isFavorite(cmd.id) ? "favorite" : "unfavorite");
}

function toPlayground(cmd: string, id: string) {
  playground.insertCommand(cmd);
  void recordUsage(id, "playground");
}

async function onExplain(cmd: string, id: string) {
  await ai.explain(cmd);
  void recordUsage(id, "explain");
}
</script>

<template>
  <section class="research">
    <h1 class="t-headline-md">Research & Collections</h1>
    <p class="muted">
      Explore frameworks and curated command packs for specific workflows.
    </p>

    <div class="tabs-header">
      <button
        class="btn btn--sm"
        :class="activeTab === 'frameworks' ? 'btn--primary' : 'btn--ghost'"
        type="button"
        @click="activeTab = 'frameworks'"
      >
        Frameworks
      </button>
      <button
        class="btn btn--sm"
        :class="activeTab === 'packs' ? 'btn--primary' : 'btn--ghost'"
        type="button"
        @click="activeTab = 'packs'"
      >
        Curated Packs
      </button>
    </div>

    <!-- Frameworks Tab -->
    <template v-if="activeTab === 'frameworks'">
      <div class="research__grid">
        <button
          v-for="fw in research.frameworks"
          :key="fw.id"
          class="card research__tile"
          :class="{ 'research__tile--active': research.selectedId === fw.id }"
          type="button"
          @click="research.select(fw.id)"
        >
          <span class="t-title-md">{{ fw.name }}</span>
          <span class="muted">{{ fw.description }}</span>
        </button>
      </div>
      <div v-if="selected" class="research__detail">
        <h2 class="t-title-md">{{ selected.name }}</h2>
        <CommandCard
          v-for="cmd in frameworkCommands"
          :key="cmd.id"
          :command="cmd"
          @copy="onCopy(cmd.command, cmd.id)"
          @explain="onExplain(cmd.command, cmd.id)"
          @playground="toPlayground(cmd.command, cmd.id)"
          @favorite="onFavorite(cmd)"
        />
      </div>
    </template>

    <!-- Curated Packs Tab -->
    <template v-else>
      <div class="research__grid">
        <button
          v-for="pack in packs"
          :key="pack.id"
          class="card research__tile"
          :class="{ 'research__tile--active': selectedPackId === pack.id }"
          type="button"
          @click="selectedPackId = pack.id"
        >
          <span class="t-title-md">{{ pack.name }}</span>
          <span class="muted">{{ pack.description }}</span>
        </button>
      </div>
      <div v-if="selectedPack" class="research__detail">
        <div class="pack-detail-header">
          <div>
            <h2 class="t-title-md">{{ selectedPack.name }}</h2>
            <p class="muted">{{ selectedPack.description }}</p>
          </div>
          <span class="badge badge--muted">{{ packCommands.length }} commands</span>
        </div>
        <CommandCard
          v-for="cmd in packCommands"
          :key="cmd.id"
          :command="cmd"
          @copy="onCopy(cmd.command, cmd.id)"
          @explain="onExplain(cmd.command, cmd.id)"
          @playground="toPlayground(cmd.command, cmd.id)"
          @favorite="onFavorite(cmd)"
        />
        <p v-if="!packCommands.length" class="muted">No commands in this pack yet.</p>
      </div>
    </template>
  </section>
</template>

<style scoped>
.research {
  display: flex;
  flex-direction: column;
  gap: var(--space-4);
}
.tabs-header {
  display: flex;
  gap: var(--space-2);
}
.research__grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(160px, 1fr));
  gap: var(--space-3);
}
.research__tile {
  display: flex;
  flex-direction: column;
  gap: var(--space-1);
  text-align: left;
  background: var(--color-surface);
  border: var(--border-width) solid var(--color-border);
  padding: var(--space-3);
  cursor: pointer;
  transition: all 0.15s ease;
}
.research__tile:hover {
  border-color: var(--color-primary);
}
.research__tile--active {
  border-color: var(--color-primary);
  background: var(--color-surface-hover);
}
.research__detail {
  display: flex;
  flex-direction: column;
  gap: var(--space-3);
  margin-top: var(--space-2);
}
.pack-detail-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  background: var(--color-surface-hover);
  padding: var(--space-3);
  border-radius: var(--radius-md);
  border: var(--border-width) solid var(--color-border);
}
.muted {
  color: var(--color-text-muted);
  font-size: var(--text-body-sm-size);
  margin: 0;
}
</style>

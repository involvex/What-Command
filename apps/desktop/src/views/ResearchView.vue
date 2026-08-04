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
import type { Command } from "../types";
import CommandCard from "../components/CommandCard.vue";

const research = useResearchStore();
const commands = useCommandsStore();
const playground = usePlaygroundStore();
const ai = useAiStore();
const frameworkCommands = ref<Command[]>([]);

onMounted(() => research.loadFrameworks());

watch(
  () => research.selectedId,
  async (id) => {
    if (id) {
      frameworkCommands.value = await commandsByFramework(id);
    }
  },
);

const selected = computed(() =>
  research.frameworks.find((f) => f.id === research.selectedId),
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
    <h1 class="t-headline-md">Research</h1>
    <p class="muted">Frameworks and command references from the offline database.</p>
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
  </section>
</template>

<style scoped>
.research {
  display: flex;
  flex-direction: column;
  gap: var(--space-4);
}
.research__grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(140px, 1fr));
  gap: var(--space-3);
}
.research__tile {
  text-align: left;
  cursor: pointer;
  display: flex;
  flex-direction: column;
  gap: var(--space-1);
  min-height: 88px;
}
.research__tile--active {
  border-color: var(--color-primary);
}
.research__detail {
  display: flex;
  flex-direction: column;
  gap: var(--space-3);
}
.muted {
  color: var(--color-text-muted);
  font-size: var(--text-body-sm-size);
}
</style>

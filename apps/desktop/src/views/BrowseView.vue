<script setup lang="ts">
import { onMounted } from "vue";
import { useCommandsStore } from "../stores/commands";
import { usePlaygroundStore } from "../stores/playground";
import CommandCard from "../components/CommandCard.vue";
import SearchBar from "../components/SearchBar.vue";
import { copyToClipboard } from "../composables/useTauri";
import { useAiStore } from "../stores/ai";

const commands = useCommandsStore();
const playground = usePlaygroundStore();
const ai = useAiStore();

onMounted(() => commands.search(""));

async function onCopy(cmd: string) {
  await copyToClipboard(cmd);
}

function toPlayground(cmd: string) {
  playground.insertCommand(cmd);
}
</script>

<template>
  <section class="browse">
    <h1 class="t-headline-md">Browse</h1>
    <SearchBar v-model="commands.query" @search="commands.search()" />
    <p v-if="commands.loading" class="muted">Loading…</p>
    <div class="browse__list">
      <CommandCard
        v-for="cmd in commands.results"
        :key="cmd.id"
        :command="cmd"
        @copy="onCopy(cmd.command)"
        @explain="ai.explain(cmd.command)"
        @playground="toPlayground(cmd.command)"
        @favorite="commands.toggleFavorite(cmd)"
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

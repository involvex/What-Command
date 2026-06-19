<script setup lang="ts">
import type { Command } from "../types";

defineProps<{ command: Command }>();
defineEmits<{
  copy: [];
  explain: [];
  playground: [];
  favorite: [];
}>();

const isDanger = (cmd: Command) => cmd.danger_level >= 2;
</script>

<template>
  <article
    class="card command-card"
    :class="{ 'command-card--danger': isDanger(command) }"
  >
    <header class="command-card__header">
      <span class="badge">{{ command.category }}</span>
      <span class="badge badge--muted">{{ command.source }}</span>
      <span v-if="isDanger(command)" class="badge badge--warning">destructive</span>
    </header>
    <pre class="command-card__code mono">{{ command.command }}</pre>
    <p class="command-card__desc">{{ command.description }}</p>
    <footer class="command-card__actions">
      <button class="btn btn--ghost btn--sm" type="button" @click="$emit('copy')">
        Copy
      </button>
      <button class="btn btn--ghost btn--sm" type="button" @click="$emit('explain')">
        Explain
      </button>
      <button class="btn btn--ghost btn--sm" type="button" @click="$emit('playground')">
        Playground
      </button>
      <button class="btn btn--icon btn--sm" type="button" @click="$emit('favorite')">
        ★
      </button>
    </footer>
  </article>
</template>

<style scoped>
.command-card {
  display: flex;
  flex-direction: column;
  gap: var(--space-3);
}
.command-card--danger {
  border-color: var(--color-warning);
}
.command-card__header {
  display: flex;
  flex-wrap: wrap;
  gap: var(--space-2);
}
.command-card__code {
  margin: 0;
  padding: var(--space-3);
  background: var(--color-background);
  border-radius: var(--radius-sm);
  overflow-x: auto;
  font-family: var(--font-mono);
  font-size: var(--text-mono-sm-size);
}
.command-card__desc {
  margin: 0;
  color: var(--color-text-secondary);
  font-size: var(--text-body-sm-size);
}
.command-card__actions {
  display: flex;
  flex-wrap: wrap;
  gap: var(--space-2);
}
</style>

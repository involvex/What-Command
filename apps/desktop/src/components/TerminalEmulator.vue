<script setup lang="ts">
defineProps<{ lines: string[] }>();
</script>

<template>
  <div class="terminal card" aria-label="Simulated terminal">
    <div class="terminal__header label-sm">Simulated terminal — preview only</div>
    <pre class="terminal__body mono"><code><span
        v-for="(line, i) in lines"
        :key="i"
        class="terminal__line"
      >{{ line }}
</span></code></pre>
  </div>
</template>

<style scoped>
.terminal {
  background: var(--color-background);
  border: var(--border-width) solid var(--color-border);
  border-radius: var(--radius-md);
  overflow: hidden;
}
.terminal__header {
  padding: var(--space-2) var(--space-3);
  border-bottom: var(--border-width) solid var(--color-border);
  color: var(--color-text-muted);
  text-transform: uppercase;
  letter-spacing: var(--text-label-sm-tracking);
  font-size: var(--text-label-sm-size);
}
.terminal__body {
  margin: 0;
  padding: var(--space-4);
  min-height: 160px;
  max-height: 320px;
  overflow: auto;
  font-family: var(--font-mono);
  font-size: var(--text-mono-sm-size);
  color: var(--color-success);
}
.terminal__line {
  display: block;
  white-space: pre-wrap;
}
</style>

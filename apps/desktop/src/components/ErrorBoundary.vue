<script setup lang="ts">
import { onErrorCaptured, ref } from "vue";

const hasError = ref(false);
const errMsg = ref("");

onErrorCaptured((error: unknown) => {
  hasError.value = true;
  errMsg.value = error instanceof Error ? error.message : String(error);
  console.error("ErrorBoundary caught:", error);
  return false;
});
</script>

<template>
  <div v-if="hasError" class="error-boundary card">
    <h3 class="t-title-md">Something went wrong</h3>
    <p class="muted">{{ errMsg }}</p>
    <div class="actions">
      <button class="btn btn--primary btn--sm" @click="$forceUpdate()">Retry</button>
    </div>
  </div>
  <slot v-else />
</template>

<style scoped>
.error-boundary {
  padding: var(--space-4);
  text-align: center;
  background: var(--color-surface);
  border: var(--border-width) solid var(--color-danger);
  border-radius: var(--radius-md);
  color: var(--color-text);
}

.muted {
  color: var(--color-text-muted);
  margin: 0;
  font-size: var(--text-body-sm-size);
}

.actions {
  margin-top: var(--space-3);
  display: flex;
  justify-content: center;
}
</style>

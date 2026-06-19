<script setup lang="ts">
import { ref } from "vue";
import { useAiStore } from "../stores/ai";
import { usePlaygroundStore } from "../stores/playground";

const ai = useAiStore();
const playground = usePlaygroundStore();
const input = ref("");

async function send() {
  const text = input.value.trim();
  if (!text) return;
  input.value = "";
  await ai.ask(text);
}

function useInPlayground(suggestion?: { command: string }) {
  if (suggestion) playground.insertCommand(suggestion.command);
}
</script>

<template>
  <section class="ai-chat">
    <h1 class="t-headline-md">AI Chat</h1>
    <p class="muted">
      Natural language → command suggestions (OpenCode Zen / Kilo Gateway).
    </p>
    <div class="ai-chat__thread card">
      <p v-if="!ai.messages.length" class="muted">
        Ask how to do something in the terminal…
      </p>
      <div
        v-for="(msg, i) in ai.messages"
        :key="i"
        class="ai-chat__msg"
        :class="`ai-chat__msg--${msg.role}`"
      >
        <p>{{ msg.content }}</p>
        <template v-if="msg.suggestion">
          <pre class="mono">{{ msg.suggestion.command }}</pre>
          <button
            class="btn btn--ghost btn--sm"
            type="button"
            @click="useInPlayground(msg.suggestion)"
          >
            Use in Playground
          </button>
        </template>
      </div>
    </div>
    <form class="ai-chat__input" @submit.prevent="send">
      <input
        v-model="input"
        class="input"
        placeholder="Describe what you want to do…"
      />
      <button class="btn btn--primary" type="submit" :disabled="ai.loading">
        Send
      </button>
    </form>
  </section>
</template>

<style scoped>
.ai-chat {
  display: flex;
  flex-direction: column;
  gap: var(--space-4);
  min-height: 0;
  flex: 1;
}
.ai-chat__thread {
  flex: 1;
  overflow: auto;
  padding: var(--space-4);
  display: flex;
  flex-direction: column;
  gap: var(--space-3);
  min-height: 200px;
}
.ai-chat__msg--user {
  color: var(--color-info);
}
.ai-chat__input {
  display: flex;
  gap: var(--space-2);
}
.ai-chat__input .input {
  flex: 1;
}
.muted {
  color: var(--color-text-muted);
  margin: 0;
}
</style>

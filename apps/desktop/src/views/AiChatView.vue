<script setup lang="ts">
import { ref } from "vue";
import { useAiStore } from "../stores/ai";
import { usePlaygroundStore } from "../stores/playground";

const ai = useAiStore();
const playground = usePlaygroundStore();
const input = ref("");

const followUps = [
  "Now tail the logs for errors",
  "Filter output for status 500",
  "Explain this command's flags",
  "Make this into a pipeline",
];

async function send(textVal?: string) {
  const text = (textVal ?? input.value).trim();
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
    <div class="ai-chat__header">
      <div>
        <h1 class="t-headline-md">AI Chat & Context Chaining</h1>
        <p class="muted">
          Multi-turn conversation retaining recent commands and workspace context.
        </p>
      </div>
      <button
        v-if="ai.messages.length"
        class="btn btn--ghost btn--sm"
        type="button"
        @click="ai.clear()"
      >
        Clear Chat
      </button>
    </div>

    <div class="ai-chat__thread card">
      <p v-if="!ai.messages.length" class="muted">
        Ask how to do something in the terminal or chain commands…
      </p>
      <p v-if="ai.error" class="error">{{ ai.error }}</p>
      <div
        v-for="(msg, i) in ai.messages"
        :key="i"
        class="ai-chat__msg"
        :class="`ai-chat__msg--${msg.role}`"
      >
        <div class="msg-header">
          <span class="fw-bold">{{
            msg.role === "user" ? "You" : "AI Assistant"
          }}</span>
        </div>
        <p class="msg-content">{{ msg.content }}</p>
        <template v-if="msg.suggestion">
          <pre class="mono">{{ msg.suggestion.command }}</pre>
          <div class="msg-actions">
            <button
              class="btn btn--ghost btn--sm"
              type="button"
              @click="useInPlayground(msg.suggestion)"
            >
              Use in Playground
            </button>
          </div>
        </template>
      </div>
    </div>

    <!-- Quick follow-up chips -->
    <div v-if="ai.messages.length" class="follow-up-chips">
      <button
        v-for="chip in followUps"
        :key="chip"
        class="btn btn--ghost btn--sm"
        type="button"
        @click="send(chip)"
      >
        {{ chip }}
      </button>
    </div>

    <form class="ai-chat__input" @submit.prevent="send()">
      <input
        v-model="input"
        class="input"
        placeholder="Ask a follow-up or describe a terminal task…"
      />
      <button class="btn btn--primary" type="submit" :disabled="ai.loading">
        {{ ai.loading ? "Thinking..." : "Send" }}
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
.ai-chat__header {
  display: flex;
  align-items: center;
  justify-content: space-between;
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
.ai-chat__msg {
  display: flex;
  flex-direction: column;
  gap: var(--space-1);
  padding: var(--space-3);
  border-radius: var(--radius-md);
  border: var(--border-width) solid var(--color-border);
}
.ai-chat__msg--user {
  background: var(--color-surface-hover);
  align-self: flex-end;
  max-width: 85%;
}
.ai-chat__msg--assistant {
  background: var(--color-surface);
  align-self: flex-start;
  max-width: 85%;
}
.msg-header {
  font-size: 11px;
  color: var(--color-text-muted);
}
.msg-content {
  margin: 0;
  white-space: pre-wrap;
}
.msg-actions {
  display: flex;
  gap: var(--space-2);
  margin-top: var(--space-1);
}
.follow-up-chips {
  display: flex;
  gap: var(--space-2);
  overflow-x: auto;
  padding-bottom: var(--space-1);
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
.error {
  color: var(--color-danger, #c62828);
  font-size: var(--text-body-sm-size);
  margin: 0;
}
</style>

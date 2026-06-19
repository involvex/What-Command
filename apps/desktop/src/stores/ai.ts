import { defineStore } from "pinia";
import { ref } from "vue";
import type { CommandSuggestion } from "../types";
import * as api from "../composables/useTauri";

export interface ChatMessage {
  role: "user" | "assistant";
  content: string;
  suggestion?: CommandSuggestion;
}

export const useAiStore = defineStore("ai", () => {
  const messages = ref<ChatMessage[]>([]);
  const loading = ref(false);

  async function ask(prompt: string, frameworkId?: string) {
    messages.value.push({ role: "user", content: prompt });
    loading.value = true;
    try {
      const suggestion = await api.askAi(prompt, frameworkId);
      messages.value.push({
        role: "assistant",
        content: suggestion.explanation,
        suggestion,
      });
      return suggestion;
    } finally {
      loading.value = false;
    }
  }

  async function explain(command: string) {
    loading.value = true;
    try {
      return await api.explainCommand(command);
    } finally {
      loading.value = false;
    }
  }

  function clear() {
    messages.value = [];
  }

  return { messages, loading, ask, explain, clear };
});

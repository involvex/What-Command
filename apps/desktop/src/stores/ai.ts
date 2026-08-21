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
  const error = ref<string | null>(null);

  async function ask(prompt: string, frameworkId?: string) {
    error.value = null;
    const history = messages.value.map((m) => ({
      role: m.role,
      content: m.content,
    }));
    messages.value.push({ role: "user", content: prompt });
    loading.value = true;
    try {
      const suggestion = await api.askAi(prompt, frameworkId, history);
      messages.value.push({
        role: "assistant",
        content: suggestion.explanation,
        suggestion,
      });
      return suggestion;
    } catch {
      error.value = "AI suggestion failed — check your API settings.";
      const stub = {
        command: `# ${prompt}`,
        explanation: "AI unavailable. Check your API provider settings.",
        danger_level: 0,
        platforms: ["common"],
      };
      messages.value.push({
        role: "assistant",
        content: stub.explanation,
        suggestion: stub as CommandSuggestion,
      });
      return stub as CommandSuggestion;
    } finally {
      loading.value = false;
    }
  }

  async function explain(command: string) {
    error.value = null;
    loading.value = true;
    try {
      return await api.explainCommand(command);
    } catch {
      error.value = "AI explanation failed — check your API settings.";
      return "AI unavailable. Configure a provider in Settings → AI Settings.";
    } finally {
      loading.value = false;
    }
  }

  function clear() {
    messages.value = [];
    error.value = null;
  }

  return { messages, loading, error, ask, explain, clear };
});

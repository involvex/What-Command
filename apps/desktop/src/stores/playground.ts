import { defineStore } from "pinia";
import { ref } from "vue";
import type { SimulateResult } from "../types";
import * as api from "../composables/useTauri";

export const usePlaygroundStore = defineStore("playground", () => {
  const command = ref("echo 'Hello from What Command'");
  const transcript = ref<string[]>([]);
  const lastResult = ref<SimulateResult | null>(null);
  const variables = ref<Record<string, string>>({});

  async function simulate() {
    const result = await api.simulateCommand(command.value, variables.value);
    lastResult.value = result;
    transcript.value.push(`$ ${command.value}`);
    if (result.blocked) {
      transcript.value.push(result.explanation);
    } else {
      transcript.value.push(result.sample_output);
    }
    return result;
  }

  function insertCommand(text: string) {
    command.value = text;
  }

  function clearTranscript() {
    transcript.value = [];
    lastResult.value = null;
  }

  return {
    command,
    transcript,
    lastResult,
    variables,
    simulate,
    insertCommand,
    clearTranscript,
  };
});

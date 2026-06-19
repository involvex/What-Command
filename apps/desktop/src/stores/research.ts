import { defineStore } from "pinia";
import { ref } from "vue";
import type { Framework } from "../types";
import * as api from "../composables/useTauri";

export const useResearchStore = defineStore("research", () => {
  const frameworks = ref<Framework[]>([]);
  const selectedId = ref<string | null>(null);
  const loading = ref(false);

  async function loadFrameworks() {
    loading.value = true;
    try {
      frameworks.value = await api.listFrameworks();
    } finally {
      loading.value = false;
    }
  }

  function select(id: string) {
    selectedId.value = id;
  }

  return { frameworks, selectedId, loading, loadFrameworks, select };
});

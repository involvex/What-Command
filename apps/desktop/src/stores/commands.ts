import { defineStore } from "pinia";
import { ref } from "vue";
import type { Command } from "../types";
import * as api from "../composables/useTauri";

export const useCommandsStore = defineStore("commands", () => {
  const results = ref<Command[]>([]);
  const loading = ref(false);
  const query = ref("");
  const favorites = ref<Command[]>([]);

  async function search(q?: string) {
    loading.value = true;
    query.value = q ?? query.value;
    try {
      results.value = await api.searchCommands(query.value);
    } finally {
      loading.value = false;
    }
  }

  function toggleFavorite(cmd: Command) {
    const idx = favorites.value.findIndex((f) => f.id === cmd.id);
    if (idx >= 0) {
      favorites.value.splice(idx, 1);
    } else {
      favorites.value.push(cmd);
    }
  }

  function isFavorite(id: string) {
    return favorites.value.some((f) => f.id === id);
  }

  return { results, loading, query, favorites, search, toggleFavorite, isFavorite };
});

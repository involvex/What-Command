import { defineStore } from "pinia";
import { ref } from "vue";
import type { Command, FilterOptions } from "../types";
import * as api from "../composables/useTauri";

export const useCommandsStore = defineStore("commands", () => {
  const results = ref<Command[]>([]);
  const loading = ref(false);
  const query = ref("");
  const favorites = ref<Command[]>([]);
  const filterOptions = ref<FilterOptions | null>(null);
  const filters = ref({
    category: "",
    platform: "",
    source: "",
    dangerMax: null as number | null,
  });

  async function loadFilterOptions() {
    try {
      filterOptions.value = await api.getFilterOptions();
    } catch {
      filterOptions.value = null;
    }
  }

  async function search(q?: string) {
    loading.value = true;
    query.value = q ?? query.value;
    try {
      const hasFilters =
        filters.value.category ||
        filters.value.platform ||
        filters.value.source ||
        filters.value.dangerMax !== null;
      if (hasFilters || query.value.trim().length > 0) {
        results.value = await api.searchCommandsFiltered(query.value, 50, {
          category: filters.value.category || undefined,
          platform: filters.value.platform || undefined,
          source: filters.value.source || undefined,
          dangerMax: filters.value.dangerMax ?? undefined,
        });
      } else {
        results.value = [];
      }
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

  function clearFilters() {
    filters.value = {
      category: "",
      platform: "",
      source: "",
      dangerMax: null,
    };
    void search();
  }

  return {
    results,
    loading,
    query,
    favorites,
    filterOptions,
    filters,
    search,
    loadFilterOptions,
    toggleFavorite,
    isFavorite,
    clearFilters,
  };
});

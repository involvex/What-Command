import { defineStore } from "pinia";
import { ref } from "vue";
import type { AppSettings } from "../types";
import * as api from "../composables/useTauri";

export const useSettingsStore = defineStore("settings", () => {
  const settings = ref<AppSettings | null>(null);
  const loading = ref(false);

  async function load() {
    loading.value = true;
    try {
      settings.value = await api.getSettings();
    } finally {
      loading.value = false;
    }
  }

  async function save(next: AppSettings) {
    await api.saveSettings(next);
    settings.value = next;
  }

  return { settings, loading, load, save };
});

<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref } from "vue";
import { useLayout } from "./composables/useLayout";
import MobileTabShell from "./layouts/MobileTabShell.vue";
import DesktopShell from "./layouts/DesktopShell.vue";
import CommandPalette from "./components/CommandPalette.vue";
import { listenShowCommandPalette } from "./composables/useTauri";

const { isMobile } = useLayout();
const shell = computed(() => (isMobile.value ? MobileTabShell : DesktopShell));

const paletteOpen = ref(false);
const isOffline = ref(false);
let unlistenShortcut: (() => void) | null = null;
let keydownHandler: ((e: KeyboardEvent) => void) | null = null;
let onlineHandler: (() => void) | null = null;
let offlineHandler: (() => void) | null = null;

onMounted(async () => {
  isOffline.value = !navigator.onLine;

  unlistenShortcut = await listenShowCommandPalette(() => {
    paletteOpen.value = true;
  });

  keydownHandler = (e: KeyboardEvent) => {
    if ((e.ctrlKey || e.metaKey) && e.key === "k") {
      e.preventDefault();
      paletteOpen.value = true;
    }
  };
  window.addEventListener("keydown", keydownHandler);

  onlineHandler = () => {
    isOffline.value = false;
  };
  offlineHandler = () => {
    isOffline.value = true;
  };
  window.addEventListener("online", onlineHandler);
  window.addEventListener("offline", offlineHandler);
});

onUnmounted(() => {
  unlistenShortcut?.();
  if (keydownHandler) {
    window.removeEventListener("keydown", keydownHandler);
  }
  if (onlineHandler) {
    window.removeEventListener("online", onlineHandler);
  }
  if (offlineHandler) {
    window.removeEventListener("offline", offlineHandler);
  }
});
</script>

<template>
  <div v-if="isOffline" class="offline-banner card">
    <span>Offline mode — local search and AI may be limited.</span>
  </div>
  <component :is="shell" />
  <CommandPalette v-model:visible="paletteOpen" />
</template>

<style>
#app {
  min-height: 100vh;
}
.offline-banner {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  padding: var(--space-2) var(--space-4);
  background: var(--color-warning);
  color: var(--color-text-on-warning, #000);
  font-size: var(--text-body-sm-size);
  text-align: center;
  z-index: 999;
}
</style>

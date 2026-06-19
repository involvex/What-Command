<script setup lang="ts">
import { computed } from "vue";
import { useLayout } from "./composables/useLayout";
import MobileTabShell from "./layouts/MobileTabShell.vue";
import DesktopShell from "./layouts/DesktopShell.vue";

const { isMobile } = useLayout();
const shell = computed(() => (isMobile.value ? MobileTabShell : DesktopShell));
</script>

<template>
  <component :is="shell" />
</template>

<style>
#app {
  min-height: 100vh;
}
</style>

<script setup lang="ts">
import { onMounted } from "vue";
import { useCommandsStore } from "../stores/commands";
import { useSettingsStore } from "../stores/settings";
import SettingsForm from "../components/SettingsForm.vue";

const favorites = useCommandsStore();
const settings = useSettingsStore();

onMounted(() => settings.load());
</script>

<template>
  <section class="more">
    <h1 class="t-headline-md">More</h1>
    <SettingsForm />
    <div class="card">
      <h2 class="t-title-md">Favorites</h2>
      <p v-if="!favorites.favorites.length" class="muted">
        Star commands from Browse to save them.
      </p>
      <ul>
        <li v-for="f in favorites.favorites" :key="f.id" class="mono">
          {{ f.command }}
        </li>
      </ul>
    </div>
  </section>
</template>

<style scoped>
.more {
  display: flex;
  flex-direction: column;
  gap: var(--space-4);
}
.muted {
  color: var(--color-text-muted);
}
</style>

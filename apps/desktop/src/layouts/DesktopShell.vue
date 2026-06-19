<script setup lang="ts">
import { RouterLink, RouterView } from "vue-router";

const links = [
  { to: "/browse", label: "Browse" },
  { to: "/playground", label: "Playground" },
  { to: "/research", label: "Research" },
  { to: "/ai", label: "AI Chat" },
  { to: "/more", label: "More" },
];
</script>

<template>
  <div class="desktop-shell">
    <aside class="desktop-shell__sidebar card">
      <h1 class="t-title-md desktop-shell__brand">What Command</h1>
      <nav class="desktop-shell__nav">
        <RouterLink
          v-for="link in links"
          :key="link.to"
          :to="link.to"
          class="desktop-shell__link"
          active-class="desktop-shell__link--active"
        >
          {{ link.label }}
        </RouterLink>
      </nav>
    </aside>
    <main class="desktop-shell__main">
      <RouterView />
    </main>
  </div>
</template>

<style scoped>
.desktop-shell {
  display: grid;
  grid-template-columns: 240px 1fr;
  min-height: 100vh;
}
.desktop-shell__sidebar {
  padding: var(--space-4);
  border-radius: 0;
  border-right: var(--border-width) solid var(--color-border);
}
.desktop-shell__brand {
  margin: 0 0 var(--space-6);
}
.desktop-shell__nav {
  display: flex;
  flex-direction: column;
  gap: var(--space-1);
}
.desktop-shell__link {
  padding: var(--space-3) var(--space-3);
  border-radius: var(--radius-sm);
  color: var(--color-text-secondary);
  text-decoration: none;
}
.desktop-shell__link--active {
  background: var(--color-primary-soft);
  color: var(--color-primary);
}
.desktop-shell__main {
  padding: var(--space-6);
  overflow: auto;
}
</style>

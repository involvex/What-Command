<script setup lang="ts">
import { RouterLink, RouterView, useRoute } from "vue-router";

const tabs = [
  { to: "/browse", label: "Browse" },
  { to: "/playground", label: "Playground" },
  { to: "/research", label: "Research" },
  { to: "/ai", label: "AI" },
  { to: "/more", label: "More" },
];

const route = useRoute();
</script>

<template>
  <div class="mobile-shell">
    <main class="mobile-shell__content">
      <RouterView />
    </main>
    <nav class="mobile-shell__tabs" aria-label="Main">
      <RouterLink
        v-for="tab in tabs"
        :key="tab.to"
        :to="tab.to"
        class="mobile-shell__tab"
        :class="{ 'mobile-shell__tab--active': route.path === tab.to }"
      >
        {{ tab.label }}
      </RouterLink>
    </nav>
  </div>
</template>

<style scoped>
.mobile-shell {
  display: flex;
  flex-direction: column;
  min-height: 100vh;
  min-height: 100dvh;
}
.mobile-shell__content {
  flex: 1;
  padding: var(--space-4);
  padding-bottom: calc(56px + env(safe-area-inset-bottom));
  overflow: auto;
}
.mobile-shell__tabs {
  position: fixed;
  bottom: 0;
  left: 0;
  right: 0;
  display: flex;
  background: var(--color-surface);
  border-top: var(--border-width) solid var(--color-border);
  padding-bottom: env(safe-area-inset-bottom);
  z-index: 10;
}
.mobile-shell__tab {
  flex: 1;
  min-height: 48px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--color-text-muted);
  text-decoration: none;
  font-size: var(--text-body-sm-size);
  font-weight: 500;
}
.mobile-shell__tab--active {
  color: var(--color-primary);
}
</style>

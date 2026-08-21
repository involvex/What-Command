<script setup lang="ts">
import { RouterLink, RouterView } from "vue-router";

const links = [
  { to: "/browse", label: "Browse", icon: "search" },
  { to: "/playground", label: "Playground", icon: "terminal" },
  { to: "/research", label: "Research", icon: "sparkles" },
  { to: "/ai", label: "AI Chat", icon: "cpu" },
  { to: "/more", label: "More", icon: "settings" },
];
</script>

<template>
  <div class="desktop-shell">
    <aside class="desktop-shell__sidebar card">
      <div class="desktop-shell__brand-wrap">
        <div class="desktop-shell__logo">WC</div>
        <h1 class="t-title-md desktop-shell__brand">What Command</h1>
      </div>
      <nav class="desktop-shell__nav">
        <RouterLink
          v-for="link in links"
          :key="link.to"
          :to="link.to"
          class="desktop-shell__link"
          active-class="desktop-shell__link--active"
        >
          <span class="desktop-shell__link-dot"></span>
          {{ link.label }}
        </RouterLink>
      </nav>
      <div class="desktop-shell__footer">
        <div class="desktop-shell__status">
          <span class="status-indicator"></span>
          <span>Local Engine Active</span>
        </div>
      </div>
    </aside>
    <main class="desktop-shell__main">
      <RouterView />
    </main>
  </div>
</template>

<style scoped>
.desktop-shell {
  display: grid;
  grid-template-columns: 260px 1fr;
  min-height: 100vh;
  background: var(--color-background);
}
.desktop-shell__sidebar {
  padding: var(--space-6) var(--space-4);
  border-radius: 0;
  border-right: var(--border-width) solid var(--color-border);
  background: var(--color-surface);
  display: flex;
  flex-direction: column;
  justify-content: space-between;
}
.desktop-shell__brand-wrap {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  margin-bottom: var(--space-8);
  padding: 0 var(--space-2);
}
.desktop-shell__logo {
  width: 36px;
  height: 36px;
  border-radius: var(--radius-sm);
  background: linear-gradient(135deg, var(--color-primary), #7886ff);
  color: #fff;
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: 700;
  font-size: 0.9rem;
  box-shadow: 0 4px 12px rgba(91, 107, 255, 0.3);
}
.desktop-shell__brand {
  margin: 0;
  font-size: 1.1rem;
  letter-spacing: -0.01em;
}
.desktop-shell__nav {
  display: flex;
  flex-direction: column;
  gap: var(--space-2);
}
.desktop-shell__link {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  padding: var(--space-3) var(--space-4);
  border-radius: var(--radius-sm);
  color: var(--color-text-secondary);
  text-decoration: none;
  font-weight: 500;
  transition: all 0.2s ease;
}
.desktop-shell__link:hover {
  background: rgba(255, 255, 255, 0.03);
  color: var(--color-text-primary);
}
.desktop-shell__link--active {
  background: var(--color-primary-soft);
  color: var(--color-primary);
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.2);
}
.desktop-shell__link-dot {
  width: 6px;
  height: 6px;
  border-radius: 50%;
  background: currentColor;
  opacity: 0.3;
}
.desktop-shell__link--active .desktop-shell__link-dot {
  opacity: 1;
  box-shadow: 0 0 8px var(--color-primary);
}
.desktop-shell__footer {
  padding-top: var(--space-4);
  border-top: var(--border-width) solid var(--color-border);
}
.desktop-shell__status {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  font-size: var(--text-body-sm-size);
  color: var(--color-text-muted);
}
.status-indicator {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: var(--color-success);
  box-shadow: 0 0 8px rgba(43, 224, 140, 0.4);
}
.desktop-shell__main {
  padding: var(--space-8);
  overflow: auto;
  background: radial-gradient(
    circle at top right,
    rgba(91, 107, 255, 0.04),
    transparent 40%
  );
}
</style>

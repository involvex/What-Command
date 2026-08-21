<script setup lang="ts">
import { RouterLink, RouterView, useRoute } from "vue-router";
import { ref } from "vue";

const tabs = [
  { to: "/browse", label: "Browse", icon: "🔍" },
  { to: "/playground", label: "Terminal", icon: "💻" },
  { to: "/research", label: "Research", icon: "📚" },
  { to: "/ai", label: "AI Chat", icon: "🤖" },
  { to: "/more", label: "More", icon: "⚙️" },
];

const route = useRoute();
const touchStartX = ref(0);
const touchEndX = ref(0);

function handleTouchStart(e: TouchEvent) {
  if (e.changedTouches[0]) {
    touchStartX.value = e.changedTouches[0].clientX;
  }
}

function handleTouchEnd(e: TouchEvent) {
  if (e.changedTouches[0]) {
    touchEndX.value = e.changedTouches[0].clientX;
    handleSwipe();
  }
}

function handleSwipe() {
  const diff = touchEndX.value - touchStartX.value;
  if (Math.abs(diff) < 70) return;

  const currentIndex = tabs.findIndex((t) => t.to === route.path);
  if (currentIndex === -1) return;

  if (diff > 0 && currentIndex > 0) {
    // Swipe right -> previous tab
    const target = tabs[currentIndex - 1];
    if (target) window.location.hash = `#${target.to}`;
  } else if (diff < 0 && currentIndex < tabs.length - 1) {
    // Swipe left -> next tab
    const target = tabs[currentIndex + 1];
    if (target) window.location.hash = `#${target.to}`;
  }
}
</script>

<template>
  <div class="mobile-shell" @touchstart="handleTouchStart" @touchend="handleTouchEnd">
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
        <span class="tab-icon">{{ tab.icon }}</span>
        <span class="tab-label">{{ tab.label }}</span>
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
  background: var(--color-background);
  color: var(--color-text);
}
.mobile-shell__content {
  flex: 1;
  padding: var(--space-4);
  padding-bottom: calc(64px + env(safe-area-inset-bottom));
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
  box-shadow: 0 -4px 12px rgba(0, 0, 0, 0.05);
}
.mobile-shell__tab {
  flex: 1;
  min-height: 56px;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 2px;
  color: var(--color-text-muted);
  text-decoration: none;
  font-size: 11px;
  font-weight: 500;
  transition: color 0.15s ease;
}
.tab-icon {
  font-size: 18px;
}
.mobile-shell__tab--active {
  color: var(--color-primary);
}
</style>

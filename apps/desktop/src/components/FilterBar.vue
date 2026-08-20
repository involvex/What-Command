<script setup lang="ts">
import { computed, onMounted } from "vue";
import { useCommandsStore } from "../stores/commands";

const commands = useCommandsStore();

const filterOpts = computed(() => commands.filterOptions);

const hasActiveFilters = computed(() => {
  return (
    !!commands.filters.category ||
    !!commands.filters.platform ||
    !!commands.filters.source ||
    commands.filters.dangerMax !== null
  );
});

const dangerLevels = [
  { label: "Safe", value: 0 },
  { label: "Low", value: 1 },
  { label: "Medium", value: 2 },
  { label: "High", value: 3 },
];

onMounted(() => {
  if (!filterOpts.value) {
    void commands.loadFilterOptions();
  }
});

function setFilter<K extends keyof typeof commands.filters>(
  key: K,
  value: (typeof commands.filters)[K],
) {
  commands.filters[key] = value;
  void commands.search();
}
</script>

<template>
  <div v-if="filterOpts" class="filter-bar">
    <div class="filter-bar__row">
      <div class="filter-bar__group">
        <label class="filter-bar__label">Category</label>
        <select
          v-model="commands.filters.category"
          class="input input--sm"
          @change="void commands.search()"
        >
          <option value="">All</option>
          <option v-for="cat in filterOpts.categories" :key="cat" :value="cat">
            {{ cat }}
          </option>
        </select>
      </div>

      <div class="filter-bar__group">
        <label class="filter-bar__label">Platform</label>
        <select
          v-model="commands.filters.platform"
          class="input input--sm"
          @change="void commands.search()"
        >
          <option value="">All</option>
          <option v-for="plat in filterOpts.platforms" :key="plat" :value="plat">
            {{ plat }}
          </option>
        </select>
      </div>

      <div class="filter-bar__group">
        <label class="filter-bar__label">Source</label>
        <select
          v-model="commands.filters.source"
          class="input input--sm"
          @change="void commands.search()"
        >
          <option value="">All</option>
          <option v-for="src in filterOpts.sources" :key="src" :value="src">
            {{ src }}
          </option>
        </select>
      </div>

      <div class="filter-bar__group">
        <label class="filter-bar__label">Max Danger</label>
        <select
          v-model="commands.filters.dangerMax"
          class="input input--sm"
          @change="void commands.search()"
        >
          <option :value="null">All</option>
          <option v-for="d in dangerLevels" :key="d.value" :value="d.value">
            {{ d.label }}
          </option>
        </select>
      </div>

      <button
        v-if="hasActiveFilters"
        class="btn btn--ghost btn--sm filter-bar__clear"
        type="button"
        @click="commands.clearFilters()"
      >
        Clear
      </button>
    </div>

    <div v-if="hasActiveFilters" class="filter-bar__chips">
      <span v-if="commands.filters.category" class="chip">
        {{ commands.filters.category }}
        <button
          type="button"
          aria-label="Remove category filter"
          @click="setFilter('category', '')"
        >
          ✕
        </button>
      </span>
      <span v-if="commands.filters.platform" class="chip">
        {{ commands.filters.platform }}
        <button
          type="button"
          aria-label="Remove platform filter"
          @click="setFilter('platform', '')"
        >
          ✕
        </button>
      </span>
      <span v-if="commands.filters.source" class="chip">
        {{ commands.filters.source }}
        <button
          type="button"
          aria-label="Remove source filter"
          @click="setFilter('source', '')"
        >
          ✕
        </button>
      </span>
      <span v-if="commands.filters.dangerMax !== null" class="chip">
        Danger ≤ {{ commands.filters.dangerMax }}
        <button
          type="button"
          aria-label="Remove danger filter"
          @click="setFilter('dangerMax', null)"
        >
          ✕
        </button>
      </span>
    </div>
  </div>
</template>

<style scoped>
.filter-bar {
  display: flex;
  flex-direction: column;
  gap: var(--space-2);
}
.filter-bar__row {
  display: flex;
  flex-wrap: wrap;
  gap: var(--space-2);
  align-items: flex-end;
}
.filter-bar__group {
  display: flex;
  flex-direction: column;
  gap: var(--space-1);
  min-width: 120px;
  flex: 1;
}
.filter-bar__label {
  font-size: var(--text-label-sm-size);
  font-weight: var(--text-label-sm-weight);
  letter-spacing: var(--text-label-sm-tracking);
  color: var(--color-text-secondary);
}
.filter-bar__clear {
  margin-bottom: var(--space-1);
}
.filter-bar__chips {
  display: flex;
  flex-wrap: wrap;
  gap: var(--space-2);
}
.chip {
  display: inline-flex;
  align-items: center;
  gap: var(--space-1);
  padding: var(--space-1) var(--space-2);
  background: var(--color-primary-soft);
  color: var(--color-primary);
  border-radius: var(--radius-sm);
  font-size: var(--text-label-sm-size);
  font-weight: var(--text-label-sm-weight);
}
.chip button {
  background: transparent;
  border: none;
  color: inherit;
  cursor: pointer;
  font-size: var(--text-label-sm-size);
  padding: 0;
  line-height: 1;
}
</style>

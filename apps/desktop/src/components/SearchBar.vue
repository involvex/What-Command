<script setup lang="ts">
defineProps<{ modelValue: string; placeholder?: string }>();
defineEmits<{ "update:modelValue": [value: string]; search: [] }>();
</script>

<template>
  <div class="search-bar">
    <input
      class="input"
      type="search"
      :value="modelValue"
      :placeholder="placeholder ?? 'Search commands…'"
      @input="$emit('update:modelValue', ($event.target as HTMLInputElement).value)"
      @keydown.enter="$emit('search')"
    />
    <button class="btn btn--primary" type="button" @click="$emit('search')">
      Search
    </button>
  </div>
</template>

<style scoped>
.search-bar {
  display: flex;
  gap: var(--space-2);
}
.search-bar .input {
  flex: 1;
}
</style>

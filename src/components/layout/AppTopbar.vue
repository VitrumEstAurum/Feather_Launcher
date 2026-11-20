
<script setup lang="ts">
import { computed } from "vue";
import { useRoute } from "vue-router";

const route = useRoute();

const pageTitle = computed(() => {
  return String(route.name ?? "Home");
});
</script>

<template>
  <header class="topbar">
    <div class="breadcrumb">
      <span>Feather</span>
      <span class="breadcrumb-separator">/</span>
      <strong>{{ pageTitle }}</strong>
    </div>

    <div class="topbar-right">
      <div class="system-status">
        <span class="system-dot"></span>
        System ready
      </div>

      <div class="version-label">0.1.0</div>
    </div>
  </header>
</template>

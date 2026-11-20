
<script setup lang="ts">
defineProps<{
  title: string;
  icon?: string;
}>();
</script>

<template>
  <section class="placeholder-page">
    <div class="placeholder-icon">{{ icon ?? "◈" }}</div>
    <div class="eyebrow">FEATHER WORKSPACE</div>
    <h1>{{ title }}</h1>
    <p>This section is under development.</p>
    <span class="development-badge">
      COMING IN FUTURE BUILDS
    </span>
  </section>
</template>


<script setup lang="ts">
import AppSidebar from "./AppSidebar.vue";
import AppTopbar from "./AppTopbar.vue";
</script>

<template>
  <div class="app-shell">
    <AppSidebar />

    <main class="main-content">
      <AppTopbar />

      <div class="page-content">
        <RouterView />

        <footer class="page-footer">
          <span>FEATHER LAUNCHER</span>
          <span>Built for Minecraft Java Edition</span>
        </footer>
      </div>
    </main>
  </div>
</template>

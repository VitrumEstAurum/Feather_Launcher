
<script setup lang="ts">
defineProps<{
  activePage: string;
}>();

const emit = defineEmits<{
  navigate: [page: string];
}>();

const navigation = [
  { name: "Home", icon: "⌂" },
  { name: "Catalog", icon: "▦" },
  { name: "Instances", icon: "◈" },
  { name: "Downloads", icon: "↓" },
];

const settingsNavigation = [
  { name: "Settings", icon: "⚙" },
];
</script>

<template>
  <aside class="sidebar">
    <div class="brand">
      <div class="brand-icon">F</div>
      <div class="brand-name">
        <span>FEATHER</span>
        <small>LAUNCHER</small>
      </div>
    </div>

    <div class="sidebar-label">WORKSPACE</div>

    <nav class="navigation">
      <button
        v-for="item in navigation"
        :key="item.name"
        class="nav-item"
        :class="{ active: activePage === item.name }"
        @click="emit('navigate', item.name)"
      >
        <span class="nav-icon">{{ item.icon }}</span>
        <span>{{ item.name }}</span>
      </button>
    </nav>

    <div class="sidebar-label settings-label">PREFERENCES</div>

    <nav class="navigation">
      <button
        v-for="item in settingsNavigation"
        :key="item.name"
        class="nav-item"
        :class="{ active: activePage === item.name }"
        @click="emit('navigate', item.name)"
      >
        <span class="nav-icon">{{ item.icon }}</span>
        <span>{{ item.name }}</span>
      </button>
    </nav>

    <div class="sidebar-bottom">
      <div class="account-card">
        <div class="account-avatar">?</div>
        <div class="account-info">
          <strong>Local profile</strong>
          <span>No account connected</span>
        </div>
        <span class="account-menu">···</span>
      </div>

      <div class="sidebar-footer">
        <span>FEATHER v0.1.0</span>
        <span class="footer-dot"></span>
      </div>
    </div>
  </aside>
</template>

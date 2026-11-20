
<script setup lang="ts">
const navigation = [
  { name: "Home", icon: "⌂", path: "/" },
  { name: "Catalog", icon: "▦", path: "/catalog" },
  { name: "Instances", icon: "◈", path: "/instances" },
  { name: "Downloads", icon: "↓", path: "/downloads" },
];

const settingsNavigation = [
  { name: "Settings", icon: "⚙", path: "/settings" },
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
      <RouterLink
        v-for="item in navigation"
        :key="item.name"
        :to="item.path"
        class="nav-item"
        active-class="active"
      >
        <span class="nav-icon">{{ item.icon }}</span>
        <span>{{ item.name }}</span>
      </RouterLink>
    </nav>

    <div class="sidebar-label settings-label">PREFERENCES</div>

    <nav class="navigation">
      <RouterLink
        v-for="item in settingsNavigation"
        :key="item.name"
        :to="item.path"
        class="nav-item"
        active-class="active"
      >
        <span class="nav-icon">{{ item.icon }}</span>
        <span>{{ item.name }}</span>
      </RouterLink>
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

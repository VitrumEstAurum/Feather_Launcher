
<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

type ApiStatus = "idle" | "checking" | "online" | "offline";

const apiStatus = ref<ApiStatus>("idle");
const apiMessage = ref("Connection has not been checked");
const lastChecked = ref("");

async function checkModrinth() {
  if (apiStatus.value === "checking") return;

  apiStatus.value = "checking";
  apiMessage.value = "Connecting to Modrinth...";

  try {
    const result = await invoke<string>("check_api_connection");
    apiStatus.value = "online";
    apiMessage.value = result;
  } catch (error) {
    apiStatus.value = "offline";
    apiMessage.value = String(error);
  }

  lastChecked.value = new Date().toLocaleTimeString("ru-RU");
}
</script>

<template>
  <section class="welcome-section">
    <div>
      <div class="eyebrow">YOUR MINECRAFT WORKSPACE</div>
      <h1>Welcome to <span>Feather.</span></h1>
      <p>Everything you need to manage your Minecraft experience.</p>
    </div>
    <div class="welcome-decoration">F</div>
  </section>

  <section class="section">
    <div class="section-heading">
      <div>
        <h2>Overview</h2>
        <p>Your launcher at a glance.</p>
      </div>
    </div>

    <div class="overview-grid">
      <div class="overview-card">
        <div class="card-icon purple">◈</div>
        <div class="card-label">INSTANCES</div>
        <div class="card-value">0</div>
        <div class="card-description">No instances created yet</div>
      </div>

      <div class="overview-card">
        <div class="card-icon blue">▦</div>
        <div class="card-label">INSTALLED MODS</div>
        <div class="card-value">0</div>
        <div class="card-description">Nothing installed yet</div>
      </div>

      <div class="overview-card">
        <div class="card-icon green">↓</div>
        <div class="card-label">DOWNLOADS</div>
        <div class="card-value">0</div>
        <div class="card-description">No active downloads</div>
      </div>
    </div>
  </section>

  <section class="section">
    <div class="section-heading">
      <div>
        <h2>System diagnostics</h2>
        <p>Check the availability of external services.</p>
      </div>
      <span class="section-tag">DEVELOPMENT</span>
    </div>

    <div class="provider-card">
      <div class="provider-main">
        <div class="provider-logo">M</div>
        <div class="provider-info">
          <strong>Modrinth</strong>
          <span>Mod catalogue provider</span>
        </div>
      </div>

      <div class="provider-details">
        <div class="provider-status">
          <span class="status-dot" :class="apiStatus"></span>
          <span v-if="apiStatus === 'idle'">Not tested</span>
          <span v-else-if="apiStatus === 'checking'">Checking...</span>
          <span v-else-if="apiStatus === 'online'">Connected</span>
          <span v-else>Connection failed</span>
        </div>

        <button
          class="check-button"
          :disabled="apiStatus === 'checking'"
          @click="checkModrinth"
        >
          <span v-if="apiStatus === 'checking'" class="spinner"></span>
          {{ apiStatus === "checking" ? "Checking" : "Check connection" }}
        </button>
      </div>

      <div v-if="apiStatus !== 'idle'" class="api-result" :class="apiStatus">
        <div class="result-message">{{ apiMessage }}</div>
        <div v-if="lastChecked" class="result-time">
          Last checked: {{ lastChecked }}
        </div>
      </div>
    </div>
  </section>

  <section class="section">
    <div class="section-heading">
      <div>
        <h2>Recent activity</h2>
        <p>What's happening in Feather.</p>
      </div>
    </div>

    <div class="empty-activity">
      <div class="empty-icon">◷</div>
      <strong>No activity yet</strong>
      <span>Your recent actions will appear here.</span>
    </div>
  </section>
</template>


<script setup lang="ts">
import AppLayout from "./components/layout/AppLayout.vue";
</script>

<template>
  <AppLayout />
</template>


<style>
:root {
  font-family: Inter, "Segoe UI", sans-serif;
  color: #e5e8f0;
  background: #111318;
  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;

  --background: #111318;
  --sidebar: #171a21;
  --surface: #20242d;
  --elevated: #292e39;
  --accent: #879bdb;
  --text: #e5e8f0;
  --muted: #9299a9;
  --border: rgba(255, 255, 255, 0.065);
}

* {
  box-sizing: border-box;
}

body {
  margin: 0;
  min-width: 700px;
  min-height: 100vh;
}

button {
  font: inherit;
}

.app-shell {
  display: flex;
  min-height: 100vh;
  background: var(--background);
}

/* Sidebar */

.sidebar {
  height: 100vh;
  position: sticky;
  top: 0;
  align-self: flex-start;
  width: 238px;
  min-width: 238px;
  background: var(--sidebar);
  border-right: 1px solid var(--border);
  display: flex;
  flex-direction: column;
  padding: 25px 15px 16px;
}

.brand {
  display: flex;
  align-items: center;
  gap: 11px;
  padding: 0 9px;
  margin-bottom: 49px;
}

.brand-icon {
  width: 39px;
  height: 39px;
  display: grid;
  place-items: center;
  background: var(--accent);
  color: #111318;
  border-radius: 11px;
  font-size: 23px;
  font-weight: 800;
}

.brand-name {
  display: flex;
  flex-direction: column;
  gap: 2px;
  letter-spacing: 2px;
  font-size: 14px;
  font-weight: 800;
}

.brand-name small {
  font-size: 9px;
  letter-spacing: 3px;
  color: var(--muted);
}

.sidebar-label {
  padding: 0 11px;
  margin-bottom: 12px;
  font-size: 9px;
  letter-spacing: 1.6px;
  color: #747c8d;
  font-weight: 700;
}

.navigation {
  display: flex;
  flex-direction: column;
  gap: 5px;
}

.nav-item {
  height: 43px;
  display: flex;
  align-items: center;
  gap: 13px;
  padding: 0 12px;
  border: 1px solid transparent;
  border-radius: 8px;
  background: transparent;
  color: #9da4b3;
  font-size: 13px;
  text-align: left;
  cursor: pointer;
  transition: 0.18s ease;
}

.nav-item:hover {
  background: rgba(255, 255, 255, 0.035);
  color: var(--text);
}

.nav-item.active {
  background: rgba(135, 155, 219, 0.11);
  color: #a9b9f0;
  border-color: rgba(135, 155, 219, 0.12);
}

.nav-icon {
  width: 20px;
  font-size: 19px;
  text-align: center;
}

.settings-label {
  margin-top: 38px;
}

.sidebar-bottom {
  margin-top: auto;
}

.account-card {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 12px 8px;
  background: var(--surface);
  border: 1px solid var(--border);
  border-radius: 9px;
}

.account-avatar {
  width: 34px;
  height: 34px;
  display: grid;
  place-items: center;
  border-radius: 8px;
  background: var(--elevated);
  color: var(--accent);
  font-weight: 700;
}

.account-info {
  display: flex;
  flex-direction: column;
  gap: 4px;
  min-width: 0;
}

.account-info strong {
  font-size: 11px;
}

.account-info span {
  color: var(--muted);
  font-size: 9px;
}

.account-menu {
  margin-left: auto;
  color: var(--muted);
}

.sidebar-footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 18px 5px 0;
  color: #707789;
  font-size: 9px;
  letter-spacing: 1px;
}

.footer-dot {
  width: 6px;
  height: 6px;
  background: #80b99a;
  border-radius: 50%;
}

/* Main */

.main-content {
  flex: 1;
  min-width: 0;
}

.topbar {
  height: 67px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 43px;
  border-bottom: 1px solid var(--border);
}

.breadcrumb {
  display: flex;
  align-items: center;
  gap: 11px;
  font-size: 12px;
  color: var(--muted);
}

.breadcrumb strong {
  color: var(--text);
  font-weight: 600;
}

.breadcrumb-separator {
  color: #505767;
}

.topbar-right {
  display: flex;
  align-items: center;
  gap: 20px;
}

.system-status {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 11px;
  color: #aab2c1;
}

.system-dot {
  width: 7px;
  height: 7px;
  background: #80b99a;
  border-radius: 50%;
}

.version-label {
  padding: 5px 9px;
  background: var(--surface);
  border: 1px solid var(--border);
  border-radius: 5px;
  color: var(--muted);
  font-size: 10px;
}

.page-content {
  max-width: 1180px;
  margin: 0 auto;
  padding: 39px 43px 22px;
}

.welcome-section {
  position: relative;
  overflow: hidden;
  min-height: 158px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 36px;
  padding: 27px 31px;
  background: linear-gradient(110deg, #202633, #20242d 70%);
  border: 1px solid rgba(135, 155, 219, 0.12);
  border-radius: 12px;
}

.eyebrow {
  color: var(--accent);
  font-size: 9px;
  font-weight: 700;
  letter-spacing: 2px;
  margin-bottom: 12px;
}

.welcome-section h1,
.placeholder-page h1 {
  margin: 0;
  font-size: 27px;
  letter-spacing: -0.8px;
  font-weight: 650;
}

.welcome-section h1 span {
  color: var(--accent);
}

.welcome-section p,
.placeholder-page p {
  color: var(--muted);
  font-size: 12px;
  margin: 10px 0 0;
}

.welcome-decoration {
  position: absolute;
  right: 52px;
  top: -50px;
  color: rgba(135, 155, 219, 0.07);
  font-size: 230px;
  line-height: 1;
  font-weight: 900;
  pointer-events: none;
}

.section {
  margin-bottom: 34px;
}

.section-heading {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 15px;
}

.section-heading h2 {
  margin: 0;
  font-size: 15px;
  font-weight: 650;
}

.section-heading p {
  color: var(--muted);
  font-size: 11px;
  margin: 5px 0 0;
}

.section-tag,
.development-badge {
  color: var(--accent);
  font-size: 9px;
  letter-spacing: 1px;
  padding: 6px 9px;
  background: rgba(135, 155, 219, 0.09);
  border-radius: 5px;
}

/* Overview cards */

.overview-grid {
  display: grid;
  grid-template-columns: repeat(3, minmax(0, 1fr));
  gap: 13px;
}

.overview-card {
  min-height: 143px;
  padding: 16px;
  background: var(--surface);
  border: 1px solid var(--border);
  border-radius: 10px;
}

.card-icon {
  width: 30px;
  height: 30px;
  display: grid;
  place-items: center;
  margin-bottom: 13px;
  border-radius: 7px;
  font-size: 16px;
}

.card-icon.purple {
  background: rgba(135, 155, 219, 0.13);
  color: #a7b8f0;
}

.card-icon.blue {
  background: rgba(98, 160, 205, 0.13);
  color: #83b8df;
}

.card-icon.green {
  background: rgba(112, 180, 146, 0.13);
  color: #8cc6a7;
}

.card-label {
  color: var(--muted);
  font-size: 9px;
  letter-spacing: 1px;
}

.card-value {
  font-size: 23px;
  font-weight: 650;
  margin-top: 5px;
}

.card-description {
  color: #737d8e;
  font-size: 10px;
  margin-top: 4px;
}

/* Provider */

.provider-card {
  padding: 19px;
  background: var(--surface);
  border: 1px solid var(--border);
  border-radius: 10px;
}

.provider-main {
  display: flex;
  align-items: center;
  gap: 12px;
}

.provider-logo {
  width: 40px;
  height: 40px;
  display: grid;
  place-items: center;
  background: #303d50;
  border-radius: 9px;
  color: #b4c5e8;
  font-size: 20px;
  font-weight: 800;
}

.provider-info {
  display: flex;
  flex-direction: column;
  gap: 5px;
}

.provider-info strong {
  font-size: 13px;
}

.provider-info span {
  color: var(--muted);
  font-size: 10px;
}

.provider-details {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 15px;
  margin-top: 20px;
  padding-top: 16px;
  border-top: 1px solid var(--border);
}

.provider-status {
  display: flex;
  align-items: center;
  gap: 8px;
  color: #a4adbc;
  font-size: 11px;
}

.status-dot {
  width: 7px;
  height: 7px;
  border-radius: 50%;
  background: #687184;
}

.status-dot.checking {
  background: var(--accent);
  animation: pulse 1s infinite;
}

.status-dot.online {
  background: #80b99a;
}

.status-dot.offline {
  background: #d17d7d;
}

.check-button {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 9px;
  min-height: 35px;
  padding: 0 14px;
  background: rgba(135, 155, 219, 0.13);
  border: 1px solid rgba(135, 155, 219, 0.23);
  border-radius: 7px;
  color: #b3c1ee;
  font-size: 10px;
  cursor: pointer;
  transition: 0.2s ease;
}

.check-button:hover:not(:disabled) {
  background: rgba(135, 155, 219, 0.23);
}

.check-button:disabled {
  opacity: 0.65;
  cursor: wait;
}

.api-result {
  margin-top: 14px;
  padding: 11px 12px;
  border-radius: 6px;
  background: rgba(255, 255, 255, 0.025);
  font-size: 10px;
  overflow-wrap: anywhere;
}

.api-result.online {
  border-left: 2px solid #80b99a;
}

.api-result.offline {
  border-left: 2px solid #d17d7d;
}

.result-message {
  color: #c0c7d4;
}

.result-time {
  color: #737d8e;
  margin-top: 6px;
}

.spinner {
  width: 12px;
  height: 12px;
  border: 2px solid rgba(179, 193, 238, 0.3);
  border-top-color: #b3c1ee;
  border-radius: 50%;
  animation: spin 0.7s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

@keyframes pulse {
  50% { opacity: 0.35; }
}

/* Activity */

.empty-activity {
  min-height: 120px;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 8px;
  background: var(--surface);
  border: 1px solid var(--border);
  border-radius: 10px;
}

.empty-icon {
  color: #78839a;
  font-size: 23px;
}

.empty-activity strong {
  font-size: 11px;
  font-weight: 600;
}

.empty-activity span {
  color: var(--muted);
  font-size: 10px;
}

/* Placeholder pages */

.placeholder-page {
  min-height: 65vh;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  text-align: center;
}

.placeholder-icon {
  width: 62px;
  height: 62px;
  display: grid;
  place-items: center;
  margin-bottom: 24px;
  border-radius: 15px;
  background: var(--surface);
  color: var(--accent);
  font-size: 27px;
}

.placeholder-page .eyebrow {
  margin-bottom: 10px;
}

.development-badge {
  margin-top: 22px;
}

.page-footer {
  display: flex;
  justify-content: space-between;
  padding-top: 19px;
  border-top: 1px solid var(--border);
  color: #626b7c;
  font-size: 9px;
  letter-spacing: 0.6px;
}

/* Responsive */

@media (max-width: 900px) {
  .sidebar {
    width: 190px;
    min-width: 190px;
  }

  .topbar {
    padding: 0 25px;
  }

  .page-content {
    padding: 28px 25px;
  }

  .overview-grid {
    grid-template-columns: repeat(2, minmax(0, 1fr));
  }
}
</style>

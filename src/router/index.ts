
import { createRouter, createWebHistory } from "vue-router";

import HomePage from "../pages/HomePage.vue";

const routes = [
  {
    path: "/",
    name: "Home",
    component: HomePage,
  },
  {
    path: "/catalog",
    name: "Catalog",
    component: () => import("../pages/CatalogPage.vue"),
  },
  {
    path: "/instances",
    name: "Instances",
    component: () => import("../pages/InstancesPage.vue"),
  },
  {
    path: "/downloads",
    name: "Downloads",
    component: () => import("../pages/DownloadsPage.vue"),
  },
  {
    path: "/settings",
    name: "Settings",
    component: () => import("../pages/SettingsPage.vue"),
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;

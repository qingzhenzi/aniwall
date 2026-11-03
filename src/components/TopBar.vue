<script setup lang="ts">
import { computed } from "vue";
import { refreshCatalog, refreshEnv, refreshLibrary, state } from "../store";

const pillClass = computed(() => {
  const env = state.env;
  if (!env) return "pill";
  return "pill " + (env.plugin_ok && env.session_ok ? "ok" : "warn");
});

const pillText = computed(() => {
  const env = state.env;
  if (!env) return "检测中…";
  return `${env.distro_id} · ${env.session_type} · ${env.plasma_version || "无 KDE"}`;
});

async function refresh() {
  await refreshEnv();
  await refreshLibrary();
  await refreshCatalog();
}
</script>

<template>
  <header class="topbar">
    <div class="brand">
      <h1>AniWall</h1>
      <p>KDE 动态壁纸管理器</p>
    </div>
    <div class="topbar-right">
      <span :class="pillClass">{{ pillText }}</span>
      <button class="ghost" @click="refresh">刷新</button>
    </div>
  </header>
</template>

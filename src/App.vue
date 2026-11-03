<script setup lang="ts">
import { onMounted, ref } from "vue";
import { listen } from "@tauri-apps/api/event";
import TopBar from "./components/TopBar.vue";
import TabNav from "./components/TabNav.vue";
import LibraryView from "./components/LibraryView.vue";
import FetchView from "./components/FetchView.vue";
import HealthView from "./components/HealthView.vue";
import SettingsView from "./components/SettingsView.vue";
import AboutView from "./components/AboutView.vue";
import ToastHost from "./components/ToastHost.vue";
import { importPaths, init, state } from "./store";

const dragging = ref(false);

onMounted(async () => {
  await init();
  await listen("tauri://drag-enter", () => {
    dragging.value = true;
  });
  await listen("tauri://drag-leave", () => {
    dragging.value = false;
  });
  await listen<{ paths: string[] }>("tauri://drag-drop", (event) => {
    dragging.value = false;
    const paths = event.payload && event.payload.paths ? event.payload.paths : [];
    if (paths.length) void importPaths(paths);
  });
});
</script>

<template>
  <div class="shell" :class="{ dragging }">
    <TopBar />

    <div class="notice">
      ⚠️ 目前<strong>只在 CachyOS + KDE Plasma 6 (Wayland)</strong> 上测试过。其它发行版请先到
      「初始化 / 诊断」补齐依赖再使用。
    </div>

    <TabNav />

    <main>
      <LibraryView v-if="state.tab === 'library'" />
      <FetchView v-else-if="state.tab === 'fetch'" />
      <HealthView v-else-if="state.tab === 'health'" />
      <SettingsView v-else-if="state.tab === 'settings'" />
      <AboutView v-else />
    </main>

    <ToastHost />
  </div>
</template>

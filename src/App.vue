<script setup lang="ts">
import { onBeforeUnmount, onMounted, ref } from "vue";
import { listen } from "@tauri-apps/api/event";
import TopBar from "./components/TopBar.vue";
import TabNav from "./components/TabNav.vue";
import LibraryView from "./components/LibraryView.vue";
import FetchView from "./components/FetchView.vue";
import SettingsView from "./components/SettingsView.vue";
import HealthView from "./components/HealthView.vue";
import AboutView from "./components/AboutView.vue";
import ConfirmModal from "./components/ConfirmModal.vue";
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

onBeforeUnmount(() => {
  /* 事件监听随窗口销毁自动清理 */
});
</script>

<template>
  <div class="shell" :class="{ dragging }">
    <TopBar />
    <TabNav />

    <main>
      <LibraryView v-if="state.tab === 'library'" />
      <FetchView v-else-if="state.tab === 'fetch'" />
      <SettingsView v-else-if="state.tab === 'settings'" />
      <HealthView v-else-if="state.tab === 'health'" />
      <AboutView v-else />
    </main>

    <ConfirmModal />
    <ToastHost />
  </div>
</template>

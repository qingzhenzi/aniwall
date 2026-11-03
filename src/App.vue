<script setup lang="ts">
import { onBeforeUnmount, onMounted, ref } from "vue";
import { listen } from "@tauri-apps/api/event";
import TopBar from "./components/TopBar.vue";
import SectionNav from "./components/SectionNav.vue";
import LibraryView from "./components/LibraryView.vue";
import FetchView from "./components/FetchView.vue";
import SettingsView from "./components/SettingsView.vue";
import HealthView from "./components/HealthView.vue";
import AboutView from "./components/AboutView.vue";
import ToastHost from "./components/ToastHost.vue";
import { importPaths, init, state } from "./store";

const sections = [
  { id: "sec-library", label: "壁纸库" },
  { id: "sec-fetch", label: "在线下载" },
  { id: "sec-settings", label: "播放设置" },
  { id: "sec-health", label: "初始化 / 诊断" },
  { id: "sec-about", label: "关于" },
];

const activeSection = ref(sections[0].id);
const dragging = ref(false);
let scrollHandler: (() => void) | null = null;

function scrollTo(id: string) {
  const element = document.getElementById(id);
  if (!element) return;
  activeSection.value = id;
  element.scrollIntoView({ behavior: "smooth", block: "start" });
}

function syncActiveSection() {
  let current = sections[0].id;
  for (const section of sections) {
    const element = document.getElementById(section.id);
    if (!element) continue;
    if (element.getBoundingClientRect().top <= 140) {
      current = section.id;
    }
  }
  activeSection.value = current;
}

onMounted(async () => {
  await init();

  scrollHandler = () => syncActiveSection();
  window.addEventListener("scroll", scrollHandler, { passive: true });

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
  if (scrollHandler) window.removeEventListener("scroll", scrollHandler);
});
</script>

<template>
  <div class="shell" :class="{ dragging }">
    <TopBar />

    <div class="notice">
      只在 <strong>CachyOS + KDE Plasma 6 (Wayland)</strong> 上测试过 · 关闭窗口会最小化到托盘，托盘菜单可退出
    </div>

    <SectionNav :sections="sections" :active="activeSection" @select="scrollTo" />

    <main>
      <LibraryView id="sec-library" />
      <FetchView id="sec-fetch" />
      <SettingsView id="sec-settings" />
      <HealthView id="sec-health" />
      <AboutView id="sec-about" />
    </main>

    <ToastHost />
  </div>
</template>

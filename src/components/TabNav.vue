<script setup lang="ts">
import { state } from "../store";
import type { TabKey } from "../types";

const tabs: Array<{ key: TabKey; label: string }> = [
  { key: "library", label: "壁纸库" },
  { key: "fetch", label: "在线下载" },
  { key: "settings", label: "播放设置" },
  { key: "health", label: "初始化 / 诊断" },
  { key: "about", label: "关于" },
];
</script>

<template>
  <nav class="tabs">
    <button
      v-for="tab in tabs"
      :key="tab.key"
      class="tab"
      :class="{ active: state.tab === tab.key }"
      @click="state.tab = tab.key"
    >
      {{ tab.label }}
    </button>
  </nav>
</template>

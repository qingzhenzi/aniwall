<script setup lang="ts">
import { state } from "../store";
import type { TabKey } from "../types";

const tabs: Array<{ key: TabKey; label: string }> = [
  { key: "library", label: "壁纸库" },
  { key: "fetch", label: "获取壁纸" },
  { key: "health", label: "初始化 / 诊断" },
  { key: "settings", label: "设置" },
  { key: "about", label: "关于" },
];
</script>

<template>
  <nav class="tabs">
    <button
      v-for="tab in tabs"
      :key="tab.key"
      :class="{ active: state.tab === tab.key }"
      @click="state.tab = tab.key"
    >
      {{ tab.label }}
    </button>
  </nav>
</template>

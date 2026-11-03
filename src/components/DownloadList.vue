<script setup lang="ts">
import type { DownloadItem } from "../types";

defineProps<{ downloads: DownloadItem[] }>();

function clamp(value: number) {
  return Math.max(0, Math.min(100, value || 0));
}

function statusText(item: DownloadItem) {
  if (item.status === "done") return "✓ 完成";
  if (item.status === "error") return "✕ 失败";
  if (item.status === "pending") return "等待中";
  return clamp(item.percent).toFixed(1) + "%";
}
</script>

<template>
  <div class="download-list">
    <p v-if="!downloads.length" class="muted">暂无下载任务。</p>
    <div
      v-for="(item, index) in downloads"
      :key="index"
      class="dl"
      :class="item.status === 'error' ? 'error' : item.status === 'done' ? 'done' : ''"
    >
      <div class="top">
        <span class="url" :title="item.url">{{ item.url }}</span>
        <span>{{ statusText(item) }}</span>
      </div>
      <div class="bar"><span :style="{ width: clamp(item.percent) + '%' }" /></div>
      <div class="muted" style="font-size: 11.5px; margin-top: 4px">{{ item.message }}</div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { DownloadItem } from "../types";
import { removeDownload } from "../store";

defineProps<{ downloads: DownloadItem[] }>();

function clamp(value: number) {
  return Math.max(0, Math.min(100, value || 0));
}

function running(item: DownloadItem) {
  return item.status === "pending" || item.status === "start" || item.status === "downloading";
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
      v-for="item in downloads"
      :key="item.id"
      class="dl"
      :class="item.status === 'error' ? 'error' : item.status === 'done' ? 'done' : ''"
    >
      <div class="top">
        <span class="url" :title="item.url">{{ item.title || item.url }}</span>
        <span>{{ statusText(item) }}</span>
      </div>
      <div class="bar"><span :style="{ width: clamp(item.percent) + '%' }" /></div>
      <div class="dl-bottom">
        <span class="muted dl-message" :title="item.message">{{ item.message }}</span>
        <button v-if="!running(item)" class="mini ghost" @click="removeDownload(item.id)">移除</button>
      </div>
    </div>
  </div>
</template>

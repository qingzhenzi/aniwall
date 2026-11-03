<script setup lang="ts">
import CatalogCard from "./CatalogCard.vue";
import DownloadList from "./DownloadList.vue";
import { computed, ref } from "vue";
import { downloadUrls, state } from "../store";

const selected = ref<string[]>([]);
const customUrl = ref("");

const selectedCount = computed(() => selected.value.length);

function toggle(bvid: string) {
  selected.value = selected.value.includes(bvid)
    ? selected.value.filter((item) => item !== bvid)
    : [...selected.value, bvid];
}

function isDownloaded(bvid: string) {
  return state.library.some((item) => (item.source || "").includes(bvid));
}

function bvidToUrl(bvid: string) {
  return `https://www.bilibili.com/video/${bvid}`;
}

function downloadSelected() {
  if (!selected.value.length) return;
  void downloadUrls(selected.value.map(bvidToUrl));
  selected.value = [];
}

function downloadAll() {
  const urls = state.catalog.map((item) => bvidToUrl(item.bvid));
  if (urls.length > 6 && !confirm(`确定下载全部 ${urls.length} 个壁纸？会占用一些时间和磁盘空间。`)) {
    return;
  }
  void downloadUrls(urls);
}

function downloadCustom() {
  const value = customUrl.value.trim();
  if (!value) return;
  void downloadUrls([value]);
  customUrl.value = "";
}
</script>

<template>
  <section>
    <div class="section-head">
      <div>
        <h2>内置清单</h2>
        <p>来自 B 站（1080p / 30fps，体积小、解码轻）。下载后自动生成封面并入库。</p>
      </div>
      <div class="actions">
        <button class="primary" :disabled="!selectedCount" @click="downloadSelected">
          下载选中{{ selectedCount ? `（${selectedCount}）` : "" }}
        </button>
        <button class="ghost" @click="downloadAll">全部下载</button>
      </div>
    </div>

    <div class="grid small">
      <CatalogCard
        v-for="item in state.catalog"
        :key="item.id"
        :item="item"
        :downloaded="isDownloaded(item.bvid)"
        :checked="selected.includes(item.bvid)"
        @toggle="toggle(item.bvid)"
        @download="downloadUrls([bvidToUrl(item.bvid)])"
      />
    </div>

    <div class="divider" />

    <div class="section-head">
      <div>
        <h2>自定义地址</h2>
        <p>支持 B 站视频链接、BV 号，或直链 mp4 / webm。</p>
      </div>
    </div>
    <div class="url-row">
      <input
        v-model="customUrl"
        type="text"
        placeholder="https://www.bilibili.com/video/BV… 或 BV… / 或 https://…/a.mp4"
        @keyup.enter="downloadCustom"
      />
      <button class="primary" @click="downloadCustom">下载</button>
    </div>

    <div class="divider" />
    <h2>下载进度</h2>
    <DownloadList :downloads="state.downloads" />
  </section>
</template>

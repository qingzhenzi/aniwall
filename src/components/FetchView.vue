<script setup lang="ts">
import { computed, ref } from "vue";
import CatalogCard from "./CatalogCard.vue";
import DownloadList from "./DownloadList.vue";
import {
  clearDownloads,
  confirmDialog,
  downloadUrls,
  state,
} from "../store";

const selected = ref<string[]>([]);
const customUrl = ref("");

const selectedCount = computed(() => selected.value.length);
const allSelected = computed(
  () => state.catalog.length > 0 && selected.value.length === state.catalog.length
);
const downloading = computed(() =>
  state.downloads.some((item) => item.status === "start" || item.status === "downloading")
);

function toggle(bvid: string) {
  selected.value = selected.value.includes(bvid)
    ? selected.value.filter((item) => item !== bvid)
    : [...selected.value, bvid];
}

function toggleAll() {
  selected.value = allSelected.value ? [] : state.catalog.map((item) => item.bvid);
}

function isDownloaded(bvid: string) {
  return state.library.some((item) => (item.source || "").includes(bvid));
}

function bvidToUrl(bvid: string) {
  return `https://www.bilibili.com/video/${bvid}`;
}

async function downloadSelected() {
  if (!selected.value.length) return;
  const urls = selected.value.map(bvidToUrl);
  selected.value = [];
  await downloadUrls(urls);
}

async function downloadAll() {
  const urls = state.catalog.map((item) => bvidToUrl(item.bvid));
  const ok = await confirmDialog(`确定下载全部 ${urls.length} 个壁纸？会占用一些时间和磁盘空间。`, {
    title: "全部下载",
    okText: "开始下载",
  });
  if (ok) await downloadUrls(urls);
}

async function downloadCustom() {
  const value = customUrl.value.trim();
  if (!value) return;
  customUrl.value = "";
  await downloadUrls([value]);
}

function clear() {
  clearDownloads();
}
</script>

<template>
  <section>
    <div class="section-head">
      <div>
        <h2>在线下载</h2>
        <p>内置清单来自 B 站（1080p / 30fps，体积小、解码轻），下载后自动生成封面并入库。</p>
      </div>
      <div class="actions">
        <button class="ghost" @click="toggleAll">{{ allSelected ? "取消全选" : "全选" }}</button>
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

    <div class="section-head">
      <div>
        <h2>下载进度</h2>
        <p v-if="downloading">下载进行中，可以继续操作其它页面。</p>
        <p v-else>最近的任务记录。</p>
      </div>
      <div class="actions">
        <button class="ghost" :disabled="!state.downloads.length || downloading" @click="clear">
          清空记录
        </button>
      </div>
    </div>
    <DownloadList :downloads="state.downloads" />
  </section>
</template>

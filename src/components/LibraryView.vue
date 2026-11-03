<script setup lang="ts">
import { computed, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import WallpaperCard from "./WallpaperCard.vue";
import {
  applyWallpaper,
  deleteWallpaper,
  importPaths,
  restoreWallpaper,
  state,
} from "../store";
import { fmtDuration, fmtResolution } from "../format";

const importPath = ref("");
const importInput = ref<HTMLInputElement | null>(null);

const activeItem = computed(
  () => state.library.find((item) => item.id === state.activeId) || null
);

const metaText = computed(() =>
  state.library.length
    ? `${state.library.length} 个壁纸 · ${state.env?.wallpaper_dir ?? "~/.local/share/aniwall/wallpapers"}`
    : "还没有壁纸"
);

async function importOne() {
  const value = importPath.value.trim();
  if (!value) return;
  await importPaths([value]);
  importPath.value = "";
}

async function remove(id: string) {
  if (!confirm("确定从壁纸库删除这个壁纸？（会删除本地文件）")) return;
  await deleteWallpaper(id);
}

async function openFolder() {
  try {
    await invoke("open_folder");
  } catch {
    /* 忽略 */
  }
}

function focusImport() {
  importInput.value?.focus();
}
</script>

<template>
  <section>
    <div class="section-head">
      <div>
        <h2>壁纸库</h2>
        <p>{{ metaText }}</p>
      </div>
      <div class="actions">
        <button class="ghost" @click="focusImport">导入本地视频…</button>
        <button class="ghost" @click="openFolder">打开壁纸目录</button>
        <button class="warn" @click="restoreWallpaper">恢复静态壁纸</button>
      </div>
    </div>

    <div v-if="activeItem" class="current-bar">
      <span class="dot" />
      当前桌面壁纸：<strong>{{ activeItem.title }}</strong>
      <span class="muted">
        {{ fmtResolution(activeItem.width, activeItem.height) }}
        {{ fmtDuration(activeItem.duration_secs) }}
      </span>
    </div>

    <div class="grid">
      <WallpaperCard
        v-for="item in state.library"
        :key="item.id"
        :item="item"
        :active="item.id === state.activeId"
        @apply="applyWallpaper(item.id)"
        @remove="remove(item.id)"
      />
    </div>

    <div v-if="!state.library.length" class="empty">
      <p>壁纸库还是空的。</p>
      <p>去「获取壁纸」下载，或者直接把视频文件拖进这个窗口。</p>
    </div>

    <div class="import-row">
      <input
        ref="importInput"
        v-model="importPath"
        type="text"
        placeholder="或输入本地视频路径，例如 /home/you/Videos/a.mp4"
        @keyup.enter="importOne"
      />
      <button class="ghost" @click="importOne">导入</button>
    </div>
    <p class="drop-hint">提示：把 mp4 / webm 文件直接拖进窗口也可以导入。</p>
  </section>
</template>

<script setup lang="ts">
import { computed, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import WallpaperCard from "./WallpaperCard.vue";
import {
  applyWallpaper,
  confirmDialog,
  deleteWallpaper,
  importPaths,
  refreshLibrary,
  restoreWallpaper,
  state,
} from "../store";
import { fmtDuration, fmtResolution } from "../format";

const keyword = ref("");

const filtered = computed(() => {
  const kw = keyword.value.trim().toLowerCase();
  const items = state.library
    .slice()
    .sort((a, b) => (b.added_at || 0) - (a.added_at || 0));
  if (!kw) return items;
  return items.filter(
    (item) =>
      item.title.toLowerCase().includes(kw) ||
      (item.source || "").toLowerCase().includes(kw)
  );
});

const activeItem = computed(
  () => state.library.find((item) => item.id === state.activeId) || null
);

const metaText = computed(() =>
  state.library.length
    ? `${state.library.length} 个壁纸 · ${state.env?.wallpaper_dir ?? "~/.local/share/aniwall/wallpapers"}`
    : "还没有壁纸"
);

async function pickFiles() {
  try {
    const selected = await open({
      multiple: true,
      directory: false,
      filters: [{ name: "视频文件", extensions: ["mp4", "webm", "mkv", "mov", "m4v"] }],
    });
    const paths = Array.isArray(selected) ? selected : selected ? [selected] : [];
    if (paths.length) await importPaths(paths as string[]);
  } catch (error) {
    console.error(error);
  }
}

async function remove(item: { id: string; title: string }) {
  const ok = await confirmDialog(`确定删除《${item.title}》吗？本地视频文件也会一起删除。`, {
    title: "删除壁纸",
    okText: "删除",
  });
  if (ok) await deleteWallpaper(item.id);
}

async function restore() {
  const ok = await confirmDialog("恢复成静态壁纸？当前动态壁纸会停止。", {
    title: "恢复静态壁纸",
    okText: "恢复",
  });
  if (ok) await restoreWallpaper();
}

async function openFolder() {
  try {
    await invoke("open_folder");
  } catch {
    /* 忽略 */
  }
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
        <button class="primary" @click="pickFiles">导入本地视频…</button>
        <button class="ghost" @click="openFolder">打开壁纸目录</button>
        <button class="ghost" @click="refreshLibrary">刷新</button>
        <button class="warn" @click="restore">恢复静态壁纸</button>
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

    <div v-if="state.library.length" class="toolbar">
      <input v-model="keyword" type="text" class="search" placeholder="搜索壁纸标题 / 来源…" />
      <span class="muted">共 {{ state.library.length }} 张，显示 {{ filtered.length }} 张</span>
    </div>

    <div class="grid">
      <WallpaperCard
        v-for="item in filtered"
        :key="item.id"
        :item="item"
        :active="item.id === state.activeId"
        @apply="applyWallpaper(item.id)"
        @remove="remove(item)"
      />
    </div>

    <div v-if="!state.library.length" class="empty">
      <p>壁纸库还是空的</p>
      <p class="muted">点上面的「导入本地视频…」，或者去「在线下载」拉清单里的壁纸</p>
      <p class="muted">也可以把 mp4 / webm 文件直接拖进这个窗口</p>
    </div>

    <div v-else-if="!filtered.length" class="empty">
      <p>没有匹配「{{ keyword }}」的壁纸</p>
    </div>
  </section>
</template>

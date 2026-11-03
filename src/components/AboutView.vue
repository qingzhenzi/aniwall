<script setup lang="ts">
import { computed } from "vue";
import { state } from "../store";

const dataDir = computed(
  () => state.env?.wallpaper_dir ?? "~/.local/share/aniwall/wallpapers"
);
</script>

<template>
  <section>
    <h2>关于 AniWall</h2>
    <p>
      AniWall 是一个为 KDE Plasma 6 写的动态壁纸管理器：从 B 站等站点下载视频，自动生成封面、维护壁纸库，
      并调用
      <a href="https://github.com/luisbocanegra/plasma-smart-video-wallpaper-reborn" target="_blank" rel="noreferrer">
        Smart Video Wallpaper Reborn
      </a>
      插件一键切换桌面壁纸。前端使用 Vue 3 + TypeScript + Vite，后端使用 Tauri（Rust）。
    </p>

    <h3>兼容性</h3>
    <ul>
      <li>开发与测试环境：<strong>CachyOS</strong>（Arch 系）+ KDE Plasma 6.7 + Wayland。</li>
      <li>其它 KDE Plasma 6 发行版理论上可用，但未测试；非 KDE 桌面（GNOME / Hyprland）不支持。</li>
      <li>Wallpaper Engine 没有 Linux 原生版，本项目用「视频壁纸」方案替代。</li>
    </ul>

    <h3>数据位置</h3>
    <ul>
      <li>壁纸库：<code>{{ dataDir }}</code></li>
      <li>设置：<code>~/.config/aniwall/settings.json</code></li>
    </ul>

    <h3>已知问题</h3>
    <ul>
      <li>锁屏 / 桌面不可见时插件会自动暂停，属正常现象。</li>
      <li>4K 素材解码开销明显更高，建议用 1080p。</li>
      <li>B 站部分视频需要登录 cookie 才有 1080p。</li>
    </ul>

    <p class="muted">
      MIT License · 仅用于个人学习与壁纸管理，请遵守素材来源站点的使用条款。
    </p>
  </section>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { state } from "../store";

const info = computed(() => state.appInfo);
const dataDir = computed(
  () => state.env?.wallpaper_dir ?? "~/.local/share/aniwall/wallpapers"
);
</script>

<template>
  <section>
    <h2>关于 AniWall</h2>
    <p>
      为 KDE Plasma 6 写的动态壁纸管理器：从 B 站下载视频、自动生成封面、维护壁纸库，
      并调用
      <a href="https://github.com/luisbocanegra/plasma-smart-video-wallpaper-reborn" target="_blank" rel="noreferrer">
        Smart Video Wallpaper Reborn
      </a>
      一键切换桌面壁纸。前端 Vue 3 + TypeScript，后端 Tauri 2（Rust），支持托盘与开机自启。
    </p>

    <div class="form" style="margin: 14px 0 18px">
      <div class="field"><span>版本</span><b>{{ info?.version || "—" }}</b></div>
      <div class="field"><span>托盘</span><b>{{ info?.tray ? "已就绪" : "不可用" }}</b></div>
      <div class="field">
        <span>开机自启</span>
        <b>{{ state.autostart ? "已开启（静默到托盘）" : "未开启" }}</b>
      </div>
      <div class="field">
        <span>测试环境</span>
        <b>{{ info?.tested_on || "CachyOS / KDE Plasma 6 / Wayland" }}</b>
      </div>
      <div class="field wide">
        <span>可执行文件</span>
        <code>{{ info?.exe || "—" }}</code>
      </div>
      <div class="field wide">
        <span>壁纸库</span>
        <code>{{ dataDir }}</code>
      </div>
    </div>

    <h3>兼容性</h3>
    <ul>
      <li>开发与测试环境：<strong>CachyOS</strong>（Arch 系）+ KDE Plasma 6.7 + Wayland。</li>
      <li>其它 KDE Plasma 6 发行版理论可用，但未测试；GNOME / Hyprland 等非 KDE 桌面不支持。</li>
      <li>Wallpaper Engine 没有 Linux 原生版，本项目用「视频壁纸」方案替代。</li>
    </ul>

    <h3>数据位置</h3>
    <ul>
      <li>壁纸库：<code>{{ dataDir }}</code></li>
      <li>设置：<code>~/.config/aniwall/settings.json</code></li>
      <li>开机自启：<code>~/.config/autostart/aniwall.desktop</code></li>
    </ul>

    <h3>已知问题</h3>
    <ul>
      <li>锁屏 / 桌面不可见时插件会自动暂停，属正常现象。</li>
      <li>4K 素材解码开销明显更高，建议用 1080p。</li>
      <li>B 站部分视频需要登录 cookie 才有 1080p。</li>
    </ul>

    <p class="muted">MIT License · 仅用于个人学习与壁纸管理，请遵守素材来源站点的使用条款。</p>
  </section>
</template>

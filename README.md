# AniWall 🖥️🎞️

**KDE Plasma 6 动态壁纸管理器**：从 B 站等站点下载视频壁纸，自动生成封面、维护壁纸库，然后一键应用到桌面。

> 前端 **Vue 3 + TypeScript + Vite**，后端 **Tauri 2（Rust）**，壁纸渲染走
> [Smart Video Wallpaper Reborn](https://github.com/luisbocanegra/plasma-smart-video-wallpaper-reborn) 插件。

---

## ⚠️ 测试范围

**只在 CachyOS（Arch 系）+ KDE Plasma 6.7 + Wayland 上测试过。**
其它 KDE Plasma 6 发行版理论可用但未验证；GNOME / Hyprland 等非 KDE 桌面不支持。

---

## 功能

- **壁纸库**：导入本地视频（拖拽或填路径）、自动生成封面缩略图、显示分辨率 / 时长 / 体积、一键应用、删除。
- **下载**：内置 B 站清单（21 个二次元动态壁纸）+ 自定义地址（B 站链接、BV 号、直链 mp4/webm）。
  下载后自动 `ffprobe` 解析元数据、`ffmpeg` 抽封面并入库。
- **一键切换**：通过 `qdbus6` 调用 PlasmaShell 脚本，把视频写入
  `Smart Video Wallpaper Reborn` 的配置，所有屏幕 / 桌面同时生效。
- **播放设置**：裁剪填充 / 静音 / 最大化时暂停 / 电量阈值暂停 / 循环播放。
- **初始化与自诊**：检测发行版、会话类型、Plasma 版本、插件与依赖（yt-dlp / ffmpeg / qdbus6 / curl / bsdtar），
  可一键安装插件、用 `pkexec pacman` 补齐依赖、创建桌面快捷方式。
- **CLI**：`aniwall --check`（环境诊断 JSON）、`--list`（列出壁纸库）、
  `--set <id|文件路径>`（直接切换壁纸）、`--restore`（恢复静态壁纸）。

---

## 系统要求

| 依赖 | 用途 | 安装 |
| --- | --- | --- |
| KDE Plasma 6 + Wayland | 运行环境 | — |
| Smart Video Wallpaper Reborn | 视频壁纸插件 | 应用内「初始化 / 诊断」一键安装 |
| `yt-dlp` | 下载 B 站等站点视频 | `sudo pacman -S yt-dlp` |
| `ffmpeg` / `ffprobe` | 封面 / 元数据 | `sudo pacman -S ffmpeg` |
| `qdbus6` | 切换壁纸 | `pacman -S qt6-tools`（KDE 环境一般已有） |
| `curl` / `bsdtar` | 下载、解压插件 | `sudo pacman -S curl libarchive` |

`pkexec` 用于在图形界面里弹管理员授权安装依赖（KDE 自带 polkit 即可）。

---

## 安装

### 方式一：AppImage（推荐，单文件）

```bash
chmod +x AniWall_0.1.0_amd64.AppImage
./AniWall_0.1.0_amd64.AppImage
```

AppImage 仍依赖系统里的 WebKitGTK 4.1（Arch 系默认已装）。

### 方式二：Arch / CachyOS 包（PKGBUILD）

```bash
git clone <repo> aniwall && cd aniwall
makepkg -si
```

### 方式三：从源码构建

```bash
# 依赖
sudo pacman -S --needed base-devel webkit2gtk-4.1 libsoup3 gtk3 librsvg \
  yt-dlp ffmpeg curl libarchive nodejs pnpm

pnpm install
cargo tauri build          # 产物：src-tauri/target/release/bundle/
```

开发调试：

```bash
pnpm tauri dev
```

### 方式四：本地直接打 pacman 包（无需联网拉 linuxdeploy）

```bash
./scripts/package-local.sh
# 产物：dist-pkg/aniwall-0.1.0-1-x86_64.pkg.tar.zst
sudo pacman -U dist-pkg/aniwall-0.1.0-1-x86_64.pkg.tar.zst
```

### 方式五：AppImage 的说明

`cargo tauri build --bundles appimage` 需要从 GitHub 拉取 `linuxdeploy` 等打包工具，
国内网络可能很慢或卡住；CI（GitHub Actions）里会正常产出 AppImage。
本地更推荐上面的 pacman 包或 `scripts/install.sh`。

---

## 数据位置

| 内容 | 路径 |
| --- | --- |
| 壁纸库 | `~/.local/share/aniwall/wallpapers/` |
| 封面 | `~/.local/share/aniwall/posters/` |
| 库索引 | `~/.local/share/aniwall/library.json` |
| 设置 | `~/.config/aniwall/settings.json` |

桌面快捷方式由应用内「创建桌面快捷方式」生成（`~/.local/share/applications/aniwall.desktop`），
也可以从应用菜单搜索「AniWall」启动；`scripts/install.sh` 与 `scripts/package-local.sh` 会在桌面放一份启动器。

---

## 常见问题

**Q：桌面是黑的 / 没有动起来？**
A：确认 `Smart Video Wallpaper Reborn` 已安装且在系统设置 → 壁纸里选中过；锁屏或桌面被遮挡时插件会自动暂停（正常）。
也可执行 `aniwall --check` 看环境是否齐全。

**Q：B 站视频只有 720p？**
A：游客 Cookie 一般能拿到 1080p；部分视频需要登录。可在「设置」里填一个 Netscape 格式的 cookies 文件路径，
下载时会传给 `yt-dlp --cookies`。

**Q：资源占用？**
A：1080p30 视频约占用单核 10~15%（整机 1% 左右）、GPU 2~5%、功耗 +1.5~2W；
4K 素材会明显更重。壁纸在「有最大化/全屏窗口」或「电量低于阈值」时会自动暂停，可自行在设置里调整。

**Q：为什么没有 musl 静态包？**
A：Tauri 依赖 WebKitGTK / GTK / libsoup，这些是 glibc 动态库，musl 静态链接在桌面 GUI 上不可行。
本项目提供 **AppImage（单文件）** 与 **Arch pacman 包** 两种等价形态；CI 里的 `x86_64-unknown-linux-gnu`
是唯一受支持的 Linux 目标。

---

## 仓库结构

```
.
├── src/                  # Vue 3 + TS 前端
│   ├── components/       # TopBar / Library / Fetch / Health / Settings / About
│   ├── store.ts          # 全局状态 + invoke 封装
│   └── types.ts          # 与 Rust 端对齐的类型
├── src-tauri/            # Tauri (Rust) 后端
│   └── src/
│       ├── lib.rs        # 命令注册
│       ├── plasma.rs     # 壁纸切换（PlasmaShell evaluateScript）
│       ├── library.rs    # 壁纸库 / 元数据 / 封面
│       ├── download.rs   # yt-dlp / curl 下载
│       ├── system.rs     # 环境检测、插件/依赖安装
│       └── catalog.rs    # 内置清单
├── catalog.json          # 内置壁纸清单（BVID / 封面 / 标签）
├── packaging/aniwall.desktop
├── scripts/
│   ├── install.sh          # 构建 AppImage 并安装到 ~/.local
│   ├── package-local.sh    # 用 release 二进制打 pacman 包
│   └── download-catalog.sh # 命令行批量下载内置清单
├── PKGBUILD
├── .github/workflows/    # GitHub Actions
└── .gitea/workflows/     # Gitea Actions
```

---

## CI

- **GitHub Actions**（`.github/workflows/build.yml`）：push / PR 时跑 `pnpm build` + `cargo tauri build`，
  产出 AppImage 与 deb 工件；打 `v*` tag 时自动创建 Release。
- **Gitea Actions**（`.gitea/workflows/build.yml`）：同上，使用 Arch 容器构建，额外产出 `.pkg.tar.zst`。

---

## 致谢与声明

- 壁纸渲染插件：[luisbocanegra/plasma-smart-video-wallpaper-reborn](https://github.com/luisbocanegra/plasma-smart-video-wallpaper-reborn)（GPLv2+）
- 内置清单里的视频均来自 B 站公开投稿，版权归原作者所有；本项目仅做本地下载 / 播放管理，
  请遵守来源站点条款，勿用于商业用途。
- 应用图标为纯色占位图（无设计元素），可自行替换 `design/icon.png` 后执行 `cargo tauri icon design/icon.png`。

MIT License。

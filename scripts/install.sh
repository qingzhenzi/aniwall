#!/usr/bin/env bash
# 本地构建 + 安装（AppImage 单文件方案）
# 用法：./scripts/install.sh
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

echo "==> 安装前端依赖"
pnpm install --frozen-lockfile

echo "==> 构建 AppImage"
cargo tauri build --bundles appimage

APPIMAGE="$(ls -1 src-tauri/target/release/bundle/appimage/*.AppImage | head -n1)"
if [ -z "$APPIMAGE" ]; then
  echo "没有找到 AppImage 产物" >&2
  exit 1
fi

BIN_DIR="${XDG_BIN_HOME:-$HOME/.local/bin}"
APPS_DIR="${XDG_DATA_HOME:-$HOME/.local/share}/applications"
ICON_DIR="${XDG_DATA_HOME:-$HOME/.local/share}/icons/hicolor/256x256/apps"
mkdir -p "$BIN_DIR" "$APPS_DIR" "$ICON_DIR"

install -m755 "$APPIMAGE" "$BIN_DIR/aniwall.AppImage"
install -m644 src-tauri/icons/128x128@2x.png "$ICON_DIR/aniwall.png"

cat > "$APPS_DIR/aniwall.desktop" <<EOF
[Desktop Entry]
Type=Application
Name=AniWall
Name[zh_CN]=AniWall 动态壁纸
Comment=KDE dynamic wallpaper manager
Comment[zh_CN]=KDE 动态壁纸管理器（下载 / 管理 / 一键切换）
Exec=${BIN_DIR}/aniwall.AppImage
Icon=aniwall
Terminal=false
Categories=Utility;Settings;
Keywords=wallpaper;video;anime;kde;plasma;
EOF

update-desktop-database "$APPS_DIR" 2>/dev/null || true

echo
echo "✅ 已安装："
echo "   可执行文件: ${BIN_DIR}/aniwall.AppImage"
echo "   桌面入口:   ${APPS_DIR}/aniwall.desktop"
echo "   现在可以在应用菜单里搜索 AniWall。"

#!/usr/bin/env bash
# 用已编译好的 release 二进制直接打一个 Arch 包（无需 makepkg / 网络）。
# 产物：dist-pkg/aniwall-<version>-1-x86_64.pkg.tar.zst
# 安装：sudo pacman -U dist-pkg/aniwall-*-x86_64.pkg.tar.zst
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

VERSION="$(python3 -c "import json;print(json.load(open('package.json'))['version'])")"
OUT="$ROOT/dist-pkg"
STAGE="$(mktemp -d)"
trap 'rm -rf "$STAGE"' EXIT

cargo build --release --manifest-path src-tauri/Cargo.toml

mkdir -p "$STAGE/usr/bin" \
         "$STAGE/usr/share/applications" \
         "$STAGE/usr/share/icons/hicolor/256x256/apps" \
         "$STAGE/usr/share/licenses/aniwall"

install -m755 src-tauri/target/release/aniwall "$STAGE/usr/bin/aniwall"
install -m644 packaging/aniwall.desktop "$STAGE/usr/share/applications/aniwall.desktop"
install -m644 src-tauri/icons/128x128@2x.png "$STAGE/usr/share/icons/hicolor/256x256/apps/aniwall.png"
install -m644 LICENSE "$STAGE/usr/share/licenses/aniwall/LICENSE"

SIZE="$(du -sb "$STAGE" | cut -f1)"
BUILDDATE="$(date +%s)"

cat > "$STAGE/.PKGINFO" <<EOF
pkgname = aniwall
pkgbase = aniwall
pkgver = ${VERSION}-1
pkgdesc = KDE Plasma 6 动态壁纸管理器：下载 / 管理 / 一键切换视频壁纸
url = https://github.com/aniwall/aniwall
builddate = ${BUILDDATE}
packager = AniWall contributors
size = ${SIZE}
arch = x86_64
license = MIT
depend = webkit2gtk-4.1
depend = gtk3
depend = librsvg
depend = glibc
depend = yt-dlp
depend = ffmpeg
depend = curl
depend = libarchive
depend = qt6-tools
depend = polkit
EOF

mkdir -p "$OUT"
PKG="$OUT/aniwall-${VERSION}-1-x86_64.pkg.tar.zst"
( cd "$STAGE" && bsdtar --zstd -cf "$PKG" .PKGINFO usr )

echo
echo "✅ 打包完成：$PKG"
echo "   安装：sudo pacman -U $PKG"

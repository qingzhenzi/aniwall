# Maintainer: AniWall contributors <aniwall@example.com>
pkgname=aniwall
pkgver=0.1.0
pkgrel=1
pkgdesc="KDE Plasma 6 动态壁纸管理器：下载 / 管理 / 一键切换视频壁纸"
arch=('x86_64')
url="https://github.com/qingzhenzi/aniwall"
license=('MIT')
depends=(
  'webkit2gtk-4.1'
  'gtk3'
  'librsvg'
  'glibc'
  'yt-dlp'
  'ffmpeg'
  'curl'
  'libarchive'
  'qt6-tools'
  'polkit'
  'libayatana-appindicator'
)
makedepends=('cargo' 'nodejs' 'pnpm' 'base-devel' 'libsoup3' 'libayatana-appindicator')
source=("$pkgname-$pkgver.tar.gz::$url/archive/refs/tags/v$pkgver.tar.gz")
sha256sums=('SKIP')

prepare() {
  cd "$pkgname-$pkgver"
  pnpm install --frozen-lockfile
}

build() {
  cd "$pkgname-$pkgver"
  pnpm build
  cargo build --release --features custom-protocol --manifest-path src-tauri/Cargo.toml
}

package() {
  cd "$pkgname-$pkgver"

  install -Dm755 src-tauri/target/release/aniwall "$pkgdir/usr/bin/aniwall"

  install -Dm644 packaging/aniwall.desktop "$pkgdir/usr/share/applications/aniwall.desktop"
  install -Dm644 src-tauri/icons/128x128@2x.png \
    "$pkgdir/usr/share/icons/hicolor/256x256/apps/aniwall.png"

  install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}

# 说明：
# 1. 本项目构建需要联网拉取 crates 与 npm 依赖（上游未做 vendor）。
# 2. 当前只支持 x86_64 与 KDE Plasma 6 + Wayland。
# 3. 若只想快速体验，推荐直接用 AppImage，无需本 PKGBUILD。

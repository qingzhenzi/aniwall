#!/usr/bin/env bash
# 把 catalog.json 里的内置清单批量下载到本地壁纸库。
# 用法：
#   ./scripts/download-catalog.sh                # 全部
#   ./scripts/download-catalog.sh BV1xx BV1yy    # 指定 BVID
#   COOKIES=/path/to/cookies.txt ./scripts/download-catalog.sh
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DEST="${XDG_DATA_HOME:-$HOME/.local/share}/aniwall/wallpapers"
mkdir -p "$DEST"

if ! command -v yt-dlp >/dev/null 2>&1; then
  echo "缺少 yt-dlp，请先安装：sudo pacman -S yt-dlp" >&2
  exit 1
fi

if [ "$#" -gt 0 ]; then
  BVIDS=("$@")
else
  mapfile -t BVIDS < <(python3 -c "
import json, sys
for item in json.load(open('$ROOT/catalog.json')):
    print(item['bvid'])
")
fi

COOKIE_ARGS=()
if [ -n "${COOKIES:-}" ] && [ -f "${COOKIES}" ]; then
  COOKIE_ARGS=(--cookies "${COOKIES}")
fi

for bvid in "${BVIDS[@]}"; do
  echo "==> ${bvid}"
  yt-dlp "${COOKIE_ARGS[@]}" --no-warnings --no-playlist \
    -f 'bv*[height<=1080][vcodec^=avc1]+ba/bv*[height<=1080]+ba/b[height<=1080]' \
    --merge-output-format mp4 \
    -o "${DEST}/${bvid}.%(ext)s" \
    "https://www.bilibili.com/video/${bvid}" || echo "!! ${bvid} 下载失败，已跳过"
done

echo "完成，壁纸保存在 ${DEST}"
echo "打开 AniWall 后会自动扫描入库。"

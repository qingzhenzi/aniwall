export function fmtSize(bytes: number): string {
  if (!bytes) return "—";
  const mb = bytes / (1024 * 1024);
  return mb >= 1024 ? (mb / 1024).toFixed(1) + " GB" : mb.toFixed(1) + " MB";
}

export function fmtDuration(secs: number): string {
  if (!secs) return "";
  const total = Math.round(secs);
  const minutes = Math.floor(total / 60);
  const seconds = total % 60;
  return minutes > 0 ? `${minutes} 分 ${String(seconds).padStart(2, "0")} 秒` : `${seconds} 秒`;
}

export function fmtResolution(width: number, height: number): string {
  return width && height ? `${width}×${height}` : "";
}

export interface ToolInfo {
  name: string;
  ok: boolean;
  version: string;
  purpose: string;
}

export interface EnvStatus {
  distro_id: string;
  distro_name: string;
  kernel: string;
  session_type: string;
  desktop: string;
  plasma_version: string;
  plugin_version: string;
  plugin_ok: boolean;
  session_ok: boolean;
  tested_distro: boolean;
  tools: ToolInfo[];
  library_count: number;
  wallpaper_dir: string;
}

export interface Wallpaper {
  id: string;
  title: string;
  path: string;
  poster?: string | null;
  size_bytes: number;
  duration_secs: number;
  width: number;
  height: number;
  source: string;
  added_at: number;
}

export interface CatalogItem {
  id: string;
  bvid: string;
  title: string;
  tags: string[];
  poster: string;
  note: string;
}

export interface Settings {
  fill_mode: number;
  mute_mode: number;
  pause_mode: number;
  battery_pauses: boolean;
  battery_level: number;
  resume_last: boolean;
  cookie_file: string;
  active_id: string;
  loop_playback: boolean;
}

export interface DesktopWallpaperState {
  plugin: string;
  videos: string;
}

export type TabKey = "library" | "fetch" | "health" | "settings" | "about";

export interface DownloadItem {
  url: string;
  percent: number;
  status: "pending" | "start" | "downloading" | "done" | "error";
  message: string;
}

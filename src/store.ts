import { reactive } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import type {
  AppInfo,
  CatalogItem,
  DownloadItem,
  EnvStatus,
  Settings,
  Wallpaper,
} from "./types";
import { pushToast } from "./toast";

export const state = reactive({
  ready: false,
  env: null as EnvStatus | null,
  appInfo: null as AppInfo | null,
  autostart: false,
  library: [] as Wallpaper[],
  catalog: [] as CatalogItem[],
  settings: null as Settings | null,
  activeId: "",
  downloads: [] as DownloadItem[],
});

export async function call<T>(
  command: string,
  args: Record<string, unknown>,
  label: string
): Promise<T> {
  try {
    return await invoke<T>(command, args);
  } catch (err) {
    pushToast(`${label}失败：${String(err)}`, "error");
    throw err;
  }
}

export function uriToPath(uri: string): string {
  try {
    return decodeURIComponent(String(uri).replace(/^file:\/\//, ""));
  } catch {
    return String(uri).replace(/^file:\/\//, "");
  }
}

export async function refreshEnv() {
  state.env = await call<EnvStatus>("env_status", {}, "读取环境信息");
}

export async function refreshCurrent() {
  try {
    const result = await invoke<Array<{ plugin: string; videos: string }>>("current_wallpaper");
    const first = Array.isArray(result) ? result[0] : undefined;
    let videos: Array<{ filename?: string; enabled?: boolean }> = [];
    if (first && first.videos) {
      try {
        videos = JSON.parse(first.videos);
      } catch {
        videos = [];
      }
    }
    const enabled = videos.find((video) => video.enabled !== false) || videos[0];
    const path = enabled && enabled.filename ? uriToPath(enabled.filename) : "";
    const match = state.library.find((item) => item.path === path);
    state.activeId = match ? match.id : "";
  } catch {
    state.activeId = "";
  }
}

export async function refreshLibrary() {
  state.library = await call<Wallpaper[]>("list_wallpapers", {}, "读取壁纸库");
  await refreshCurrent();
}

export async function refreshCatalog() {
  state.catalog = await call<CatalogItem[]>("catalog", {}, "读取内置清单");
}

export async function loadSettings() {
  state.settings = await call<Settings>("read_settings", {}, "读取设置");
}

export async function loadAutostart() {
  state.autostart = await call<boolean>("get_autostart", {}, "读取开机自启状态");
}

export async function setAutostart(enabled: boolean): Promise<boolean> {
  try {
    const message = await invoke<string>("set_autostart", { enabled });
    pushToast(message, "ok");
    state.autostart = await invoke<boolean>("get_autostart");
  } catch (err) {
    pushToast(`设置开机自启失败：${String(err)}`, "error");
    state.autostart = !enabled;
  }
  return state.autostart;
}

export async function loadAppInfo() {
  state.appInfo = await call<AppInfo>("app_info", {}, "读取应用信息");
}

export async function randomWallpaper() {
  try {
    const message = await invoke<string>("random_wallpaper");
    pushToast(message, "ok");
    await refreshCurrent();
  } catch (err) {
    pushToast(`随机换壁纸失败：${String(err)}`, "error");
  }
}

export async function nextWallpaper() {
  try {
    const message = await invoke<string>("next_wallpaper");
    pushToast(message, "ok");
    await refreshCurrent();
  } catch (err) {
    pushToast(`切换下一张失败：${String(err)}`, "error");
  }
}

export async function saveSettings(next: Settings): Promise<boolean> {
  try {
    await invoke<string>("save_settings", { settings: next });
    state.settings = next;
    pushToast("设置已保存", "ok");
    await refreshCurrent();
    return true;
  } catch (err) {
    pushToast(`保存设置失败：${String(err)}`, "error");
    return false;
  }
}

export async function applyWallpaper(id: string) {
  try {
    const message = await invoke<string>("set_wallpaper", { id });
    pushToast(message, "ok");
    await refreshCurrent();
  } catch (err) {
    pushToast(`应用壁纸失败：${String(err)}`, "error");
  }
}

export async function deleteWallpaper(id: string) {
  try {
    await invoke("delete_wallpaper", { id });
    pushToast("已从壁纸库删除", "ok");
    await refreshLibrary();
    await refreshCatalog();
  } catch (err) {
    pushToast(`删除失败：${String(err)}`, "error");
  }
}

export async function restoreWallpaper() {
  try {
    const message = await invoke<string>("restore_wallpaper");
    pushToast(message, "ok");
    await refreshCurrent();
  } catch (err) {
    pushToast(`恢复静态壁纸失败：${String(err)}`, "error");
  }
}

export async function importPaths(paths: string[]) {
  if (!paths.length) return;
  try {
    const added = await invoke<Wallpaper[]>("import_files", { paths });
    pushToast(`已导入 ${added.length} 个视频`, "ok");
    await refreshLibrary();
    await refreshCatalog();
  } catch (err) {
    pushToast(`导入失败：${String(err)}`, "error");
  }
}

export async function downloadUrls(urls: string[]) {
  const list = urls.map((url) => url.trim()).filter(Boolean);
  if (!list.length) {
    pushToast("没有选中的壁纸", "warn");
    return;
  }
  state.downloads = list.map((url) => ({
    url,
    percent: 0,
    status: "pending",
    message: "等待中",
  }));
  try {
    await invoke("download_urls", { urls: list });
    pushToast("下载任务结束", "ok");
  } catch (err) {
    pushToast(`下载失败：${String(err)}`, "error");
  }
  await refreshLibrary();
  await refreshCatalog();
}

const installCommands = {
  plugin: ["install_plugin", "安装壁纸插件"],
  deps: ["install_deps", "安装系统依赖"],
  shortcut: ["install_shortcut", "创建桌面快捷方式"],
} as const;

export async function runInstall(kind: keyof typeof installCommands): Promise<string> {
  const [command, label] = installCommands[kind];
  return call<string>(command, {}, label);
}

export function hookEvents() {
  void listen<{ index: number; percent: number; status: string; message: string }>(
    "download-progress",
    (event) => {
      const payload = event.payload;
      const item = state.downloads[payload.index];
      if (!item) return;
      item.percent = payload.percent || 0;
      item.status = (payload.status as DownloadItem["status"]) || item.status;
      item.message = payload.message || item.message;
    }
  );
  void listen("library-updated", () => {
    void refreshLibrary().then(() => refreshCatalog());
  });
  void listen<string>("wallpaper-changed", (event) => {
    if (event.payload) pushToast(String(event.payload), "ok");
    void refreshCurrent();
  });
}

export async function init() {
  hookEvents();
  await Promise.all([refreshEnv(), refreshCatalog(), loadSettings()]);
  await Promise.allSettled([loadAutostart(), loadAppInfo()]);
  await refreshLibrary();
  state.ready = true;
  void invoke("ui_ready").catch(() => {});
}

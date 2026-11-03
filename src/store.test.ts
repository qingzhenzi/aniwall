import { describe, expect, it, vi } from "vitest";

const h = vi.hoisted(() => {
  const handlers: Record<string, (event: { payload: any }) => void> = {};
  const invoke = vi.fn(async (cmd: string) => {
    switch (cmd) {
      case "env_status":
        return {
          distro_id: "cachyos",
          distro_name: "CachyOS",
          kernel: "6.18",
          session_type: "wayland",
          desktop: "KDE",
          plasma_version: "plasmashell 6.7.4",
          plugin_version: "2.13.0",
          plugin_ok: true,
          session_ok: true,
          tested_distro: true,
          tools: [],
          library_count: 0,
          wallpaper_dir: "/tmp/aniwall-test",
        };
      case "catalog":
        return [];
      case "read_settings":
        return {
          fill_mode: 2,
          mute_mode: 5,
          pause_mode: 0,
          battery_pauses: true,
          battery_level: 20,
          resume_last: true,
          cookie_file: "",
          active_id: "",
          loop_playback: true,
        };
      case "get_autostart":
        return false;
      case "app_info":
        return { name: "AniWall", version: "0.1.0", exe: "", autostart: false, tray: true, tested_on: "" };
      case "list_wallpapers":
        return [];
      case "download_urls":
        return [];
      default:
        return null;
    }
  });
  return { handlers, invoke };
});

vi.mock("@tauri-apps/api/core", () => ({
  invoke: h.invoke,
  convertFileSrc: (path: string) => path,
}));

vi.mock("@tauri-apps/api/event", () => ({
  listen: vi.fn(async (name: string, handler: (event: { payload: any }) => void) => {
    h.handlers[name] = handler;
    return () => {};
  }),
}));

describe("下载队列", () => {
  it("批量下载时每个链接各自一条任务，进度按 batch+index 更新", async () => {
    const store = await import("./store");
    await store.init();

    await store.downloadUrls(["https://example.com/1", "https://example.com/2", "https://example.com/3"]);

    const tasks = store.state.downloads.filter((item) => item.url.startsWith("https://example.com/"));
    expect(tasks).toHaveLength(3);
    const batchId = tasks[0].batchId;
    expect(tasks[1].batchId).toBe(batchId);
    expect(tasks[2].batchId).toBe(batchId);
    expect(new Set(tasks.map((item) => item.id)).size).toBe(3);

    h.handlers["download-progress"]({
      payload: {
        batch_id: batchId,
        index: 1,
        percent: 66.6,
        status: "downloading",
        message: "[download] 66.6%",
      },
    });

    const updated = store.state.downloads.find((item) => item.id === `${batchId}-1`);
    expect(updated).toBeTruthy();
    expect(updated!.percent).toBeCloseTo(66.6);
    expect(updated!.status).toBe("downloading");
  });
});

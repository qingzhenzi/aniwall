import { describe, expect, it, vi } from "vitest";
import { mount } from "@vue/test-utils";
import DownloadList from "./DownloadList.vue";
import type { DownloadItem } from "../types";

vi.mock("../store", () => ({ removeDownload: vi.fn() }));

const items: DownloadItem[] = [
  {
    id: "b1-0",
    batchId: "b1",
    url: "https://example.com/a",
    title: "壁纸 A",
    percent: 100,
    status: "done",
    message: "下载完成",
    createdAt: 1,
  },
  {
    id: "b1-1",
    batchId: "b1",
    url: "https://example.com/b",
    title: "",
    percent: 42.5,
    status: "downloading",
    message: "[download] 42.5% of 10MiB",
    createdAt: 2,
  },
  {
    id: "b1-2",
    batchId: "b1",
    url: "https://example.com/c",
    title: "",
    percent: 0,
    status: "error",
    message: "yt-dlp 下载失败",
    createdAt: 3,
  },
];

describe("DownloadList", () => {
  it("多个下载任务会全部渲染（而不是只显示一个）", () => {
    const wrapper = mount(DownloadList, { props: { downloads: items } });
    expect(wrapper.findAll(".dl")).toHaveLength(3);
    expect(wrapper.text()).toContain("壁纸 A");
    expect(wrapper.text()).toContain("42.5%");
    expect(wrapper.text()).toContain("yt-dlp 下载失败");
  });

  it("空队列显示占位提示", () => {
    const wrapper = mount(DownloadList, { props: { downloads: [] } });
    expect(wrapper.findAll(".dl")).toHaveLength(0);
    expect(wrapper.text()).toContain("暂无下载任务");
  });
});

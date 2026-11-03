import { afterEach, describe, expect, it } from "vitest";
import { applyTheme, resolveTheme } from "./theme";

describe("主题切换", () => {
  afterEach(() => {
    document.documentElement.removeAttribute("data-theme");
  });

  it("system 不设置 data-theme（交给系统媒体查询）", () => {
    expect(resolveTheme("system")).toBe("");
    applyTheme("system");
    expect(document.documentElement.hasAttribute("data-theme")).toBe(false);
  });

  it("显式选择浅色/深色会设置 data-theme", () => {
    applyTheme("dark");
    expect(document.documentElement.getAttribute("data-theme")).toBe("dark");
    applyTheme("light");
    expect(document.documentElement.getAttribute("data-theme")).toBe("light");
  });
});

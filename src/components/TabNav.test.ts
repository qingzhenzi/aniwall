import { describe, expect, it, vi } from "vitest";
import { mount } from "@vue/test-utils";
import TabNav from "./TabNav.vue";

const h = vi.hoisted(() => ({ state: { tab: "library" } }));

vi.mock("../store", () => ({ state: h.state }));

describe("TabNav", () => {
  it("点击标签切换到对应页面（多页面而不是单页滚动）", async () => {
    const wrapper = mount(TabNav);
    const tabs = wrapper.findAll(".tab");
    expect(tabs).toHaveLength(5);
    await tabs[1].trigger("click");
    expect(h.state.tab).toBe("fetch");
    await tabs[3].trigger("click");
    expect(h.state.tab).toBe("health");
  });
});

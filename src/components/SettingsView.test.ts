import { describe, expect, it, vi } from "vitest";
import { mount } from "@vue/test-utils";
import SettingsView from "./SettingsView.vue";

const h = vi.hoisted(() => ({
  state: {
    settings: {
      fill_mode: 2,
      mute_mode: 5,
      pause_mode: 0,
      battery_pauses: true,
      battery_level: 20,
      resume_last: true,
      cookie_file: "",
      active_id: "",
      loop_playback: true,
    },
    autostart: false,
  },
  setAutostart: vi.fn(async (value: boolean) => value),
}));

vi.mock("../store", () => ({
  state: h.state,
  loadAutostart: vi.fn(async () => false),
  saveSettings: vi.fn(async () => true),
  setAutostart: h.setAutostart,
}));

describe("SettingsView", () => {
  it("勾选开机自启时把「勾选后的值」传给后端（修复取反 bug）", async () => {
    const wrapper = mount(SettingsView);
    const checkbox = wrapper.find('input[type="checkbox"]');
    expect(checkbox.exists()).toBe(true);
    await checkbox.setValue(true);
    expect(h.setAutostart).toHaveBeenCalledWith(true);
  });
});

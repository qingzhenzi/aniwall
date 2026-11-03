import { describe, expect, it } from "vitest";
import { FILL_OPTIONS, MUTE_OPTIONS, PAUSE_OPTIONS, THEME_OPTIONS } from "./settings-options";

/**
 * 防止再次把插件枚举值写错（之前 FillMode 0/1 的文案是反的：
 * 选「适应比例」实际执行的是拉伸）。
 */
describe("设置选项与插件枚举对齐", () => {
  it("FillMode: 0=Stretch 1=PreserveAspectFit 2=PreserveAspectCrop", () => {
    const byValue = Object.fromEntries(FILL_OPTIONS.map((o) => [o.value, o.label]));
    expect(byValue[0]).toContain("拉伸");
    expect(byValue[1]).toContain("适应");
    expect(byValue[2]).toContain("裁剪");
  });

  it("MuteMode 覆盖插件 0/1/2/4/5（4=Never, 5=Always）", () => {
    const values = MUTE_OPTIONS.map((o) => o.value).sort((a, b) => a - b);
    expect(values).toEqual([0, 1, 2, 4, 5]);
    expect(MUTE_OPTIONS.find((o) => o.value === 4)!.label).toContain("从不静音");
    expect(MUTE_OPTIONS.find((o) => o.value === 5)!.label).toContain("始终静音");
  });

  it("PauseMode 为 0..3", () => {
    expect(PAUSE_OPTIONS.map((o) => o.value)).toEqual([0, 1, 2, 3]);
  });

  it("主题选项包含 跟随系统/浅色/深色", () => {
    expect(THEME_OPTIONS.map((o) => o.value)).toEqual(["system", "light", "dark"]);
  });
});

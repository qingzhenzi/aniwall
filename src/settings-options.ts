/**
 * 选项值与 Smart Video Wallpaper Reborn 插件枚举严格对应，改之前先看插件源码：
 *   contents/ui/code/enum.js   —— MuteMode / PauseMode
 *   contents/ui/config.qml     —— FillMode 使用 VideoOutput.Stretch / PreserveAspectFit / PreserveAspectCrop
 */

export interface Option<T> {
  value: T;
  label: string;
}

/** VideoOutput: Stretch=0, PreserveAspectFit=1, PreserveAspectCrop=2 */
export const FILL_OPTIONS: Array<Option<number>> = [
  { value: 2, label: "裁剪填充（推荐）" },
  { value: 0, label: "拉伸铺满" },
  { value: 1, label: "适应比例（可能留黑边）" },
];

/** enum.js MuteMode: 0=MaxOrFull 1=ActiveWindow 2=WindowVisible 4=Never 5=Always */
export const MUTE_OPTIONS: Array<Option<number>> = [
  { value: 5, label: "始终静音（推荐）" },
  { value: 4, label: "从不静音（想用音频设备时选这个）" },
  { value: 2, label: "有窗口时静音" },
  { value: 1, label: "当前窗口活跃时静音" },
  { value: 0, label: "有最大化 / 全屏窗口时静音" },
];

/** enum.js PauseMode: 0=MaxOrFull 1=ActiveWindow 2=WindowVisible 3=Never */
export const PAUSE_OPTIONS: Array<Option<number>> = [
  { value: 0, label: "有最大化 / 全屏窗口时暂停（推荐）" },
  { value: 1, label: "当前窗口活跃时暂停" },
  { value: 2, label: "有任意窗口时暂停" },
  { value: 3, label: "从不暂停" },
];

export const THEME_OPTIONS: Array<Option<string>> = [
  { value: "system", label: "跟随系统" },
  { value: "light", label: "浅色" },
  { value: "dark", label: "深色" },
];

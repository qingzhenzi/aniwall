export type ThemeChoice = "system" | "light" | "dark";

/** 把用户选择解析成实际要挂到 <html data-theme> 的值；system 返回空字符串 */
export function resolveTheme(theme: string | undefined | null): string {
  if (theme === "light" || theme === "dark") return theme;
  return "";
}

export function applyTheme(theme: string | undefined | null) {
  const root = document.documentElement;
  const resolved = resolveTheme(theme);
  if (resolved) root.setAttribute("data-theme", resolved);
  else root.removeAttribute("data-theme");
}

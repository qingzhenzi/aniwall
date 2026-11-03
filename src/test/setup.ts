// jsdom 下补齐 Tauri 运行时不会提供的东西
if (!window.matchMedia) {
  // @ts-expect-error jsdom 没有实现
  window.matchMedia = () => ({
    matches: false,
    addEventListener: () => {},
    removeEventListener: () => {},
  });
}

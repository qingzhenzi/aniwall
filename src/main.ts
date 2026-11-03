import { createApp } from "vue";
import { invoke } from "@tauri-apps/api/core";
import App from "./App.vue";
import "./style.css";

function report(message: string) {
  void invoke("ui_log", { message }).catch(() => {});
}

window.addEventListener("error", (event) => {
  report(`error: ${event.message} @ ${event.filename}:${event.lineno}:${event.colno}`);
});
window.addEventListener("unhandledrejection", (event) => {
  report(`rejection: ${String((event as PromiseRejectionEvent).reason)}`);
});

report("boot: main.ts loaded");
createApp(App).mount("#app");

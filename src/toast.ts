import { reactive } from "vue";

export interface ToastItem {
  id: number;
  message: string;
  kind: "info" | "ok" | "warn" | "error";
}

export const toasts = reactive<ToastItem[]>([]);

let seq = 1;

export function pushToast(message: string, kind: ToastItem["kind"] = "info") {
  const id = seq++;
  toasts.push({ id, message, kind });
  window.setTimeout(() => {
    const index = toasts.findIndex((item) => item.id === id);
    if (index >= 0) toasts.splice(index, 1);
  }, 4600);
}

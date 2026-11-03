<script setup lang="ts">
import { computed, ref } from "vue";
import { refreshEnv, runInstall, state } from "../store";

type CheckState = "ok" | "warn" | "bad";

interface CheckRow {
  label: string;
  detail: string;
  state: CheckState;
  action?: { kind: "plugin" | "deps"; text: string };
}

const busy = ref(false);
const output = ref("");

const checks = computed<CheckRow[]>(() => {
  const env = state.env;
  if (!env) return [];
  const rows: CheckRow[] = [
    {
      label: "发行版",
      detail: `${env.distro_name}（内核 ${env.kernel}）`,
      state: env.tested_distro ? "ok" : "warn",
    },
    {
      label: "桌面会话",
      detail: `${env.desktop || "unknown"} / ${env.session_type}`,
      state: env.session_ok ? "ok" : "bad",
    },
    {
      label: "Plasma 版本",
      detail: env.plasma_version || "未检测到 plasmashell",
      state: env.plasma_version ? "ok" : "bad",
    },
    {
      label: "视频壁纸插件",
      detail: env.plugin_ok
        ? `Smart Video Wallpaper Reborn ${env.plugin_version}`
        : "未安装 Smart Video Wallpaper Reborn",
      state: env.plugin_ok ? "ok" : "bad",
      action: env.plugin_ok ? undefined : { kind: "plugin", text: "安装插件" },
    },
    {
      label: "托盘",
      detail: state.appInfo?.tray
        ? "已就绪 · 关闭窗口最小化到托盘，托盘左键显示/隐藏"
        : "未就绪（重开应用或检查 libayatana-appindicator）",
      state: state.appInfo?.tray ? "ok" : "warn",
    },
    {
      label: "开机自启",
      detail: state.autostart
        ? "已开启 · 登录后静默启动到托盘（--minimized）"
        : "未开启 · 可在「播放设置」里打开",
      state: state.autostart ? "ok" : "warn",
    },
  ];
  for (const tool of env.tools) {
    rows.push({
      label: `依赖 · ${tool.name}`,
      detail: tool.ok ? tool.version || "已安装" : `缺失 —— ${tool.purpose}`,
      state: tool.ok ? "ok" : "bad",
      action: tool.ok ? undefined : { kind: "deps", text: "安装依赖" },
    });
  }
  return rows;
});

const problemCount = computed(() => checks.value.filter((row) => row.state !== "ok").length);

async function run(kind: "plugin" | "deps" | "shortcut") {
  if (busy.value) return;
  busy.value = true;
  output.value = "执行中…";
  try {
    const message = await runInstall(kind);
    output.value = message;
  } catch (error) {
    output.value = "失败：" + String(error);
  } finally {
    busy.value = false;
    await refreshEnv();
  }
}
</script>

<template>
  <section>
    <div class="section-head">
      <div>
        <h2>初始化 / 诊断</h2>
        <p>检查运行环境，并一键补齐缺失的依赖与插件。</p>
      </div>
      <div class="actions">
        <button class="ghost" @click="refreshEnv">重新检测</button>
      </div>
    </div>

    <div class="health-summary">
      <span v-if="problemCount === 0">✅ {{ checks.length }} 项检查全部通过，可以正常使用。</span>
      <span v-else>⚠️ {{ checks.length }} 项检查中有 <b>{{ problemCount }}</b> 项需要处理。</span>
    </div>

    <div class="health-list">
      <div v-for="row in checks" :key="row.label" class="health-row">
        <div class="icon" :class="row.state">
          {{ row.state === "ok" ? "✓" : row.state === "warn" ? "!" : "✕" }}
        </div>
        <div class="text">
          <b>{{ row.label }}</b>
          <div>{{ row.detail }}</div>
        </div>
        <button
          v-if="row.action"
          class="mini ghost"
          :disabled="busy"
          @click="run(row.action.kind)"
        >
          {{ row.action.text }}
        </button>
      </div>
    </div>

    <div class="divider" />

    <h2>常用操作</h2>
    <div class="actions wrap">
      <button :disabled="busy" @click="run('plugin')">安装壁纸插件</button>
      <button class="ghost" :disabled="busy" @click="run('deps')">安装系统依赖（pkexec）</button>
      <button class="ghost" :disabled="busy" @click="run('shortcut')">创建桌面快捷方式</button>
    </div>

    <pre v-if="output" class="output">{{ output }}</pre>
  </section>
</template>

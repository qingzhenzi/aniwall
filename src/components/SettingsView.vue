<script setup lang="ts">
import { onMounted, reactive, ref, watch } from "vue";
import { loadAutostart, saveSettings, setAutostart, state } from "../store";
import type { Settings } from "../types";

const form = reactive<Settings>({
  fill_mode: 2,
  mute_mode: 5,
  pause_mode: 0,
  battery_pauses: true,
  battery_level: 20,
  resume_last: true,
  cookie_file: "",
  active_id: "",
  loop_playback: true,
});

watch(
  () => state.settings,
  (value) => {
    if (value) Object.assign(form, value);
  },
  { immediate: true, deep: true }
);

const autostart = ref(state.autostart);
const saving = ref(false);

watch(
  () => state.autostart,
  (value) => {
    autostart.value = value;
  }
);

onMounted(() => {
  void loadAutostart();
});

async function save() {
  saving.value = true;
  try {
    await saveSettings({ ...form });
  } finally {
    saving.value = false;
  }
}

async function toggleAutostart() {
  // v-model 已经把勾选状态改成新值，这里直接同步给后端
  autostart.value = await setAutostart(autostart.value);
}
</script>

<template>
  <section>
    <div class="section-head">
      <div>
        <h2>播放设置</h2>
        <p>保存后会同步给当前已应用的壁纸；其它选项即时生效。</p>
      </div>
      <div class="actions">
        <button class="primary" :disabled="saving" @click="save">保存设置</button>
      </div>
    </div>

    <div class="form">
      <label class="field">
        <span>开机自启</span>
        <div class="inline">
          <input v-model="autostart" type="checkbox" @change="toggleAutostart" />
          <span class="muted">启动后静默到托盘，不弹窗口</span>
        </div>
        <span class="hint">写入 ~/.config/autostart/aniwall.desktop（带 --minimized 参数）</span>
      </label>

      <div class="field">
        <span>托盘与窗口</span>
        <div class="inline">
          <span class="muted">关闭窗口 = 最小化到托盘；托盘左键显示/隐藏，托盘菜单可随机换、退出</span>
        </div>
      </div>

      <label class="field">
        <span>填充模式</span>
        <select v-model.number="form.fill_mode">
          <option :value="2">裁剪填充（推荐）</option>
          <option :value="1">拉伸铺满</option>
          <option :value="0">适应比例</option>
        </select>
      </label>

      <label class="field">
        <span>音频</span>
        <select v-model.number="form.mute_mode">
          <option :value="5">始终静音（推荐）</option>
          <option :value="4">从不静音</option>
          <option :value="2">有窗口时静音</option>
        </select>
      </label>

      <label class="field">
        <span>暂停策略</span>
        <select v-model.number="form.pause_mode">
          <option :value="0">有最大化 / 全屏窗口时暂停（推荐）</option>
          <option :value="1">当前窗口活跃时暂停</option>
          <option :value="2">有任何窗口时暂停</option>
          <option :value="3">从不暂停</option>
        </select>
      </label>

      <label class="field">
        <span>电量低于阈值时暂停</span>
        <div class="inline">
          <input v-model="form.battery_pauses" type="checkbox" />
          <input v-model.number="form.battery_level" type="number" min="5" max="90" step="5" class="num" />
          <span class="muted">%</span>
        </div>
      </label>

      <label class="field">
        <span>循环播放</span>
        <div class="inline">
          <input v-model="form.loop_playback" type="checkbox" />
        </div>
      </label>

      <label class="field wide">
        <span>B 站 cookies 文件（可选，部分视频需要登录才有 1080p）</span>
        <input v-model="form.cookie_file" type="text" placeholder="/home/you/bili-cookies.txt" />
      </label>
    </div>
  </section>
</template>

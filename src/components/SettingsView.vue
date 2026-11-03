<script setup lang="ts">
import { computed, onMounted, reactive, ref, watch } from "vue";
import { loadAutostart, saveSettings, setAutostart, state } from "../store";
import type { Settings } from "../types";
import {
  FILL_OPTIONS,
  MUTE_OPTIONS,
  PAUSE_OPTIONS,
  THEME_OPTIONS,
} from "../settings-options";

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
  volume: 1,
  audio_device: "",
  theme: "system",
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

const volumePercent = computed({
  get: () => Math.round((form.volume ?? 1) * 100),
  set: (value: number) => {
    form.volume = Math.max(0, Math.min(100, value)) / 100;
  },
});

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
  autostart.value = await setAutostart(autostart.value);
}
</script>

<template>
  <section>
    <div class="section-head">
      <div>
        <h2>播放设置</h2>
        <p>点「保存设置」后会立刻应用到当前桌面壁纸（如果正在播放视频壁纸）。</p>
      </div>
      <div class="actions">
        <button class="primary" :disabled="saving" @click="save">保存设置</button>
      </div>
    </div>

    <div class="form">
      <label class="field">
        <span>界面主题</span>
        <select v-model="form.theme">
          <option v-for="option in THEME_OPTIONS" :key="option.value" :value="option.value">
            {{ option.label }}
          </option>
        </select>
        <span class="hint">「跟随系统」会随 KDE 的浅色 / 深色自动切换</span>
      </label>

      <label class="field">
        <span>开机自启</span>
        <div class="inline">
          <input v-model="autostart" type="checkbox" @change="toggleAutostart" />
          <span class="muted">启动后静默到托盘，不弹窗口</span>
        </div>
        <span class="hint">写入 ~/.config/autostart/aniwall.desktop（带 --minimized 参数）</span>
      </label>

      <label class="field">
        <span>填充模式</span>
        <select v-model.number="form.fill_mode">
          <option v-for="option in FILL_OPTIONS" :key="option.value" :value="option.value">
            {{ option.label }}
          </option>
        </select>
        <span class="hint">对应插件 Positioning：0=拉伸 1=适应比例 2=裁剪填充</span>
      </label>

      <label class="field">
        <span>音频 / 静音模式</span>
        <select v-model.number="form.mute_mode">
          <option v-for="option in MUTE_OPTIONS" :key="option.value" :value="option.value">
            {{ option.label }}
          </option>
        </select>
        <span class="hint">
          默认「始终静音」。想在插件自己的设置里用音频输出设备，要先把这里改成「从不静音」，
          否则设备选了也不会有声音。
        </span>
      </label>

      <label class="field">
        <span>音量（{{ volumePercent }}%）</span>
        <input v-model.number="volumePercent" type="range" min="0" max="100" step="5" class="range" />
        <span class="hint">仅在「从不静音 / 部分静音模式」下生效</span>
      </label>

      <label class="field">
        <span>音频输出设备（可选）</span>
        <input v-model="form.audio_device" type="text" placeholder="留空 = 系统默认；填插件设置里显示的设备 ID" />
        <span class="hint">插件设置 → 音频设备 里能看到设备 ID</span>
      </label>

      <label class="field">
        <span>暂停策略</span>
        <select v-model.number="form.pause_mode">
          <option v-for="option in PAUSE_OPTIONS" :key="option.value" :value="option.value">
            {{ option.label }}
          </option>
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

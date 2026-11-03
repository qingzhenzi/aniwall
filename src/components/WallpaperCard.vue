<script setup lang="ts">
import { computed, ref } from "vue";
import { convertFileSrc } from "@tauri-apps/api/core";
import type { Wallpaper } from "../types";
import { fmtDuration, fmtResolution, fmtSize } from "../format";

const props = defineProps<{ item: Wallpaper; active: boolean }>();
const emit = defineEmits<{ apply: []; remove: [] }>();

const brokenPoster = ref(false);

const posterUrl = computed(() => {
  if (!props.item.poster || brokenPoster.value) return "";
  return convertFileSrc(props.item.poster);
});

const meta = computed(() =>
  [fmtResolution(props.item.width, props.item.height), fmtDuration(props.item.duration_secs), fmtSize(props.item.size_bytes)]
    .filter(Boolean)
    .join(" · ")
);
</script>

<template>
  <article class="card" :class="{ active }">
    <div
      class="poster"
      :style="posterUrl ? { backgroundImage: `url('${posterUrl}')` } : undefined"
    >
      <span v-if="!posterUrl">▶</span>
      <span v-if="active" class="badge">使用中</span>
      <img
        v-if="posterUrl"
        :src="posterUrl"
        alt=""
        style="display: none"
        @error="brokenPoster = true"
      />
    </div>
    <div class="body">
      <h3 :title="item.title">{{ item.title }}</h3>
      <div class="meta">{{ meta || "—" }}</div>
      <div class="row">
        <button class="mini" @click="emit('apply')">{{ active ? "重新应用" : "应用" }}</button>
        <button class="mini danger" @click="emit('remove')">删除</button>
      </div>
    </div>
  </article>
</template>

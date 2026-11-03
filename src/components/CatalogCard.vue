<script setup lang="ts">
import type { CatalogItem } from "../types";

defineProps<{ item: CatalogItem; downloaded: boolean; checked: boolean }>();
const emit = defineEmits<{ toggle: []; download: [] }>();
</script>

<template>
  <article class="card">
    <div
      class="poster"
      :style="item.poster ? { backgroundImage: `url('${item.poster}')` } : undefined"
    >
      <input class="check" type="checkbox" :checked="checked" @change="emit('toggle')" />
      <span v-if="downloaded" class="badge">已入库</span>
    </div>
    <div class="body">
      <h3 :title="item.title">{{ item.title }}</h3>
      <div class="meta">{{ item.note || "—" }}</div>
      <div class="tags">
        <span v-for="tag in item.tags" :key="tag" class="tag">{{ tag }}</span>
      </div>
      <div class="row">
        <button class="mini ghost" @click="emit('download')">下载</button>
      </div>
    </div>
  </article>
</template>

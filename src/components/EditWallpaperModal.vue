<script setup lang="ts">
import { ref, watch } from "vue";
import type { Wallpaper } from "../types";

const props = defineProps<{ item: Wallpaper | null }>();
const emit = defineEmits<{ save: [title: string]; close: [] }>();

const title = ref("");

watch(
  () => props.item,
  (value) => {
    title.value = value ? value.title : "";
  },
  { immediate: true }
);

function save() {
  if (!title.value.trim()) return;
  emit("save", title.value.trim());
}
</script>

<template>
  <div v-if="item" class="modal-mask" @click.self="emit('close')">
    <div class="modal">
      <h3>编辑壁纸</h3>
      <p class="modal-message">重命名后只影响壁纸库里的显示标题，不会改动视频文件。</p>
      <label class="field modal-field">
        <span>标题</span>
        <input v-model="title" type="text" placeholder="给这张壁纸起个名字" @keyup.enter="save" />
      </label>
      <p class="muted modal-path">{{ item.path }}</p>
      <div class="modal-actions">
        <button class="ghost" @click="emit('close')">取消</button>
        <button class="primary" :disabled="!title.trim()" @click="save">保存</button>
      </div>
    </div>
  </div>
</template>

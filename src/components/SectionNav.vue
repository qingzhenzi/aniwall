<script setup lang="ts">
defineProps<{
  sections: Array<{ id: string; label: string }>;
  active: string;
}>();

const emit = defineEmits<{ select: [id: string] }>();
</script>

<template>
  <nav class="section-nav">
    <button
      v-for="section in sections"
      :key="section.id"
      :class="{ active: active === section.id }"
      @click="emit('select', section.id)"
    >
      {{ section.label }}
    </button>
  </nav>
</template>

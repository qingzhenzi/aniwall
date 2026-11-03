<script setup lang="ts">
import { toasts } from "../toast";
</script>

<template>
  <div class="toasts">
    <div v-for="toast in toasts" :key="toast.id" class="toast" :class="toast.kind">
      {{ toast.message }}
    </div>
  </div>
</template>

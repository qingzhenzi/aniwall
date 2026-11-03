<script setup lang="ts">
import { confirmState, resolveConfirm } from "../store";
</script>

<template>
  <div v-if="confirmState.open" class="modal-mask" @click.self="resolveConfirm(false)">
    <div class="modal" role="dialog" aria-modal="true">
      <h3>{{ confirmState.title }}</h3>
      <p class="modal-message">{{ confirmState.message }}</p>
      <div class="modal-actions">
        <button class="ghost" @click="resolveConfirm(false)">{{ confirmState.cancelText }}</button>
        <button class="primary" @click="resolveConfirm(true)">{{ confirmState.okText }}</button>
      </div>
    </div>
  </div>
</template>

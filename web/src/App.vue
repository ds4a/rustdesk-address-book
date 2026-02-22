<script setup lang="ts">
import { useUiStore } from './stores/ui'

// Initialize UI store to apply dark mode class on mount
useUiStore()
</script>

<template>
  <router-view />
</template>

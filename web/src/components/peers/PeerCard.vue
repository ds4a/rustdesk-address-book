<script setup lang="ts">
import type { Peer } from '../../api/addressBook'

const props = defineProps<{ peer: Peer }>()
const emit = defineEmits<{ delete: [id: string] }>()

const platformColors: Record<string, string> = {
  Windows: 'bg-platform-windows',
  'Mac OS': 'bg-platform-macos',
  macOS: 'bg-platform-macos',
  Linux: 'bg-platform-linux',
  Android: 'bg-platform-android',
  iOS: 'bg-platform-ios',
}

const platformIcons: Record<string, string> = {
  Windows: 'W',
  'Mac OS': 'M',
  macOS: 'M',
  Linux: 'L',
  Android: 'A',
  iOS: 'i',
}

function getPlatformColor(platform: string) {
  return platformColors[platform] || 'bg-gray-400'
}

function getPlatformIcon(platform: string) {
  return platformIcons[platform] || '?'
}

function displayName(peer: Peer) {
  return peer.alias || peer.hostname || peer.id
}
</script>

<template>
  <div class="group relative bg-white dark:bg-rd-card border border-gray-200 dark:border-rd-border rounded-xl overflow-hidden hover:shadow-md dark:hover:border-rd-text-secondary/30 transition-all cursor-pointer">
    <!-- Platform color strip -->
    <div class="h-1" :class="getPlatformColor(peer.platform)" />

    <div class="p-4">
      <!-- Header: platform icon + ID -->
      <div class="flex items-start justify-between mb-3">
        <div class="flex items-center gap-2">
          <div class="w-9 h-9 rounded-lg flex items-center justify-center text-white text-sm font-bold" :class="getPlatformColor(peer.platform)">
            {{ getPlatformIcon(peer.platform) }}
          </div>
          <div>
            <p class="text-sm font-semibold text-gray-900 dark:text-rd-text leading-tight">{{ displayName(peer) }}</p>
            <p class="text-xs text-gray-500 dark:text-rd-text-secondary font-mono">{{ peer.id }}</p>
          </div>
        </div>

        <!-- Delete button -->
        <button
          @click.stop="emit('delete', peer.id)"
          class="opacity-0 group-hover:opacity-100 p-1 rounded text-gray-400 hover:text-rd-danger transition-all"
          title="Remove peer"
        >
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
          </svg>
        </button>
      </div>

      <!-- Info -->
      <div class="text-xs text-gray-500 dark:text-rd-text-secondary space-y-0.5">
        <p v-if="peer.username">User: {{ peer.username }}</p>
        <p v-if="peer.platform">{{ peer.platform }}</p>
      </div>

      <!-- Tags -->
      <div v-if="peer.tags.length" class="flex flex-wrap gap-1 mt-3">
        <span
          v-for="tag in peer.tags"
          :key="tag"
          class="px-2 py-0.5 text-xs rounded bg-rd-primary/10 text-rd-primary dark:bg-rd-primary/20"
        >
          {{ tag }}
        </span>
      </div>
    </div>
  </div>
</template>

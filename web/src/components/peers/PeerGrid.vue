<script setup lang="ts">
import type { Peer } from '../../api/addressBook'
import PeerCard from './PeerCard.vue'

defineProps<{ peers: Peer[] }>()
const emit = defineEmits<{ delete: [id: string] }>()
</script>

<template>
  <div v-if="peers.length" class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 2xl:grid-cols-5 gap-4">
    <PeerCard
      v-for="peer in peers"
      :key="peer.id"
      :peer="peer"
      @delete="emit('delete', $event)"
    />
  </div>
  <div v-else class="flex flex-col items-center justify-center py-20 text-gray-400 dark:text-rd-text-secondary">
    <svg class="w-16 h-16 mb-4 opacity-50" fill="none" stroke="currentColor" viewBox="0 0 24 24">
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M9.75 17L9 20l-1 1h8l-1-1-.75-3M3 13h18M5 17h14a2 2 0 002-2V5a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
    </svg>
    <p class="text-sm">No peers in this address book</p>
    <p class="text-xs mt-1">Add peers using the RustDesk client or the button above</p>
  </div>
</template>

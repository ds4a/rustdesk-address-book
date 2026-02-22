<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { api } from '../api/client'
import { useAddressBookStore } from '../stores/addressBook'

const abStore = useAddressBookStore()

const stats = ref({
  totalPeers: 0,
  totalTags: 0,
  totalUsers: 0,
  recentAudit: [] as any[],
})

onMounted(async () => {
  await abStore.init()
  stats.value.totalPeers = abStore.totalPeers
  stats.value.totalTags = abStore.tags.length

  try {
    const users = await api('/api/users')
    stats.value.totalUsers = users.total || 0
  } catch {
    // not admin, ignore
  }
})

const statCards = [
  { label: 'Peers', key: 'totalPeers', icon: 'monitor', color: 'bg-blue-500' },
  { label: 'Tags', key: 'totalTags', icon: 'tag', color: 'bg-green-500' },
  { label: 'Users', key: 'totalUsers', icon: 'users', color: 'bg-purple-500' },
]
</script>

<template>
  <div>
    <h1 class="text-lg font-semibold text-gray-900 dark:text-rd-text mb-6">Dashboard</h1>

    <!-- Stats grid -->
    <div class="grid grid-cols-1 sm:grid-cols-3 gap-4 mb-8">
      <div
        v-for="card in statCards"
        :key="card.key"
        class="bg-white dark:bg-rd-card border border-gray-200 dark:border-rd-border rounded-xl p-5"
      >
        <div class="flex items-center justify-between">
          <div>
            <p class="text-sm text-gray-500 dark:text-rd-text-secondary">{{ card.label }}</p>
            <p class="text-2xl font-bold text-gray-900 dark:text-rd-text mt-1">
              {{ stats[card.key as keyof typeof stats] }}
            </p>
          </div>
          <div class="w-10 h-10 rounded-lg flex items-center justify-center" :class="card.color + '/10'">
            <svg v-if="card.icon === 'monitor'" class="w-5 h-5 text-blue-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9.75 17L9 20l-1 1h8l-1-1-.75-3M3 13h18M5 17h14a2 2 0 002-2V5a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
            </svg>
            <svg v-else-if="card.icon === 'tag'" class="w-5 h-5 text-green-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 7h.01M7 3h5c.512 0 1.024.195 1.414.586l7 7a2 2 0 010 2.828l-7 7a2 2 0 01-2.828 0l-7-7A2 2 0 013 12V7a4 4 0 014-4z" />
            </svg>
            <svg v-else-if="card.icon === 'users'" class="w-5 h-5 text-purple-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197m13.5-9a2.5 2.5 0 11-5 0 2.5 2.5 0 015 0z" />
            </svg>
          </div>
        </div>
      </div>
    </div>

    <!-- Quick info -->
    <div class="bg-white dark:bg-rd-card border border-gray-200 dark:border-rd-border rounded-xl p-6">
      <h2 class="text-sm font-semibold text-gray-900 dark:text-rd-text mb-3">Getting Started</h2>
      <div class="text-sm text-gray-600 dark:text-rd-text-secondary space-y-2">
        <p>1. Configure your RustDesk client to use this server's address on port 21114.</p>
        <p>2. Log in with your credentials from the RustDesk client.</p>
        <p>3. Your address book will sync automatically between the client and this server.</p>
        <p>4. Use the <router-link to="/address-book" class="text-rd-primary hover:underline">Address Book</router-link> page to manage peers and tags from the web.</p>
      </div>
    </div>
  </div>
</template>

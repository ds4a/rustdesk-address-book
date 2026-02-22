<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { useAddressBookStore } from '../stores/addressBook'
import PeerGrid from '../components/peers/PeerGrid.vue'
import TagPanel from '../components/tags/TagPanel.vue'
import AddPeerDialog from '../components/peers/AddPeerDialog.vue'

const store = useAddressBookStore()
const showAddPeer = ref(false)
const search = ref('')

onMounted(() => {
  store.init()
})

const displayedPeers = computed(() => {
  let peers = store.filteredPeers()
  if (search.value) {
    const q = search.value.toLowerCase()
    peers = peers.filter(p =>
      p.id.toLowerCase().includes(q) ||
      p.alias.toLowerCase().includes(q) ||
      p.hostname.toLowerCase().includes(q) ||
      p.username.toLowerCase().includes(q)
    )
  }
  return peers
})

async function handleAddPeer(peer: { id: string; alias: string; platform: string; tags: string[] }) {
  await store.addPeer(peer)
  showAddPeer.value = false
}

async function handleDeletePeer(id: string) {
  if (confirm(`Remove peer ${id}?`)) {
    await store.removePeer([id])
  }
}

async function handleAddTag(name: string, color: number) {
  await store.addTag(name, color)
}

async function handleDeleteTag(name: string) {
  if (confirm(`Delete tag "${name}"?`)) {
    await store.removeTag([name])
  }
}
</script>

<template>
  <div class="flex gap-6 h-full">
    <!-- Tag sidebar -->
    <div class="w-48 shrink-0">
      <div class="bg-white dark:bg-rd-card border border-gray-200 dark:border-rd-border rounded-xl p-4">
        <TagPanel
          :tags="store.tags"
          :selected-tag="store.selectedTag"
          @select="store.selectedTag = $event"
          @add="handleAddTag"
          @delete="handleDeleteTag"
        />
      </div>
    </div>

    <!-- Main content -->
    <div class="flex-1 min-w-0">
      <!-- Toolbar -->
      <div class="flex items-center justify-between mb-4">
        <div class="flex items-center gap-3">
          <h1 class="text-lg font-semibold text-gray-900 dark:text-rd-text">Address Book</h1>
          <span class="text-sm text-gray-500 dark:text-rd-text-secondary">{{ store.totalPeers }} peers</span>
        </div>
        <div class="flex items-center gap-3">
          <!-- Search -->
          <div class="relative">
            <svg class="w-4 h-4 absolute left-3 top-1/2 -translate-y-1/2 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
            </svg>
            <input
              v-model="search"
              type="text"
              placeholder="Search peers..."
              class="pl-9 pr-3 py-2 text-sm rounded-lg border border-gray-300 dark:border-rd-border bg-white dark:bg-rd-card text-gray-900 dark:text-rd-text focus:outline-none focus:ring-2 focus:ring-rd-primary w-56"
            />
          </div>
          <!-- Add button -->
          <button
            @click="showAddPeer = true"
            class="px-4 py-2 text-sm rounded-lg bg-rd-primary hover:bg-rd-primary-hover text-white font-medium transition-colors flex items-center gap-2"
          >
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
            </svg>
            Add Peer
          </button>
        </div>
      </div>

      <!-- Peer grid -->
      <div v-if="store.loading" class="flex items-center justify-center py-20">
        <div class="w-8 h-8 border-2 border-rd-primary border-t-transparent rounded-full animate-spin" />
      </div>
      <PeerGrid v-else :peers="displayedPeers" @delete="handleDeletePeer" />
    </div>
  </div>

  <!-- Add peer dialog -->
  <AddPeerDialog v-if="showAddPeer" @close="showAddPeer = false" @add="handleAddPeer" />
</template>

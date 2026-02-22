<script setup lang="ts">
import { ref, onMounted } from 'vue'
import * as abApi from '../api/addressBook'

const groups = ref<abApi.GroupItem[]>([])
const showAdd = ref(false)
const newGroup = ref({ name: '', note: '' })
const error = ref('')

async function loadGroups() {
  const res = await abApi.getGroups()
  groups.value = res.data
}

async function handleCreate() {
  error.value = ''
  if (!newGroup.value.name) {
    error.value = 'Group name is required'
    return
  }
  try {
    await abApi.createGroup(newGroup.value)
    newGroup.value = { name: '', note: '' }
    showAdd.value = false
    await loadGroups()
  } catch (e: any) {
    error.value = e.message
  }
}

async function handleDelete(id: number, name: string) {
  if (confirm(`Delete group "${name}"?`)) {
    await abApi.deleteGroup(id)
    await loadGroups()
  }
}

onMounted(loadGroups)
</script>

<template>
  <div>
    <div class="flex items-center justify-between mb-6">
      <h1 class="text-lg font-semibold text-gray-900 dark:text-rd-text">Groups</h1>
      <button
        @click="showAdd = true"
        class="px-4 py-2 text-sm rounded-lg bg-rd-primary hover:bg-rd-primary-hover text-white font-medium transition-colors flex items-center gap-2"
      >
        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
        </svg>
        Add Group
      </button>
    </div>

    <!-- Add group form -->
    <div v-if="showAdd" class="bg-white dark:bg-rd-card border border-gray-200 dark:border-rd-border rounded-xl p-6 mb-6">
      <h2 class="text-sm font-semibold text-gray-900 dark:text-rd-text mb-4">New Group</h2>
      <div v-if="error" class="mb-4 p-3 text-sm text-red-600 bg-red-50 dark:bg-red-900/20 dark:text-red-400 rounded-lg">{{ error }}</div>
      <div class="grid grid-cols-2 gap-4">
        <div>
          <label class="block text-xs font-medium text-gray-600 dark:text-rd-text-secondary mb-1">Name *</label>
          <input v-model="newGroup.name" class="w-full px-3 py-2 text-sm rounded-lg border border-gray-300 dark:border-rd-border bg-white dark:bg-rd-scaffold text-gray-900 dark:text-rd-text focus:outline-none focus:ring-2 focus:ring-rd-primary" />
        </div>
        <div>
          <label class="block text-xs font-medium text-gray-600 dark:text-rd-text-secondary mb-1">Note</label>
          <input v-model="newGroup.note" class="w-full px-3 py-2 text-sm rounded-lg border border-gray-300 dark:border-rd-border bg-white dark:bg-rd-scaffold text-gray-900 dark:text-rd-text focus:outline-none focus:ring-2 focus:ring-rd-primary" />
        </div>
      </div>
      <div class="flex justify-end gap-3 mt-4">
        <button @click="showAdd = false" class="px-4 py-2 text-sm rounded-lg text-gray-600 dark:text-rd-text-secondary hover:bg-gray-100 dark:hover:bg-rd-card-hover">Cancel</button>
        <button @click="handleCreate" class="px-4 py-2 text-sm rounded-lg bg-rd-primary hover:bg-rd-primary-hover text-white font-medium">Create</button>
      </div>
    </div>

    <!-- Groups table -->
    <div class="bg-white dark:bg-rd-card border border-gray-200 dark:border-rd-border rounded-xl overflow-hidden">
      <table class="w-full">
        <thead>
          <tr class="border-b border-gray-200 dark:border-rd-border">
            <th class="text-left text-xs font-medium text-gray-500 dark:text-rd-text-secondary uppercase tracking-wide px-6 py-3">Name</th>
            <th class="text-left text-xs font-medium text-gray-500 dark:text-rd-text-secondary uppercase tracking-wide px-6 py-3">Note</th>
            <th class="text-left text-xs font-medium text-gray-500 dark:text-rd-text-secondary uppercase tracking-wide px-6 py-3">Created</th>
            <th class="text-right text-xs font-medium text-gray-500 dark:text-rd-text-secondary uppercase tracking-wide px-6 py-3">Actions</th>
          </tr>
        </thead>
        <tbody>
          <tr
            v-for="group in groups"
            :key="group.id"
            class="border-b border-gray-100 dark:border-rd-border/50 last:border-0 hover:bg-gray-50 dark:hover:bg-rd-card-hover"
          >
            <td class="px-6 py-3 text-sm text-gray-900 dark:text-rd-text font-medium">{{ group.name }}</td>
            <td class="px-6 py-3 text-sm text-gray-600 dark:text-rd-text-secondary">{{ group.note || '-' }}</td>
            <td class="px-6 py-3 text-sm text-gray-600 dark:text-rd-text-secondary">{{ group.created_at }}</td>
            <td class="px-6 py-3 text-right">
              <button
                @click="handleDelete(group.id, group.name)"
                class="text-sm text-gray-400 hover:text-rd-danger transition-colors"
              >
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                </svg>
              </button>
            </td>
          </tr>
        </tbody>
      </table>
      <div v-if="!groups.length" class="text-center py-10 text-sm text-gray-400 dark:text-rd-text-secondary">
        No groups found
      </div>
    </div>
  </div>
</template>

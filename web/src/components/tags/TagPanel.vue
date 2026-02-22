<script setup lang="ts">
import { ref } from 'vue'
import type { Tag } from '../../api/addressBook'

const props = defineProps<{
  tags: Tag[]
  selectedTag: string | null
}>()

const emit = defineEmits<{
  select: [tag: string | null]
  add: [name: string, color: number]
  delete: [name: string]
}>()

const showAdd = ref(false)
const newTagName = ref('')

function intToHex(n: number): string {
  // Convert ARGB integer to hex color string
  const hex = (n >>> 0).toString(16).padStart(8, '0')
  return '#' + hex.slice(2) // strip alpha
}

function addTag() {
  if (!newTagName.value.trim()) return
  emit('add', newTagName.value.trim(), 0xFF0071FF) // default to RustDesk primary
  newTagName.value = ''
  showAdd.value = false
}
</script>

<template>
  <div class="space-y-1">
    <div class="flex items-center justify-between mb-2">
      <h3 class="text-xs font-semibold text-gray-500 dark:text-rd-text-secondary uppercase tracking-wide">Tags</h3>
      <button
        @click="showAdd = !showAdd"
        class="p-1 rounded text-gray-400 hover:text-rd-primary transition-colors"
        title="Add tag"
      >
        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
        </svg>
      </button>
    </div>

    <!-- Add tag form -->
    <div v-if="showAdd" class="mb-2">
      <form @submit.prevent="addTag" class="flex gap-1">
        <input
          v-model="newTagName"
          type="text"
          class="flex-1 px-2 py-1 text-xs rounded border border-gray-300 dark:border-rd-border bg-white dark:bg-rd-scaffold text-gray-900 dark:text-rd-text focus:outline-none focus:ring-1 focus:ring-rd-primary"
          placeholder="Tag name"
          autofocus
        />
        <button type="submit" class="px-2 py-1 text-xs rounded bg-rd-primary text-white">Add</button>
      </form>
    </div>

    <!-- All tag (clear filter) -->
    <button
      @click="emit('select', null)"
      class="w-full flex items-center px-3 py-1.5 text-sm rounded-lg transition-colors"
      :class="!selectedTag
        ? 'bg-rd-primary text-white'
        : 'text-gray-600 dark:text-rd-text-secondary hover:bg-gray-100 dark:hover:bg-rd-card-hover'"
    >
      All
    </button>

    <!-- Tag list -->
    <div v-for="tag in tags" :key="tag.name" class="group flex items-center">
      <button
        @click="emit('select', tag.name)"
        class="flex-1 flex items-center px-3 py-1.5 text-sm rounded-lg transition-colors"
        :class="selectedTag === tag.name
          ? 'bg-rd-primary text-white'
          : 'text-gray-600 dark:text-rd-text-secondary hover:bg-gray-100 dark:hover:bg-rd-card-hover'"
      >
        <span
          class="w-2.5 h-2.5 rounded-full mr-2 shrink-0"
          :style="{ backgroundColor: intToHex(tag.color) }"
        />
        <span class="truncate">{{ tag.name }}</span>
      </button>
      <button
        @click="emit('delete', tag.name)"
        class="opacity-0 group-hover:opacity-100 p-1 rounded text-gray-400 hover:text-rd-danger transition-all shrink-0"
        title="Delete tag"
      >
        <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
        </svg>
      </button>
    </div>
  </div>
</template>

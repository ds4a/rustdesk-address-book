<script setup lang="ts">
import { useAuthStore } from '../../stores/auth'
import { useUiStore } from '../../stores/ui'
import { useRouter } from 'vue-router'

const auth = useAuthStore()
const ui = useUiStore()
const router = useRouter()

async function handleLogout() {
  await auth.logout()
  router.push('/login')
}
</script>

<template>
  <header class="h-14 border-b border-gray-200 dark:border-rd-border bg-white dark:bg-rd-card flex items-center justify-between px-6">
    <div class="text-sm text-gray-500 dark:text-rd-text-secondary">
      <slot name="title" />
    </div>
    <div class="flex items-center gap-3">
      <!-- Dark mode toggle -->
      <button
        @click="ui.toggleDark()"
        class="p-2 rounded-lg text-gray-500 dark:text-rd-text-secondary hover:bg-gray-100 dark:hover:bg-rd-card-hover transition-colors"
        :title="ui.dark ? 'Switch to light mode' : 'Switch to dark mode'"
      >
        <svg v-if="ui.dark" class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z" />
        </svg>
        <svg v-else class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z" />
        </svg>
      </button>

      <!-- Logout -->
      <button
        @click="handleLogout"
        class="px-3 py-1.5 text-sm rounded-lg text-gray-600 dark:text-rd-text-secondary hover:bg-gray-100 dark:hover:bg-rd-card-hover transition-colors"
      >
        Logout
      </button>
    </div>
  </header>
</template>

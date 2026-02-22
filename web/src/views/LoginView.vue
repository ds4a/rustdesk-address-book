<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '../stores/auth'

const auth = useAuthStore()
const router = useRouter()

const username = ref('')
const password = ref('')
const error = ref('')
const loading = ref(false)

async function handleLogin() {
  error.value = ''
  loading.value = true
  try {
    await auth.login(username.value, password.value)
    router.push('/')
  } catch (e: any) {
    error.value = e.message || 'Login failed'
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <div class="min-h-screen flex items-center justify-center bg-gray-50 dark:bg-rd-scaffold">
    <div class="w-full max-w-sm">
      <!-- Logo -->
      <div class="text-center mb-8">
        <div class="w-16 h-16 rounded-2xl bg-rd-primary flex items-center justify-center mx-auto mb-4">
          <svg class="w-10 h-10 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9.75 17L9 20l-1 1h8l-1-1-.75-3M3 13h18M5 17h14a2 2 0 002-2V5a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
          </svg>
        </div>
        <h1 class="text-xl font-semibold text-gray-900 dark:text-rd-text">RustDesk Address Book</h1>
        <p class="text-sm text-gray-500 dark:text-rd-text-secondary mt-1">Sign in to manage your devices</p>
      </div>

      <!-- Form -->
      <form @submit.prevent="handleLogin" class="bg-white dark:bg-rd-card rounded-2xl p-6 shadow-sm border border-gray-200 dark:border-rd-border">
        <div v-if="error" class="mb-4 p-3 text-sm text-red-600 bg-red-50 dark:bg-red-900/20 dark:text-red-400 rounded-lg">
          {{ error }}
        </div>

        <div class="space-y-4">
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-rd-text-secondary mb-1">Username</label>
            <input
              v-model="username"
              type="text"
              required
              autofocus
              class="w-full px-3 py-2 rounded-lg border border-gray-300 dark:border-rd-border bg-white dark:bg-rd-scaffold text-gray-900 dark:text-rd-text focus:outline-none focus:ring-2 focus:ring-rd-primary focus:border-transparent text-sm"
              placeholder="admin"
            />
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-rd-text-secondary mb-1">Password</label>
            <input
              v-model="password"
              type="password"
              required
              class="w-full px-3 py-2 rounded-lg border border-gray-300 dark:border-rd-border bg-white dark:bg-rd-scaffold text-gray-900 dark:text-rd-text focus:outline-none focus:ring-2 focus:ring-rd-primary focus:border-transparent text-sm"
              placeholder="Password"
            />
          </div>
        </div>

        <button
          type="submit"
          :disabled="loading"
          class="w-full mt-6 py-2.5 bg-rd-primary hover:bg-rd-primary-hover text-white text-sm font-medium rounded-lg transition-colors disabled:opacity-50"
        >
          {{ loading ? 'Signing in...' : 'Sign In' }}
        </button>
      </form>
    </div>
  </div>
</template>

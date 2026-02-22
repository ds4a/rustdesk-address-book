import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import * as authApi from '../api/auth'

export const useAuthStore = defineStore('auth', () => {
  const token = ref(localStorage.getItem('token') || '')
  const user = ref<authApi.UserInfo | null>(
    JSON.parse(localStorage.getItem('user') || 'null')
  )

  const isAuthenticated = computed(() => !!token.value)
  const isAdmin = computed(() => user.value?.is_admin ?? false)

  async function login(username: string, password: string) {
    const res = await authApi.login(username, password)
    token.value = res.access_token
    user.value = res.user
    localStorage.setItem('token', res.access_token)
    localStorage.setItem('user', JSON.stringify(res.user))
  }

  async function logout() {
    try {
      await authApi.logout()
    } catch {
      // ignore
    }
    token.value = ''
    user.value = null
    localStorage.removeItem('token')
    localStorage.removeItem('user')
  }

  return { token, user, isAuthenticated, isAdmin, login, logout }
})

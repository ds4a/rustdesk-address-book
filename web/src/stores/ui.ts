import { defineStore } from 'pinia'
import { ref, watch } from 'vue'

export const useUiStore = defineStore('ui', () => {
  const dark = ref(localStorage.getItem('theme') === 'dark' ||
    (!localStorage.getItem('theme') && window.matchMedia('(prefers-color-scheme: dark)').matches))

  watch(dark, (val) => {
    localStorage.setItem('theme', val ? 'dark' : 'light')
    document.documentElement.classList.toggle('dark', val)
  }, { immediate: true })

  function toggleDark() {
    dark.value = !dark.value
  }

  return { dark, toggleDark }
})

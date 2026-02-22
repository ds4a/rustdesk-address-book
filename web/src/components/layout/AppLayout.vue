<script setup lang="ts">
import Sidebar from './Sidebar.vue'
import TopBar from './TopBar.vue'
</script>

<template>
  <div class="flex h-screen bg-gray-50 dark:bg-rd-scaffold">
    <Sidebar />
    <div class="flex-1 flex flex-col overflow-hidden">
      <TopBar />
      <main class="flex-1 overflow-y-auto p-6">
        <router-view />
      </main>
    </div>
  </div>
</template>

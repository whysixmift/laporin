<script setup lang="ts">
const route = useRoute()

const navItems = [
  { name: 'Ringkasan & Metrik', path: '/admin', icon: 'M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6' },
  { name: 'Semua Laporan', path: '/admin/reports', icon: 'M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z' },
  { name: 'Kelola Pengguna', path: '/admin/users', icon: 'M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z' },
  { name: 'AI Playground', path: '/admin/ai-playground', icon: 'M13 10V3L4 14h7v7l9-11h-7z' },
  { name: 'Antrean Tugas (Jobs)', path: '/admin/jobs', icon: 'M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10' },
  { name: 'Status Sistem', path: '/admin/system', icon: 'M9 3v2m6-2v2M9 19v2m6-2v2M5 9H3m2 6H3m18-6h-2m2 6h-2M7 19h10a2 2 0 002-2V7a2 2 0 00-2-2H7a2 2 0 00-2 2v10a2 2 0 002 2zM9 9h6v6H9V9z' },
]
</script>

<template>
  <div class="border-b border-border bg-surface/50 mb-8">
    <div class="max-w-6xl mx-auto px-4 sm:px-6">
      <div class="flex items-center justify-between py-3 border-b border-border/50">
        <div class="flex items-center gap-2.5">
          <div class="w-2.5 h-2.5 rounded-full bg-amber-400 animate-pulse" />
          <h1 class="text-sm font-semibold tracking-wide uppercase font-mono text-ink-primary">
            Laporin Control Center &bull; Admin
          </h1>
        </div>
        <div class="text-xs text-ink-muted font-mono hidden sm:block">
          Mode Bebas Akses Aktif &bull; Role: Superadmin
        </div>
      </div>

      <nav class="flex items-center gap-1 overflow-x-auto py-2 -mx-2 px-2 scrollbar-none">
        <NuxtLink
          v-for="item in navItems"
          :key="item.path"
          :to="item.path"
          :class="[
            'flex items-center gap-2 px-3 py-1.5 rounded-md text-xs sm:text-sm font-medium whitespace-nowrap transition-all',
            route.path === item.path
              ? 'bg-amber-500/15 text-amber-300 border border-amber-500/30 shadow-subtle'
              : 'text-ink-secondary hover:text-ink-primary hover:bg-surface-elevated'
          ]"
        >
          <svg class="w-4 h-4 opacity-75" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" :d="item.icon" />
          </svg>
          <span>{{ item.name }}</span>
        </NuxtLink>
      </nav>
    </div>
  </div>
</template>

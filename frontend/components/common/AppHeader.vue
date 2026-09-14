<script setup lang="ts">
const { isAuthenticated, isAdmin, authState, logout } = useAuth()
const route = useRoute()
</script>

<template>
  <header class="border-b border-border bg-canvas/90 backdrop-blur sticky top-0 z-40">
    <div class="max-w-6xl mx-auto px-4 sm:px-6 h-14 flex items-center justify-between">
      <!-- Brand / Logo -->
      <NuxtLink to="/" class="flex items-center gap-2.5 group">
        <div class="w-7 h-7 rounded bg-surface-elevated border border-border-strong flex items-center justify-center text-accent-400 font-mono text-xs font-bold shadow-subtle group-hover:border-accent-500/50 transition-colors">
          L
        </div>
        <div class="flex items-baseline gap-1.5">
          <span class="font-bold tracking-tight text-ink-primary text-base">laporin</span>
          <span class="text-[10px] uppercase font-mono tracking-widest text-ink-muted hidden sm:inline">PKL Engine</span>
        </div>
      </NuxtLink>

      <!-- Navigation Links -->
      <nav class="flex items-center gap-2 sm:gap-4">
        <template v-if="isAuthenticated">
          <NuxtLink
            v-if="isAdmin"
            to="/admin"
            :class="[
              'text-xs sm:text-sm px-3 py-1.5 rounded-md transition-colors font-medium inline-flex items-center gap-1.5',
              route.path.startsWith('/admin')
                ? 'bg-amber-500/10 text-amber-400 border border-amber-500/30'
                : 'text-amber-400/90 hover:text-amber-300 hover:bg-amber-500/10'
            ]"
          >
            <span class="w-2 h-2 rounded-full bg-amber-400 animate-pulse" />
            <span>Admin</span>
          </NuxtLink>

          <NuxtLink
            to="/dashboard"
            :class="[
              'text-xs sm:text-sm px-3 py-1.5 rounded-md transition-colors font-medium',
              route.path === '/dashboard'
                ? 'bg-surface-elevated text-ink-primary border border-border'
                : 'text-ink-secondary hover:text-ink-primary'
            ]"
          >
            Laporan Saya
          </NuxtLink>

          <NuxtLink
            to="/reports/new"
            class="text-xs sm:text-sm px-3 py-1.5 rounded-md bg-accent-600 hover:bg-accent-500 text-white font-medium transition-colors inline-flex items-center gap-1.5 shadow-subtle"
          >
            <svg class="w-3.5 h-3.5" viewBox="0 0 20 20" fill="currentColor">
              <path fill-rule="evenodd" d="M10 3a1 1 0 011 1v5h5a1 1 0 110 2h-5v5a1 1 0 11-2 0v-5H4a1 1 0 110-2h5V4a1 1 0 011-1z" clip-rule="evenodd" />
            </svg>
            <span class="hidden sm:inline">Buat Laporan</span>
            <span class="sm:hidden">Baru</span>
          </NuxtLink>

          <div class="h-4 w-px bg-border mx-1" />


          <!-- User dropdown / Logout -->
          <div class="flex items-center gap-2">
            <span class="text-xs text-ink-muted font-mono hidden md:inline truncate max-w-[140px]" :title="authState.email || ''">
              {{ authState.email }}
            </span>
            <button
              type="button"
              class="text-xs text-ink-secondary hover:text-ink-primary px-2.5 py-1.5 rounded hover:bg-surface-elevated transition-colors"
              title="Keluar akun"
              @click="logout"
            >
              Keluar
            </button>
          </div>
        </template>

        <template v-else>
          <NuxtLink
            to="/login"
            class="text-xs sm:text-sm px-3 py-1.5 rounded-md text-ink-secondary hover:text-ink-primary font-medium transition-colors"
          >
            Masuk
          </NuxtLink>
          <NuxtLink
            to="/register"
            class="text-xs sm:text-sm px-3.5 py-1.5 rounded-md bg-surface-elevated hover:bg-surface-hover text-ink-primary border border-border font-medium transition-colors"
          >
            Daftar
          </NuxtLink>
        </template>
      </nav>
    </div>
  </header>
</template>

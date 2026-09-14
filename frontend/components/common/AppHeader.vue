<script setup lang="ts">
const { isAuthenticated, isAdmin, authState, logout } = useAuth()
const route = useRoute()
const isMobileMenuOpen = ref(false)

// Close mobile menu when navigating
watch(() => route.fullPath, () => {
  isMobileMenuOpen.value = false
})

const handleLogout = async () => {
  isMobileMenuOpen.value = false
  await logout()
}
</script>

<template>
  <header class="border-b border-border/40 bg-canvas/90 backdrop-blur-md sticky top-0 z-40 transition-colors">
    <div class="max-w-5xl mx-auto px-4 sm:px-6 h-14 flex items-center justify-between">
      <!-- Brand -->
      <NuxtLink to="/" class="flex items-center gap-2 group select-none">
        <span class="font-serif text-lg sm:text-xl font-normal tracking-tight text-ink-primary group-hover:text-white transition-colors">
          Laporin<span class="text-accent-500 font-sans">.</span>
        </span>
      </NuxtLink>

      <!-- Desktop Navigation -->
      <nav class="hidden md:flex items-center gap-6 text-xs">
        <template v-if="isAuthenticated">
          <NuxtLink
            to="/dashboard"
            :class="[
              'transition-colors font-medium',
              route.path === '/dashboard'
                ? 'text-ink-primary'
                : 'text-ink-secondary hover:text-ink-primary'
            ]"
          >
            Laporan Saya
          </NuxtLink>

          <NuxtLink
            v-if="isAdmin"
            to="/admin"
            :class="[
              'transition-colors font-mono font-medium text-[11px]',
              route.path.startsWith('/admin')
                ? 'text-ochre-400'
                : 'text-ink-muted hover:text-ochre-400'
            ]"
          >
            Admin
          </NuxtLink>

          <NuxtLink
            to="/reports/new"
            class="px-3 py-1.5 rounded bg-ink-primary text-canvas hover:bg-white font-medium transition-all"
          >
            + Buat Laporan
          </NuxtLink>

          <!-- Account / Logout -->
          <div class="flex items-center gap-3 pl-2 border-l border-border/60">
            <span class="text-[11px] font-mono text-ink-muted truncate max-w-[130px]" :title="authState.email || ''">
              {{ authState.email }}
            </span>
            <button
              type="button"
              class="text-ink-muted hover:text-danger-400 text-xs transition-colors cursor-pointer"
              title="Keluar"
              @click="handleLogout"
            >
              Keluar
            </button>
          </div>
        </template>

        <template v-else>
          <NuxtLink
            to="/login"
            class="text-ink-secondary hover:text-ink-primary font-medium transition-colors"
          >
            Masuk
          </NuxtLink>
          <NuxtLink
            to="/register"
            class="px-3.5 py-1.5 rounded bg-ink-primary text-canvas hover:bg-white font-medium transition-all"
          >
            Mulai
          </NuxtLink>
        </template>
      </nav>

      <!-- Mobile Toggle -->
      <div class="flex items-center gap-2 md:hidden">
        <NuxtLink
          v-if="!isAuthenticated"
          to="/register"
          class="text-xs px-2.5 py-1 rounded bg-ink-primary text-canvas font-medium"
        >
          Mulai
        </NuxtLink>
        <NuxtLink
          v-else
          to="/reports/new"
          class="text-xs px-2.5 py-1 rounded bg-ink-primary text-canvas font-medium"
        >
          + Baru
        </NuxtLink>

        <button
          type="button"
          class="p-1.5 text-ink-secondary hover:text-ink-primary transition-colors"
          aria-label="Toggle menu"
          @click="isMobileMenuOpen = !isMobileMenuOpen"
        >
          <svg v-if="!isMobileMenuOpen" class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <line x1="4" y1="8" x2="20" y2="8" stroke-width="1.5" stroke-linecap="round" />
            <line x1="4" y1="16" x2="20" y2="16" stroke-width="1.5" stroke-linecap="round" />
          </svg>
          <svg v-else class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <line x1="6" y1="6" x2="18" y2="18" stroke-width="1.5" stroke-linecap="round" />
            <line x1="6" y1="18" x2="18" y2="6" stroke-width="1.5" stroke-linecap="round" />
          </svg>
        </button>
      </div>
    </div>

    <!-- Mobile Drawer -->
    <div
      v-if="isMobileMenuOpen"
      class="md:hidden border-b border-border bg-surface px-4 py-4 space-y-3 text-xs"
    >
      <template v-if="isAuthenticated">
        <div class="pb-2 border-b border-border/60 text-[11px] font-mono text-ink-muted flex items-center justify-between">
          <span class="truncate">{{ authState.email }}</span>
          <span v-if="isAdmin" class="text-ochre-400">ADMIN</span>
        </div>

        <NuxtLink
          to="/dashboard"
          class="block py-1.5 text-ink-primary font-medium"
        >
          Laporan Saya
        </NuxtLink>

        <NuxtLink
          to="/reports/new"
          class="block py-1.5 text-accent-400 font-medium"
        >
          + Buat Laporan Baru
        </NuxtLink>

        <NuxtLink
          v-if="isAdmin"
          to="/admin"
          class="block py-1.5 text-ochre-400 font-mono"
        >
          Admin Panel
        </NuxtLink>

        <div class="pt-2 border-t border-border/60">
          <button
            type="button"
            class="text-danger-400 text-xs py-1"
            @click="handleLogout"
          >
            Keluar dari Akun
          </button>
        </div>
      </template>

      <template v-else>
        <NuxtLink
          to="/login"
          class="block py-1.5 text-ink-primary"
        >
          Masuk
        </NuxtLink>
        <NuxtLink
          to="/register"
          class="block py-1.5 text-accent-400 font-medium"
        >
          Daftar Akun Baru
        </NuxtLink>
      </template>
    </div>
  </header>
</template>


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
  <header class="border-b border-border bg-canvas/95 backdrop-blur sticky top-0 z-40">
    <div class="max-w-6xl mx-auto px-4 sm:px-6 h-14 flex items-center justify-between">
      <!-- Brand / Logo -->
      <NuxtLink to="/" class="flex items-center gap-2.5 group shrink-0">
        <div class="w-7 h-7 rounded bg-surface-elevated border border-border-strong flex items-center justify-center text-accent-400 font-mono text-xs font-bold shadow-subtle group-hover:border-accent-500/50 transition-colors">
          L
        </div>
        <div class="flex items-baseline gap-1.5">
          <span class="font-bold tracking-tight text-ink-primary text-base">laporin</span>
          <span class="text-[10px] uppercase font-mono tracking-widest text-ink-muted hidden sm:inline">PKL Engine</span>
        </div>
      </NuxtLink>

      <!-- Desktop Navigation Links -->
      <nav class="hidden md:flex items-center gap-2 sm:gap-4">
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
            <span>Buat Laporan</span>
          </NuxtLink>

          <div class="h-4 w-px bg-border mx-1" />

          <!-- User dropdown / Logout -->
          <div class="flex items-center gap-2">
            <span class="text-xs text-ink-muted font-mono truncate max-w-[140px]" :title="authState.email || ''">
              {{ authState.email }}
            </span>
            <button
              type="button"
              class="text-xs text-ink-secondary hover:text-ink-primary px-2.5 py-1.5 rounded hover:bg-surface-elevated transition-colors"
              title="Keluar akun"
              @click="handleLogout"
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

      <!-- Mobile Right Bar: Quick Action + Hamburger -->
      <div class="flex items-center gap-2 md:hidden">
        <NuxtLink
          v-if="isAuthenticated"
          to="/reports/new"
          class="text-xs px-2.5 py-1.5 rounded-md bg-accent-600 hover:bg-accent-500 text-white font-medium inline-flex items-center gap-1 shadow-subtle"
        >
          <svg class="w-3.5 h-3.5" viewBox="0 0 20 20" fill="currentColor">
            <path fill-rule="evenodd" d="M10 3a1 1 0 011 1v5h5a1 1 0 110 2h-5v5a1 1 0 11-2 0v-5H4a1 1 0 110-2h5V4a1 1 0 011-1z" clip-rule="evenodd" />
          </svg>
          <span>Baru</span>
        </NuxtLink>

        <NuxtLink
          v-if="!isAuthenticated"
          to="/login"
          class="text-xs px-2.5 py-1.5 rounded-md bg-surface-elevated text-ink-primary border border-border font-medium"
        >
          Masuk
        </NuxtLink>

        <!-- Hamburger Toggle Button -->
        <button
          type="button"
          class="p-2 rounded-lg text-ink-secondary hover:text-ink-primary bg-surface hover:bg-surface-elevated border border-border transition-colors"
          aria-label="Menu Navigasi"
          @click="isMobileMenuOpen = !isMobileMenuOpen"
        >
          <svg v-if="!isMobileMenuOpen" class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
          </svg>
          <svg v-else class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </div>
    </div>

    <!-- Mobile Drawer / Dropdown Menu -->
    <Transition
      enter-active-class="transition duration-200 ease-out"
      enter-from-class="opacity-0 -translate-y-2"
      enter-to-class="opacity-100 translate-y-0"
      leave-active-class="transition duration-150 ease-in"
      leave-from-class="opacity-100 translate-y-0"
      leave-to-class="opacity-0 -translate-y-2"
    >
      <div
        v-if="isMobileMenuOpen"
        class="md:hidden border-b border-border bg-surface-elevated/98 px-4 py-5 shadow-elevated space-y-4 max-h-[80vh] overflow-y-auto"
      >
        <!-- User Badge if logged in -->
        <div v-if="isAuthenticated" class="p-3 rounded-lg bg-surface border border-border-subtle flex items-center justify-between">
          <div class="space-y-0.5 truncate mr-2">
            <div class="text-[11px] text-ink-muted font-mono">Akun Masuk</div>
            <div class="text-xs font-semibold text-ink-primary truncate">{{ authState.email }}</div>
          </div>
          <span
            v-if="isAdmin"
            class="px-2 py-0.5 rounded text-[10px] font-mono bg-amber-500/20 text-amber-300 border border-amber-500/40 shrink-0"
          >
            Admin
          </span>
        </div>

        <!-- Links List -->
        <div class="flex flex-col space-y-1">
          <NuxtLink
            v-if="isAdmin"
            to="/admin"
            class="flex items-center gap-2.5 px-3 py-2.5 rounded-lg text-xs font-medium text-amber-400 bg-amber-500/10 border border-amber-500/20"
          >
            <span class="w-2 h-2 rounded-full bg-amber-400 animate-pulse" />
            <span>Admin Control Center</span>
          </NuxtLink>

          <NuxtLink
            to="/dashboard"
            class="flex items-center gap-2.5 px-3 py-2.5 rounded-lg text-xs font-medium text-ink-primary hover:bg-surface transition-colors"
          >
            <svg class="w-4 h-4 text-ink-muted" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6" />
            </svg>
            <span>Daftar Laporan Saya</span>
          </NuxtLink>

          <NuxtLink
            to="/reports/new"
            class="flex items-center gap-2.5 px-3 py-2.5 rounded-lg text-xs font-medium text-accent-400 hover:bg-surface transition-colors"
          >
            <svg class="w-4 h-4 text-accent-400" viewBox="0 0 20 20" fill="currentColor">
              <path fill-rule="evenodd" d="M10 3a1 1 0 011 1v5h5a1 1 0 110 2h-5v5a1 1 0 11-2 0v-5H4a1 1 0 110-2h5V4a1 1 0 011-1z" clip-rule="evenodd" />
            </svg>
            <span>Buat Laporan Baru</span>
          </NuxtLink>
        </div>

        <!-- Direct Admin Support & Manual Payment -->
        <div class="pt-3 border-t border-border space-y-2">
          <div class="text-[11px] text-ink-muted font-medium">Bantuan & Pembayaran Manual Admin:</div>
          <div class="grid grid-cols-2 gap-2">
            <a
              href="https://wa.me/6285117206413"
              target="_blank"
              rel="noopener noreferrer"
              class="flex items-center justify-center gap-1.5 p-2 rounded bg-emerald-500/10 text-emerald-400 border border-emerald-500/20 text-[11px] font-medium"
            >
              <span>WA 1</span>
            </a>
            <a
              href="https://wa.me/6288809028653"
              target="_blank"
              rel="noopener noreferrer"
              class="flex items-center justify-center gap-1.5 p-2 rounded bg-emerald-500/10 text-emerald-400 border border-emerald-500/20 text-[11px] font-medium"
            >
              <span>WA 2</span>
            </a>
          </div>
        </div>

        <!-- Logout / Auth CTA -->
        <div class="pt-2 border-t border-border">
          <button
            v-if="isAuthenticated"
            type="button"
            class="w-full py-2.5 px-3 rounded-lg bg-surface text-danger-400 hover:bg-danger-500/10 text-xs font-medium transition-colors text-center border border-border"
            @click="handleLogout"
          >
            Keluar dari Akun
          </button>
          <div v-else class="grid grid-cols-2 gap-2">
            <NuxtLink
              to="/login"
              class="py-2 px-3 rounded-lg text-xs font-medium text-center bg-surface border border-border text-ink-primary"
            >
              Masuk
            </NuxtLink>
            <NuxtLink
              to="/register"
              class="py-2 px-3 rounded-lg text-xs font-medium text-center bg-accent-600 text-white"
            >
              Daftar
            </NuxtLink>
          </div>
        </div>
      </div>
    </Transition>
  </header>
</template>

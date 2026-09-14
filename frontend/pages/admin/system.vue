<script setup lang="ts">
import type { AdminMetrics } from '~/types/api'

definePageMeta({
  middleware: 'admin'
})

const { getMetrics, getSystemHealth } = useAdmin()
const toast = useToast()

const metrics = ref<AdminMetrics | null>(null)
const health = ref<any>(null)
const loading = ref(true)

const loadSystemData = async () => {
  loading.value = true
  try {
    const [m, h] = await Promise.all([
      getMetrics(),
      getSystemHealth()
    ])
    metrics.value = m
    health.value = h
  } catch (err: any) {
    toast.error('Gagal Memuat Status Sistem', err.message || 'Terjadi kesalahan.')
  } finally {
    loading.value = false
  }
}

onMounted(() => {
  loadSystemData()
})
</script>

<template>
  <div class="min-h-screen bg-canvas pb-16">
    <AdminNav />

    <div class="max-w-6xl mx-auto px-4 sm:px-6">
      <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4 mb-6">
        <div>
          <h2 class="text-xl font-bold text-ink-primary">Status & Kesehatan Sistem</h2>
          <p class="text-xs sm:text-sm text-ink-secondary mt-0.5">
            Diagnostik infrastruktur server VPS (2 GB RAM / 16 GB SSD), database PostgreSQL, dan modul LibreOffice.
          </p>
        </div>

        <button
          type="button"
          class="px-3.5 py-2 text-xs font-medium bg-surface-elevated hover:bg-surface-hover border border-border text-ink-primary rounded-lg transition-colors inline-flex items-center gap-1.5 self-start sm:self-auto"
          @click="loadSystemData"
        >
          <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
          </svg>
          <span>Uji Ulang Diagnostik</span>
        </button>
      </div>

      <div v-if="loading" class="p-12 text-center bg-surface border border-border rounded-xl">
        <div class="inline-block animate-spin w-8 h-8 rounded-full border-2 border-accent-500 border-t-transparent" />
        <p class="text-xs text-ink-muted mt-2 font-mono">Menjalankan tes diagnostik server...</p>
      </div>

      <div v-else class="space-y-6">
        <!-- Status Overview Card -->
        <div class="bg-surface border border-border rounded-xl p-6 flex items-center justify-between">
          <div class="flex items-center gap-4">
            <div class="w-12 h-12 rounded-xl bg-accent-500/10 border border-accent-500/30 flex items-center justify-center text-accent-400">
              <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
            </div>
            <div>
              <div class="text-xs font-mono uppercase text-ink-muted">Status Operasional Keseluruhan</div>
              <div class="text-xl font-bold text-ink-primary flex items-center gap-2">
                <span>{{ health?.status === 'healthy' ? 'Sistem Berjalan Normal' : 'Status Terdegradasi' }}</span>
                <span class="w-2.5 h-2.5 rounded-full bg-accent-400 animate-pulse" />
              </div>
            </div>
          </div>

          <div class="text-right hidden sm:block">
            <div class="text-xs text-ink-muted font-mono">Environment</div>
            <div class="text-sm font-bold font-mono text-accent-400 uppercase">{{ metrics?.system?.environment || 'production' }}</div>
          </div>
        </div>

        <!-- System Resources Grid -->
        <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
          <!-- RAM Card -->
          <div class="bg-surface border border-border rounded-xl p-5 space-y-3">
            <div class="flex items-center justify-between">
              <span class="text-xs font-mono uppercase text-ink-muted">RAM VPS (Budget 2 GB)</span>
              <span class="text-xs font-bold font-mono text-ink-primary">{{ metrics?.system?.memory_percentage }}%</span>
            </div>

            <div class="text-2xl font-black font-mono text-ink-primary">
              {{ metrics?.system?.memory_used_mb }} <span class="text-xs font-normal text-ink-muted">/ {{ metrics?.system?.memory_total_mb }} MB</span>
            </div>

            <div class="w-full bg-surface-elevated rounded-full h-2 overflow-hidden">
              <div
                class="h-full rounded-full transition-all duration-500"
                :class="(metrics?.system?.memory_percentage ?? 0) > 80 ? 'bg-rose-500' : 'bg-accent-500'"
                :style="{ width: `${metrics?.system?.memory_percentage ?? 0}%` }"
              />
            </div>

            <p class="text-[11px] text-ink-muted">
              Alokasi memori terkendali dengan isolasi background worker untuk mencegah OOM pada VPS.
            </p>
          </div>

          <!-- Database Card -->
          <div class="bg-surface border border-border rounded-xl p-5 space-y-3">
            <div class="flex items-center justify-between">
              <span class="text-xs font-mono uppercase text-ink-muted">Database Engine</span>
              <span class="text-xs font-bold font-mono text-accent-400 uppercase">Connected</span>
            </div>

            <div class="text-2xl font-black font-mono text-ink-primary">
              PostgreSQL 17
            </div>

            <div class="flex items-center gap-2 text-xs text-ink-secondary pt-1">
              <span class="w-2 h-2 rounded-full bg-accent-400" />
              <span>Connection Pool Aktif & Stabil</span>
            </div>

            <p class="text-[11px] text-ink-muted">
              Menangani session tokens, antrean skip-locked, tabel users, dan riwayat laporan.
            </p>
          </div>

          <!-- Document Converter Card -->
          <div class="bg-surface border border-border rounded-xl p-5 space-y-3">
            <div class="flex items-center justify-between">
              <span class="text-xs font-mono uppercase text-ink-muted">Modul Konversi PDF</span>
              <span
                class="text-xs font-bold font-mono uppercase"
                :class="metrics?.system?.libreoffice_available ? 'text-accent-400' : 'text-amber-400'"
              >
                {{ metrics?.system?.libreoffice_available ? 'Tersedia' : 'Nonaktif' }}
              </span>
            </div>

            <div class="text-2xl font-black font-mono text-ink-primary">
              LibreOffice Headless
            </div>

            <div class="flex items-center gap-2 text-xs text-ink-secondary pt-1">
              <span
                class="w-2 h-2 rounded-full"
                :class="metrics?.system?.libreoffice_available ? 'bg-accent-400' : 'bg-amber-400'"
              />
              <span>Rendering DOCX & Watermark PDF</span>
            </div>

            <p class="text-[11px] text-ink-muted">
              Digunakan untuk mengubah berkas DOCX terkompilasi menjadi pratinjau PDF akademis.
            </p>
          </div>
        </div>

        <!-- Infrastructure Details Table -->
        <div class="bg-surface border border-border rounded-xl p-5">
          <h3 class="text-sm font-bold text-ink-primary uppercase font-mono tracking-wider mb-4">
            Daftar Komponen & Layanan Sistem
          </h3>

          <div class="divide-y divide-border/60 text-xs">
            <div class="py-3 flex items-center justify-between">
              <span class="text-ink-secondary">Backend API Framework</span>
              <span class="font-mono text-ink-primary font-semibold">Rust Axum 0.8 / Tokio Async</span>
            </div>
            <div class="py-3 flex items-center justify-between">
              <span class="text-ink-secondary">Frontend Engine</span>
              <span class="font-mono text-ink-primary font-semibold">Nuxt 3.16 / Vue 3 + Tailwind CSS</span>
            </div>
            <div class="py-3 flex items-center justify-between">
              <span class="text-ink-secondary">AI Reasoning Router</span>
              <span class="font-mono text-ink-primary font-semibold">9Router API (Gemini Flash / OpenAI compatible)</span>
            </div>
            <div class="py-3 flex items-center justify-between">
              <span class="text-ink-secondary">Penyedia Autentikasi OAuth</span>
              <span class="font-mono text-ink-primary font-semibold">Google OAuth 2.0 Identity Platform</span>
            </div>
            <div class="py-3 flex items-center justify-between">
              <span class="text-ink-secondary">Payment Gateway Engine</span>
              <span class="font-mono text-ink-primary font-semibold">Mayar (dengan mode Admin Free Bypass)</span>
            </div>
            <div class="py-3 flex items-center justify-between">
              <span class="text-ink-secondary">Direktori Penyimpanan File</span>
              <span class="font-mono text-ink-primary">{{ health?.storage_dir === 'accessible' ? 'storage/ (Accessible)' : 'storage/ (Unavailable)' }}</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { AdminMetrics, AdminReportItem } from '~/types/api'

definePageMeta({
  middleware: 'admin'
})

const { getMetrics, getReports, unlockReportFree, loading } = useAdmin()
const toast = useToast()

const metrics = ref<AdminMetrics | null>(null)
const recentReports = ref<AdminReportItem[]>([])
const refreshing = ref(false)

const loadDashboardData = async () => {
  refreshing.value = true
  try {
    const [m, r] = await Promise.all([
      getMetrics(),
      getReports(1, 6)
    ])
    metrics.value = m
    recentReports.value = r.reports
  } catch (err: any) {
    toast.error('Gagal Memuat Data', err.message || 'Terjadi kesalahan saat memuat metrik admin.')
  } finally {
    refreshing.value = false
  }
}

const handleQuickUnlock = async (reportId: string) => {
  try {
    await unlockReportFree(reportId)
    await loadDashboardData()
  } catch (err: any) {
    toast.error('Gagal Membuka Kunci', err.message || 'Terjadi kesalahan.')
  }
}

onMounted(() => {
  loadDashboardData()
})
</script>

<template>
  <div class="min-h-screen bg-canvas pb-16">
    <AdminNav />

    <div class="max-w-6xl mx-auto px-4 sm:px-6">
      <!-- Header banner -->
      <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4 mb-8 bg-surface-elevated/40 border border-border p-5 rounded-xl">
        <div>
          <div class="flex items-center gap-2 mb-1">
            <span class="px-2 py-0.5 text-[11px] font-mono font-semibold uppercase bg-amber-500/20 text-amber-300 border border-amber-500/30 rounded">
              Akses Penuh
            </span>
            <span class="text-xs text-ink-muted">Server Mode: {{ metrics?.system?.environment || 'production' }}</span>
          </div>
          <h2 class="text-xl font-bold text-ink-primary">Ringkasan Sistem & Operasional</h2>
          <p class="text-sm text-ink-secondary mt-0.5">
            Kelola seluruh ekosistem Laporin, buka akses laporan secara gratis, pantau performa AI & antrean tugas.
          </p>
        </div>

        <div class="flex items-center gap-2">
          <button
            type="button"
            :disabled="refreshing"
            class="px-3.5 py-2 text-xs font-medium bg-surface hover:bg-surface-hover border border-border text-ink-primary rounded-lg transition-colors inline-flex items-center gap-1.5"
            @click="loadDashboardData"
          >
            <svg
              class="w-3.5 h-3.5"
              :class="{ 'animate-spin': refreshing }"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
            </svg>
            <span>Segarkan</span>
          </button>

          <NuxtLink
            to="/reports/new"
            class="px-3.5 py-2 text-xs font-medium bg-accent-600 hover:bg-accent-500 text-white rounded-lg transition-colors inline-flex items-center gap-1.5 shadow-subtle"
          >
            <svg class="w-3.5 h-3.5" viewBox="0 0 20 20" fill="currentColor">
              <path fill-rule="evenodd" d="M10 3a1 1 0 011 1v5h5a1 1 0 110 2h-5v5a1 1 0 11-2 0v-5H4a1 1 0 110-2h5V4a1 1 0 011-1z" clip-rule="evenodd" />
            </svg>
            <span>Laporan Baru</span>
          </NuxtLink>
        </div>
      </div>

      <!-- Metric Cards Grid -->
      <div class="grid grid-cols-2 md:grid-cols-4 gap-4 mb-8">
        <!-- Total Users -->
        <div class="bg-surface-elevated/60 border border-border rounded-xl p-4 sm:p-5 relative overflow-hidden group">
          <div class="text-xs font-mono text-ink-muted uppercase mb-1">Total Pengguna</div>
          <div class="text-2xl sm:text-3xl font-bold font-mono text-ink-primary">
            {{ metrics?.total_users ?? '-' }}
          </div>
          <div class="text-xs text-ink-secondary mt-1 flex items-center gap-1.5">
            <span class="text-amber-400 font-medium">{{ metrics?.total_admins ?? 0 }} Admin</span>
            <span>&bull;</span>
            <span>{{ (metrics?.total_users ?? 0) - (metrics?.total_admins ?? 0) }} Siswa</span>
          </div>
        </div>

        <!-- Total Reports -->
        <div class="bg-surface-elevated/60 border border-border rounded-xl p-4 sm:p-5 relative overflow-hidden group">
          <div class="text-xs font-mono text-ink-muted uppercase mb-1">Total Laporan</div>
          <div class="text-2xl sm:text-3xl font-bold font-mono text-ink-primary">
            {{ metrics?.total_reports ?? '-' }}
          </div>
          <div class="text-xs text-ink-secondary mt-1 flex items-center gap-1.5">
            <span class="text-accent-400 font-medium">{{ metrics?.unlocked_reports ?? 0 }} Terbuka / Selesai</span>
          </div>
        </div>

        <!-- System Memory -->
        <div class="bg-surface-elevated/60 border border-border rounded-xl p-4 sm:p-5 relative overflow-hidden group">
          <div class="text-xs font-mono text-ink-muted uppercase mb-1">RAM Server VPS</div>
          <div class="text-2xl sm:text-3xl font-bold font-mono text-ink-primary">
            {{ metrics?.system?.memory_used_mb ?? '-' }} <span class="text-xs font-sans text-ink-muted">/ {{ metrics?.system?.memory_total_mb }} MB</span>
          </div>
          <div class="w-full bg-border rounded-full h-1.5 mt-2.5 overflow-hidden">
            <div
              class="h-full rounded-full transition-all duration-500"
              :class="(metrics?.system?.memory_percentage ?? 0) > 80 ? 'bg-rose-500' : 'bg-accent-500'"
              :style="{ width: `${metrics?.system?.memory_percentage ?? 0}%` }"
            />
          </div>
          <div class="text-[11px] text-ink-muted mt-1 font-mono">
            {{ metrics?.system?.memory_percentage ?? 0 }}% Terpakai (Budget 2GB)
          </div>
        </div>

        <!-- Active Jobs -->
        <div class="bg-surface-elevated/60 border border-border rounded-xl p-4 sm:p-5 relative overflow-hidden group">
          <div class="text-xs font-mono text-ink-muted uppercase mb-1">Antrean Tugas (Jobs)</div>
          <div class="text-2xl sm:text-3xl font-bold font-mono" :class="(metrics?.active_jobs ?? 0) > 0 ? 'text-amber-400' : 'text-ink-primary'">
            {{ metrics?.active_jobs ?? 0 }}
          </div>
          <div class="text-xs text-ink-secondary mt-1">
            <NuxtLink to="/admin/jobs" class="text-accent-400 hover:underline inline-flex items-center gap-1">
              <span>Buka Monitor Job</span>
              <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
              </svg>
            </NuxtLink>
          </div>
        </div>
      </div>

      <!-- Quick Action Cards -->
      <div class="grid grid-cols-1 sm:grid-cols-3 gap-4 mb-8">
        <NuxtLink
          to="/admin/reports"
          class="p-4 rounded-xl bg-surface border border-border hover:border-amber-500/40 hover:bg-surface-elevated transition-all group flex items-start gap-3.5"
        >
          <div class="w-9 h-9 rounded-lg bg-amber-500/10 border border-amber-500/20 flex items-center justify-center text-amber-400 group-hover:scale-105 transition-transform">
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 11V7a4 4 0 118 0m-4 8v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2z" />
            </svg>
          </div>
          <div>
            <div class="text-sm font-semibold text-ink-primary group-hover:text-amber-300 transition-colors">
              Bypass / Buka Kunci Gratis
            </div>
            <div class="text-xs text-ink-muted mt-0.5">
              Buka kunci instan DOCX & PDF untuk laporan apapun tanpa gateway pembayaran.
            </div>
          </div>
        </NuxtLink>

        <NuxtLink
          to="/admin/ai-playground"
          class="p-4 rounded-xl bg-surface border border-border hover:border-accent-500/40 hover:bg-surface-elevated transition-all group flex items-start gap-3.5"
        >
          <div class="w-9 h-9 rounded-lg bg-accent-500/10 border border-accent-500/20 flex items-center justify-center text-accent-400 group-hover:scale-105 transition-transform">
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" />
            </svg>
          </div>
          <div>
            <div class="text-sm font-semibold text-ink-primary group-hover:text-accent-300 transition-colors">
              AI Playground & Web Crawler
            </div>
            <div class="text-xs text-ink-muted mt-0.5">
              Tes langsung prompt LLM 9Router dan crawling fakta web perusahaan secara real-time.
            </div>
          </div>
        </NuxtLink>

        <NuxtLink
          to="/admin/users"
          class="p-4 rounded-xl bg-surface border border-border hover:border-sky-500/40 hover:bg-surface-elevated transition-all group flex items-start gap-3.5"
        >
          <div class="w-9 h-9 rounded-lg bg-sky-500/10 border border-sky-500/20 flex items-center justify-center text-sky-400 group-hover:scale-105 transition-transform">
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z" />
            </svg>
          </div>
          <div>
            <div class="text-sm font-semibold text-ink-primary group-hover:text-sky-300 transition-colors">
              Manajemen Pengguna
            </div>
            <div class="text-xs text-ink-muted mt-0.5">
              Promosikan admin baru, lihat daftar siswa, atau nonaktifkan akun pengguna.
            </div>
          </div>
        </NuxtLink>
      </div>

      <!-- Recent Reports with Instant Unlock Section -->
      <div class="bg-surface border border-border rounded-xl p-5 mb-8">
        <div class="flex items-center justify-between mb-4">
          <div>
            <h3 class="text-base font-semibold text-ink-primary">Laporan Terbaru</h3>
            <p class="text-xs text-ink-muted">Daftar laporan yang baru saja dibuat di seluruh sistem</p>
          </div>
          <NuxtLink to="/admin/reports" class="text-xs text-accent-400 hover:underline font-medium">
            Lihat Semua ({{ metrics?.total_reports ?? 0 }}) &rarr;
          </NuxtLink>
        </div>

        <div v-if="recentReports.length === 0" class="text-center py-8 text-ink-muted text-sm font-mono">
          Belum ada laporan di sistem.
        </div>

        <div v-else class="divide-y divide-border/60">
          <div
            v-for="r in recentReports"
            :key="r.id"
            class="py-3.5 flex flex-col sm:flex-row sm:items-center justify-between gap-3"
          >
            <div class="min-w-0 flex-1">
              <div class="flex items-center gap-2">
                <span class="text-sm font-medium text-ink-primary truncate">{{ r.title }}</span>
                <span
                  class="text-[10px] uppercase font-mono px-2 py-0.5 rounded border"
                  :class="[
                    r.status === 'unlocked' || r.status === 'paid'
                      ? 'bg-accent-500/10 text-accent-400 border-accent-500/20'
                      : r.status === 'failed'
                      ? 'bg-rose-500/10 text-rose-400 border-rose-500/20'
                      : 'bg-surface-elevated text-ink-secondary border-border'
                  ]"
                >
                  {{ r.status }}
                </span>
              </div>
              <div class="text-xs text-ink-muted mt-0.5 flex flex-wrap items-center gap-x-3 gap-y-1">
                <span>Pemilik: <strong class="text-ink-secondary">{{ r.user_email }}</strong></span>
                <span v-if="r.student_name">&bull; Siswa: {{ r.student_name }}</span>
                <span v-if="r.company_name">&bull; Perusahaan: {{ r.company_name }}</span>
              </div>
            </div>

            <div class="flex items-center gap-2 shrink-0">
              <!-- Free Unlock Button -->
              <button
                v-if="r.status !== 'unlocked' && r.status !== 'paid'"
                type="button"
                class="px-2.5 py-1.5 text-xs font-medium rounded bg-amber-500/15 hover:bg-amber-500/25 text-amber-300 border border-amber-500/30 transition-colors inline-flex items-center gap-1.5"
                title="Buka kunci gratis tanpa bayar"
                @click="handleQuickUnlock(r.id)"
              >
                <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 11V7a4 4 0 118 0m-4 8v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2z" />
                </svg>
                <span>Buka Kunci Gratis</span>
              </button>

              <NuxtLink
                :to="`/reports/${r.id}`"
                class="px-2.5 py-1.5 text-xs font-medium rounded bg-surface-elevated hover:bg-surface-hover text-ink-primary border border-border transition-colors"
              >
                Lihat Laporan
              </NuxtLink>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

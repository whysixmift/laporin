<script setup lang="ts">
import type { Report, ReportStatus } from '~/types/api'

const { listReports } = useReports()
const { isAuthenticated } = useAuth()
const router = useRouter()

const reports = ref<Report[]>([])
const loading = ref(true)
const error = ref('')
const activeFilter = ref<'all' | 'draft' | 'in_progress' | 'ready'>('all')

const fetchReports = async () => {
  loading.value = true
  error.value = ''
  try {
    reports.value = await listReports()
  } catch (err: unknown) {
    const apiErr = err as { message?: string; status?: number }
    if (apiErr.status === 401) {
      router.replace('/login?redirect=/dashboard')
      return
    }
    error.value = apiErr.message || 'Gagal memuat daftar laporan.'
  } finally {
    loading.value = false
  }
}

onMounted(() => {
  fetchReports()
})

const filteredReports = computed(() => {
  if (activeFilter.value === 'all') return reports.value

  if (activeFilter.value === 'draft') {
    return reports.value.filter((r) => r.status === 'draft')
  }

  if (activeFilter.value === 'in_progress') {
    return reports.value.filter((r) =>
      ['researching', 'research_completed', 'generating', 'generated'].includes(r.status)
    )
  }

  if (activeFilter.value === 'ready') {
    return reports.value.filter((r) =>
      ['preview_ready', 'payment_pending', 'paid', 'unlocked'].includes(r.status)
    )
  }

  return reports.value
})
</script>

<template>
  <div class="max-w-6xl mx-auto px-4 sm:px-6 py-8 sm:py-10 space-y-8">
    <!-- Top Header -->
    <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4 pb-6 border-b border-border">
      <div class="space-y-1">
        <h1 class="text-2xl font-bold text-ink-primary">
          Laporan PKL Saya
        </h1>
        <p class="text-xs sm:text-sm text-ink-secondary">
          Kelola dokumen laporan magang dan lanjutkan proses penyusunan.
        </p>
      </div>

      <BaseButton to="/reports/new" variant="primary" size="md">
        <template #leading>
          <svg class="w-4 h-4" viewBox="0 0 20 20" fill="currentColor">
            <path fill-rule="evenodd" d="M10 3a1 1 0 011 1v5h5a1 1 0 110 2h-5v5a1 1 0 11-2 0v-5H4a1 1 0 110-2h5V4a1 1 0 011-1z" clip-rule="evenodd" />
          </svg>
        </template>
        Buat Laporan Baru
      </BaseButton>
    </div>

    <!-- Filter Tabs -->
    <div class="flex items-center gap-2 overflow-x-auto pb-1 text-xs">
      <button
        type="button"
        :class="[
          'px-3 py-1.5 rounded-md font-medium transition-colors border',
          activeFilter === 'all'
            ? 'bg-surface-elevated text-ink-primary border-border'
            : 'bg-transparent text-ink-muted hover:text-ink-secondary border-transparent'
        ]"
        @click="activeFilter = 'all'"
      >
        Semua ({{ reports.length }})
      </button>

      <button
        type="button"
        :class="[
          'px-3 py-1.5 rounded-md font-medium transition-colors border',
          activeFilter === 'draft'
            ? 'bg-surface-elevated text-ink-primary border-border'
            : 'bg-transparent text-ink-muted hover:text-ink-secondary border-transparent'
        ]"
        @click="activeFilter = 'draft'"
      >
        Draf
      </button>

      <button
        type="button"
        :class="[
          'px-3 py-1.5 rounded-md font-medium transition-colors border',
          activeFilter === 'in_progress'
            ? 'bg-surface-elevated text-ink-primary border-border'
            : 'bg-transparent text-ink-muted hover:text-ink-secondary border-transparent'
        ]"
        @click="activeFilter = 'in_progress'"
      >
        Sedang Diproses
      </button>

      <button
        type="button"
        :class="[
          'px-3 py-1.5 rounded-md font-medium transition-colors border',
          activeFilter === 'ready'
            ? 'bg-surface-elevated text-ink-primary border-border'
            : 'bg-transparent text-ink-muted hover:text-ink-secondary border-transparent'
        ]"
        @click="activeFilter = 'ready'"
      >
        Pratinjau / Selesai
      </button>
    </div>

    <!-- Error State -->
    <BaseAlert v-if="error" type="error" dismissible @dismiss="fetchReports">
      {{ error }}
    </BaseAlert>

    <!-- Loading State -->
    <div v-if="loading" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-5">
      <div v-for="i in 3" :key="i" class="p-5 rounded-lg border border-border bg-surface animate-pulse space-y-4">
        <div class="h-4 bg-surface-elevated rounded w-1/3" />
        <div class="h-5 bg-surface-elevated rounded w-3/4" />
        <div class="h-4 bg-surface-elevated rounded w-1/2" />
        <div class="pt-4 border-t border-border-subtle h-8 bg-surface-elevated rounded" />
      </div>
    </div>

    <!-- Empty State -->
    <div
      v-else-if="filteredReports.length === 0"
      class="py-16 px-6 rounded-xl border border-dashed border-border bg-surface/40 text-center space-y-4 max-w-lg mx-auto"
    >
      <div class="w-12 h-12 rounded-full bg-surface-elevated border border-border mx-auto flex items-center justify-center text-ink-muted">
        <svg class="w-6 h-6" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
          <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" />
          <polyline points="14 2 14 8 20 8" />
          <line x1="16" y1="13" x2="8" y2="13" />
          <line x1="16" y1="17" x2="8" y2="17" />
          <polyline points="10 9 9 9 8 9" />
        </svg>
      </div>

      <div class="space-y-1">
        <h3 class="text-base font-semibold text-ink-primary">
          Belum ada laporan PKL.
        </h3>
        <p class="text-xs text-ink-secondary leading-relaxed">
          Mulai dari data diri dan catatan kegiatan magang kamu untuk menyusun draf laporan pertama.
        </p>
      </div>

      <div class="pt-2">
        <BaseButton to="/reports/new" variant="primary" size="md">
          Mulai Buat Laporan Baru
        </BaseButton>
      </div>
    </div>

    <!-- Report List Grid -->
    <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-5">
      <ReportCard
        v-for="report in filteredReports"
        :key="report.id"
        :report="report"
      />
    </div>
  </div>
</template>

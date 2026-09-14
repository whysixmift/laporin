<script setup lang="ts">
import type { Report } from '~/types/api'

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
    error.value = apiErr.message || 'Gagal memuat daftar dokumen.'
  } finally {
    loading.value = false
  }
}

onMounted(() => {
  fetchReports()
})

const filteredReports = computed(() => {
  if (activeFilter.value === 'all') return reports.value
  if (activeFilter.value === 'draft') return reports.value.filter((r) => r.status === 'draft')
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
  <div class="max-w-6xl mx-auto px-4 sm:px-6 py-10 space-y-10">
    <!-- Top Workspace Banner -->
    <div class="p-8 rounded-2xl bg-gradient-to-r from-[#121722] via-[#141a26] to-[#0f131a] border border-border-strong shadow-doc flex flex-col md:flex-row md:items-center justify-between gap-6 relative overflow-hidden">
      <div class="space-y-2 relative z-10">
        <div class="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-emerald-500/10 border border-emerald-500/30 text-xs text-emerald-400 font-mono">
          ATELIER DOKUMEN PKL
        </div>
        <h1 class="text-2xl sm:text-3xl font-extrabold text-ink-primary font-sans tracking-tight">
          Meja Kerja Dokumen
        </h1>
        <p class="text-xs sm:text-sm text-ink-secondary max-w-xl">
          Kelola berkas laporan magang, pantau riset bab, dan akses pratinjau dokumen siap cetak.
        </p>
      </div>

      <div class="relative z-10 shrink-0">
        <BaseButton to="/reports/new" variant="primary" size="lg" class="shadow-elevated font-semibold">
          <template #leading>
            <svg class="w-4 h-4" viewBox="0 0 20 20" fill="currentColor">
              <path fill-rule="evenodd" d="M10 3a1 1 0 011 1v5h5a1 1 0 110 2h-5v5a1 1 0 11-2 0v-5H4a1 1 0 110-2h5V4a1 1 0 011-1z" clip-rule="evenodd" />
            </svg>
          </template>
          Susun Dokumen Baru
        </BaseButton>
      </div>
    </div>

    <!-- Filter Pills -->
    <div class="flex items-center justify-between gap-4 flex-wrap border-b border-border pb-4">
      <div class="flex items-center gap-2 overflow-x-auto pb-1 text-xs">
        <button
          type="button"
          class="px-3.5 py-1.5 rounded-lg font-mono font-bold transition-all border cursor-pointer"
          :class="activeFilter === 'all'
            ? 'bg-emerald-500 text-white border-emerald-400 shadow-subtle'
            : 'bg-surface hover:bg-surface-elevated text-ink-secondary border-border'"
          @click="activeFilter = 'all'"
        >
          Semua ({{ reports.length }})
        </button>

        <button
          type="button"
          class="px-3.5 py-1.5 rounded-lg font-mono font-bold transition-all border cursor-pointer"
          :class="activeFilter === 'draft'
            ? 'bg-emerald-500 text-white border-emerald-400 shadow-subtle'
            : 'bg-surface hover:bg-surface-elevated text-ink-secondary border-border'"
          @click="activeFilter = 'draft'"
        >
          Draf
        </button>

        <button
          type="button"
          class="px-3.5 py-1.5 rounded-lg font-mono font-bold transition-all border cursor-pointer"
          :class="activeFilter === 'in_progress'
            ? 'bg-emerald-500 text-white border-emerald-400 shadow-subtle'
            : 'bg-surface hover:bg-surface-elevated text-ink-secondary border-border'"
          @click="activeFilter = 'in_progress'"
        >
          Dalam Proses
        </button>

        <button
          type="button"
          class="px-3.5 py-1.5 rounded-lg font-mono font-bold transition-all border cursor-pointer"
          :class="activeFilter === 'ready'
            ? 'bg-emerald-500 text-white border-emerald-400 shadow-subtle'
            : 'bg-surface hover:bg-surface-elevated text-ink-secondary border-border'"
          @click="activeFilter = 'ready'"
        >
          Pratinjau & Selesai
        </button>
      </div>

      <div class="text-xs font-mono text-ink-muted">
        Total: {{ filteredReports.length }} Berkas
      </div>
    </div>

    <!-- Error Alert -->
    <BaseAlert v-if="error" type="error" dismissible @dismiss="fetchReports">
      {{ error }}
    </BaseAlert>

    <!-- Loading Skeletons -->
    <div v-if="loading" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
      <div v-for="i in 3" :key="i" class="p-6 rounded-2xl border border-border bg-surface animate-pulse space-y-4">
        <div class="h-4 bg-surface-elevated rounded w-1/3" />
        <div class="h-6 bg-surface-elevated rounded w-3/4" />
        <div class="h-16 bg-surface-elevated rounded" />
        <div class="h-8 bg-surface-elevated rounded w-full" />
      </div>
    </div>

    <!-- Empty Folio State -->
    <div
      v-else-if="filteredReports.length === 0"
      class="py-20 px-6 rounded-2xl border-2 border-dashed border-border-strong bg-[#0f131a] text-center space-y-5 max-w-lg mx-auto shadow-doc"
    >
      <div class="w-16 h-16 rounded-2xl bg-surface-elevated border border-border mx-auto flex items-center justify-center text-emerald-400 shadow-elevated">
        <svg class="w-8 h-8" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
          <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" />
          <polyline points="14 2 14 8 20 8" />
          <line x1="16" y1="13" x2="8" y2="13" />
          <line x1="16" y1="17" x2="8" y2="17" />
        </svg>
      </div>

      <div class="space-y-1.5">
        <h3 class="text-lg font-bold text-ink-primary font-sans">
          Meja Dokumen Masih Kosong
        </h3>
        <p class="text-xs text-ink-secondary leading-relaxed font-serif max-w-sm mx-auto">
          Mulai masukkan data tempat magang dan catatan kegiatan untuk menyusun dokumen laporan PKL pertamamu.
        </p>
      </div>

      <div class="pt-2">
        <BaseButton to="/reports/new" variant="primary" size="md" class="font-semibold">
          Susun Laporan PKL Sekarang
        </BaseButton>
      </div>
    </div>

    <!-- 3D Folio Grid -->
    <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
      <ReportCard
        v-for="report in filteredReports"
        :key="report.id"
        :report="report"
      />
    </div>
  </div>
</template>

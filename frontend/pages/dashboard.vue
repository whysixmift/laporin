<script setup lang="ts">
import type { Report } from '~/types/api'

const { listReports } = useReports()
const { isAuthenticated } = useAuth()
const router = useRouter()

const reports = ref<Report[]>([])
const loading = ref(true)
const error = ref('')

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

const getStatusLabel = (status: string) => {
  switch (status) {
    case 'draft': return 'Draf'
    case 'researching': return 'Riset'
    case 'research_completed': return 'Riset Selesai'
    case 'generating': return 'Menyusun...'
    case 'preview_ready': return 'Pratinjau'
    case 'payment_pending': return 'Menunggu Bayar'
    case 'paid':
    case 'unlocked': return 'Terbuka'
    case 'failed': return 'Terkendala'
    default: return status
  }
}

const getActionLabel = (status: string) => {
  switch (status) {
    case 'draft': return 'Mulai Riset →'
    case 'researching': return 'Pantau Riset →'
    case 'research_completed': return 'Susun Laporan →'
    case 'generating': return 'Lihat Status →'
    case 'preview_ready': return 'Pratinjau & Unduh →'
    case 'payment_pending': return 'Buka Kunci →'
    case 'paid':
    case 'unlocked': return 'Unduh DOCX →'
    default: return 'Buka →'
  }
}

const formatDate = (dateStr?: string) => {
  if (!dateStr) return '—'
  return new Intl.DateTimeFormat('id-ID', {
    day: 'numeric',
    month: 'short',
    year: 'numeric'
  }).format(new Date(dateStr))
}

const activeReport = computed(() => {
  if (!reports.value.length) return null
  // Return the first unfinished or most recent report
  return reports.value[0]
})
</script>

<template>
  <div class="max-w-5xl mx-auto px-4 sm:px-6 py-12 sm:py-16 space-y-12">
    <!-- Top Header -->
    <div class="flex flex-col sm:flex-row sm:items-baseline justify-between gap-4 pb-6 border-b border-border/40">
      <div class="space-y-1">
        <h1 class="text-2xl sm:text-3xl font-serif font-normal text-ink-primary">
          Laporan Saya
        </h1>
        <p class="text-xs text-ink-muted">
          Workspace penyusunan dan arsip dokumen PKL kamu.
        </p>
      </div>

      <NuxtLink
        to="/reports/new"
        class="inline-flex items-center justify-center px-4 py-2 rounded bg-ink-primary text-canvas hover:bg-white text-xs font-medium transition-all text-center w-full sm:w-auto"
      >
        + Buat Laporan Baru
      </NuxtLink>
    </div>

    <!-- Error Alert -->
    <BaseAlert v-if="error" type="error" dismissible @dismiss="fetchReports">
      {{ error }}
    </BaseAlert>

    <!-- Loading State -->
    <div v-if="loading" class="py-16 text-center text-xs font-mono text-ink-muted">
      Memuat daftar dokumen...
    </div>

    <!-- Empty State -->
    <div
      v-else-if="reports.length === 0"
      class="py-20 text-center space-y-4 max-w-sm mx-auto"
    >
      <div class="space-y-1">
        <h3 class="text-base font-medium text-ink-primary">
          Belum ada laporan
        </h3>
        <p class="text-xs text-ink-muted leading-relaxed">
          Mulai masukkan data tempat magang dan catatan kegiatan untuk membuat laporan PKL pertamamu.
        </p>
      </div>

      <div>
        <NuxtLink
          to="/reports/new"
          class="inline-flex items-center justify-center px-4 py-2 rounded bg-ink-primary text-canvas hover:bg-white text-xs font-medium transition-all"
        >
          Susun Laporan Sekarang
        </NuxtLink>
      </div>
    </div>

    <!-- Workspace Content -->
    <div v-else class="space-y-12">
      <!-- Active Report Callout (If exists) -->
      <div
        v-if="activeReport"
        class="p-6 bg-surface rounded border border-border/60 space-y-4"
      >
        <div class="flex items-center justify-between text-xs">
          <span class="font-mono text-[11px] text-ink-muted uppercase tracking-wider">Dokumen Aktif</span>
          <span class="font-mono text-[11px] text-ink-secondary">{{ getStatusLabel(activeReport.status) }}</span>
        </div>

        <div class="flex flex-col md:flex-row md:items-center justify-between gap-4">
          <div class="space-y-1">
            <h2 class="text-lg font-serif font-normal text-ink-primary">
              {{ activeReport.title || 'Laporan PKL' }}
            </h2>
            <div class="text-xs text-ink-muted flex items-center gap-2">
              <span>{{ activeReport.internship?.company_name || 'Instansi DU/DI' }}</span>
              <span class="text-ink-faint">·</span>
              <span>Diperbarui {{ formatDate(activeReport.updated_at || activeReport.created_at) }}</span>
            </div>
          </div>

          <NuxtLink
            :to="`/reports/${activeReport.id}`"
            class="inline-flex items-center justify-center px-4 py-2 rounded bg-accent-600 hover:bg-accent-500 text-white text-xs font-medium transition-all shrink-0"
          >
            {{ getActionLabel(activeReport.status) }}
          </NuxtLink>
        </div>
      </div>

      <!-- All Reports Table / List -->
      <div class="space-y-4">
        <div class="flex items-center justify-between text-xs border-b border-border/40 pb-2">
          <span class="font-mono text-[11px] text-ink-muted uppercase tracking-wider">Semua Berkas ({{ reports.length }})</span>
        </div>

        <div class="divide-y divide-border/30">
          <NuxtLink
            v-for="report in reports"
            :key="report.id"
            :to="`/reports/${report.id}`"
            class="py-4 flex flex-col sm:flex-row sm:items-center justify-between gap-3 group hover:bg-surface/40 -mx-3 px-3 rounded transition-colors"
          >
            <div class="space-y-1 min-w-0">
              <h3 class="text-sm font-medium text-ink-primary group-hover:text-white transition-colors truncate">
                {{ report.title || 'Laporan PKL' }}
              </h3>
              <div class="text-xs text-ink-muted flex items-center gap-2 font-mono text-[11px]">
                <span>{{ report.internship?.company_name || 'Instansi' }}</span>
                <span class="text-ink-faint">·</span>
                <span>{{ formatDate(report.created_at) }}</span>
              </div>
            </div>

            <div class="flex items-center justify-between sm:justify-end gap-4 shrink-0 text-xs">
              <span class="font-mono text-[11px] text-ink-muted">
                {{ getStatusLabel(report.status) }}
              </span>
              <span class="text-ink-muted group-hover:text-ink-primary transition-colors">
                →
              </span>
            </div>
          </NuxtLink>
        </div>
      </div>
    </div>
  </div>
</template>


<script setup lang="ts">
import type { AdminReportItem, ReportStatus } from '~/types/api'

definePageMeta({
  middleware: 'admin'
})

const { getReports, unlockReportFree, regenerateReport, deleteReport } = useAdmin()
const toast = useToast()

const reports = ref<AdminReportItem[]>([])
const total = ref(0)
const page = ref(1)
const totalPages = ref(1)
const limit = ref(15)
const statusFilter = ref<string>('')
const searchQuery = ref('')
const loading = ref(true)
const actionInProgress = ref<string | null>(null)

const statuses: { label: string; value: string }[] = [
  { label: 'Semua Status', value: '' },
  { label: 'Draft', value: 'draft' },
  { label: 'Riset Sedang Berjalan', value: 'researching' },
  { label: 'Riset Selesai', value: 'research_completed' },
  { label: 'Generasi Dokumen', value: 'generating' },
  { label: 'Pratinjau Siap', value: 'preview_ready' },
  { label: 'Menunggu Bayar', value: 'payment_pending' },
  { label: 'Sudah Bayar', value: 'paid' },
  { label: 'Terbuka (Unlocked)', value: 'unlocked' },
  { label: 'Gagal (Failed)', value: 'failed' }
]

const loadReports = async () => {
  loading.value = true
  try {
    const data = await getReports(
      page.value,
      limit.value,
      statusFilter.value || undefined,
      searchQuery.value.trim() || undefined
    )
    reports.value = data.reports
    total.value = data.total
    totalPages.value = data.total_pages || 1
  } catch (err: any) {
    toast.error('Gagal Memuat Laporan', err.message || 'Terjadi kesalahan.')
  } finally {
    loading.value = false
  }
}

const handleSearch = () => {
  page.value = 1
  loadReports()
}

const handleStatusChange = () => {
  page.value = 1
  loadReports()
}

const handlePageChange = (newPage: number) => {
  if (newPage < 1 || newPage > totalPages.value) return
  page.value = newPage
  loadReports()
}

const handleUnlockFree = async (reportId: string) => {
  actionInProgress.value = reportId
  try {
    await unlockReportFree(reportId)
    await loadReports()
  } catch (err: any) {
    toast.error('Gagal Membuka Kunci', err.message || 'Terjadi kesalahan.')
  } finally {
    actionInProgress.value = null
  }
}

const handleRegenerate = async (reportId: string) => {
  actionInProgress.value = reportId
  try {
    await regenerateReport(reportId)
    await loadReports()
  } catch (err: any) {
    toast.error('Gagal Regenerasi', err.message || 'Terjadi kesalahan.')
  } finally {
    actionInProgress.value = null
  }
}

const handleDelete = async (reportId: string, title: string) => {
  if (!confirm(`Apakah Anda yakin ingin menghapus laporan "${title}"? Tindakan ini tidak dapat dibatalkan.`)) {
    return
  }
  actionInProgress.value = reportId
  try {
    await deleteReport(reportId)
    await loadReports()
  } catch (err: any) {
    toast.error('Gagal Menghapus Laporan', err.message || 'Terjadi kesalahan.')
  } finally {
    actionInProgress.value = null
  }
}

const formatDate = (iso: string) => {
  if (!iso) return '-'
  const d = new Date(iso)
  return d.toLocaleDateString('id-ID', {
    day: 'numeric',
    month: 'short',
    year: 'numeric',
    hour: '2-digit',
    minute: '2-digit'
  })
}

onMounted(() => {
  loadReports()
})
</script>

<template>
  <div class="min-h-screen bg-canvas pb-16">
    <AdminNav />

    <div class="max-w-6xl mx-auto px-4 sm:px-6">
      <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4 mb-6">
        <div>
          <h2 class="text-xl font-bold text-ink-primary">Semua Laporan Sistem</h2>
          <p class="text-xs sm:text-sm text-ink-secondary mt-0.5">
            Daftar lengkap seluruh laporan PKL dari semua pengguna. Anda dapat membuka kunci gratis, meregenerasi, atau melihat dokumen langsung.
          </p>
        </div>

        <NuxtLink
          to="/reports/new"
          class="px-3.5 py-2 text-xs font-semibold bg-accent-600 hover:bg-accent-500 text-white rounded-lg transition-colors inline-flex items-center gap-1.5 self-start sm:self-auto shadow-subtle"
        >
          <svg class="w-3.5 h-3.5" viewBox="0 0 20 20" fill="currentColor">
            <path fill-rule="evenodd" d="M10 3a1 1 0 011 1v5h5a1 1 0 110 2h-5v5a1 1 0 11-2 0v-5H4a1 1 0 110-2h5V4a1 1 0 011-1z" clip-rule="evenodd" />
          </svg>
          <span>Buat Laporan Baru</span>
        </NuxtLink>
      </div>

      <!-- Filters & Search Bar -->
      <div class="bg-surface border border-border p-4 rounded-xl mb-6 flex flex-col md:flex-row gap-3 items-center justify-between">
        <div class="w-full md:w-96 relative">
          <input
            v-model="searchQuery"
            type="text"
            placeholder="Cari judul, email, siswa, atau instansi..."
            class="w-full pl-9 pr-4 py-2 text-xs bg-surface-elevated border border-border rounded-lg text-ink-primary placeholder:text-ink-muted focus:outline-none focus:border-accent-500"
            @keyup.enter="handleSearch"
          />
          <svg class="w-4 h-4 text-ink-muted absolute left-3 top-2.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
          </svg>
        </div>

        <div class="flex items-center gap-3 w-full md:w-auto">
          <select
            v-model="statusFilter"
            class="px-3 py-2 text-xs bg-surface-elevated border border-border rounded-lg text-ink-primary focus:outline-none focus:border-accent-500 w-full md:w-auto"
            @change="handleStatusChange"
          >
            <option v-for="st in statuses" :key="st.value" :value="st.value">
              {{ st.label }}
            </option>
          </select>

          <button
            type="button"
            class="px-3 py-2 text-xs font-medium bg-surface-elevated hover:bg-surface-hover border border-border text-ink-primary rounded-lg transition-colors whitespace-nowrap"
            @click="handleSearch"
          >
            Terapkan Filter
          </button>
        </div>
      </div>

      <!-- Reports Table -->
      <div class="bg-surface border border-border rounded-xl overflow-hidden shadow-subtle">
        <div v-if="loading" class="p-12 text-center">
          <div class="inline-block animate-spin w-8 h-8 rounded-full border-2 border-accent-500 border-t-transparent" />
          <p class="text-xs text-ink-muted mt-2 font-mono">Memuat daftar laporan...</p>
        </div>

        <div v-else-if="reports.length === 0" class="p-12 text-center text-ink-muted">
          <svg class="w-10 h-10 mx-auto opacity-40 mb-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
          </svg>
          <div class="text-sm font-medium text-ink-primary">Tidak ada laporan ditemukan</div>
          <div class="text-xs text-ink-muted mt-0.5">Coba sesuaikan kata kunci pencarian atau status filter.</div>
        </div>

        <div v-else class="overflow-x-auto">
          <table class="w-full text-left border-collapse text-xs">
            <thead>
              <tr class="border-b border-border bg-surface-elevated/80 font-mono text-[11px] text-ink-muted uppercase">
                <th class="py-3 px-4">Laporan / Proyek</th>
                <th class="py-3 px-4">Pengguna</th>
                <th class="py-3 px-4">Siswa & Instansi</th>
                <th class="py-3 px-4">Status</th>
                <th class="py-3 px-4">Dibuat</th>
                <th class="py-3 px-4 text-right">Aksi Admin</th>
              </tr>
            </thead>
            <tbody class="divide-y divide-border/60">
              <tr v-for="r in reports" :key="r.id" class="hover:bg-surface-elevated/40 transition-colors">
                <!-- Title -->
                <td class="py-3 px-4 max-w-[220px]">
                  <NuxtLink :to="`/reports/${r.id}`" class="font-medium text-ink-primary hover:text-accent-400 truncate block">
                    {{ r.title }}
                  </NuxtLink>
                  <span class="text-[10px] font-mono text-ink-muted block truncate">{{ r.id }}</span>
                </td>

                <!-- Owner Email -->
                <td class="py-3 px-4 max-w-[160px]">
                  <span class="font-mono text-ink-secondary truncate block" :title="r.user_email">
                    {{ r.user_email }}
                  </span>
                </td>

                <!-- Student & Company -->
                <td class="py-3 px-4">
                  <div class="font-medium text-ink-primary">{{ r.student_name || '-' }}</div>
                  <div class="text-[11px] text-ink-muted">{{ r.company_name || '-' }}</div>
                </td>

                <!-- Status -->
                <td class="py-3 px-4">
                  <span
                    class="text-[10px] uppercase font-mono px-2 py-0.5 rounded border inline-block"
                    :class="[
                      r.status === 'unlocked' || r.status === 'paid'
                        ? 'bg-accent-500/10 text-accent-400 border-accent-500/20 font-semibold'
                        : r.status === 'failed'
                        ? 'bg-rose-500/10 text-rose-400 border-rose-500/20'
                        : 'bg-surface-elevated text-ink-secondary border-border'
                    ]"
                  >
                    {{ r.status }}
                  </span>
                </td>

                <!-- Date -->
                <td class="py-3 px-4 text-ink-muted whitespace-nowrap">
                  {{ formatDate(r.created_at) }}
                </td>

                <!-- Actions -->
                <td class="py-3 px-4 text-right">
                  <div class="flex items-center justify-end gap-1.5">
                    <!-- Free Unlock Button -->
                    <button
                      v-if="r.status !== 'unlocked' && r.status !== 'paid'"
                      type="button"
                      :disabled="actionInProgress === r.id"
                      class="px-2.5 py-1 text-[11px] font-medium rounded bg-amber-500/15 hover:bg-amber-500/25 text-amber-300 border border-amber-500/30 transition-colors inline-flex items-center gap-1"
                      title="Buka kunci gratis tanpa bayar"
                      @click="handleUnlockFree(r.id)"
                    >
                      <svg class="w-3 h-3 text-amber-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 11V7a4 4 0 118 0m-4 8v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2z" />
                      </svg>
                      <span>Buka Gratis</span>
                    </button>

                    <!-- Regenerate Button -->
                    <button
                      type="button"
                      :disabled="actionInProgress === r.id"
                      class="p-1 text-ink-muted hover:text-ink-primary hover:bg-surface-elevated rounded transition-colors"
                      title="Regenerasi Dokumen"
                      @click="handleRegenerate(r.id)"
                    >
                      <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
                      </svg>
                    </button>

                    <!-- View Button -->
                    <NuxtLink
                      :to="`/reports/${r.id}`"
                      class="p-1 text-ink-muted hover:text-accent-400 hover:bg-surface-elevated rounded transition-colors"
                      title="Lihat Detail Laporan"
                    >
                      <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z" />
                      </svg>
                    </NuxtLink>

                    <!-- Delete Button -->
                    <button
                      type="button"
                      :disabled="actionInProgress === r.id"
                      class="p-1 text-ink-muted hover:text-rose-400 hover:bg-rose-500/10 rounded transition-colors"
                      title="Hapus Laporan"
                      @click="handleDelete(r.id, r.title)"
                    >
                      <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                      </svg>
                    </button>
                  </div>
                </td>
              </tr>
            </tbody>
          </table>
        </div>

        <!-- Pagination Bar -->
        <div class="p-4 border-t border-border flex flex-col sm:flex-row items-center justify-between gap-3 text-xs text-ink-muted">
          <div>
            Menampilkan {{ reports.length }} dari {{ total }} total laporan (Halaman {{ page }} dari {{ totalPages }})
          </div>

          <div class="flex items-center gap-1.5">
            <button
              type="button"
              :disabled="page <= 1"
              class="px-2.5 py-1 rounded bg-surface-elevated border border-border disabled:opacity-40 disabled:cursor-not-allowed hover:bg-surface-hover text-ink-primary"
              @click="handlePageChange(page - 1)"
            >
              &larr; Sebelumnya
            </button>
            <span class="px-2 font-mono">{{ page }}</span>
            <button
              type="button"
              :disabled="page >= totalPages"
              class="px-2.5 py-1 rounded bg-surface-elevated border border-border disabled:opacity-40 disabled:cursor-not-allowed hover:bg-surface-hover text-ink-primary"
              @click="handlePageChange(page + 1)"
            >
              Selanjutnya &rarr;
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { AdminJobItem } from '~/types/api'

definePageMeta({
  middleware: 'admin'
})

const { getJobs, retryJob, cancelJob } = useAdmin()
const toast = useToast()

const jobs = ref<AdminJobItem[]>([])
const page = ref(1)
const limit = ref(30)
const jobTypeFilter = ref<string>('')
const statusFilter = ref<string>('')
const loading = ref(true)
const actionInProgress = ref<string | null>(null)
let autoRefreshTimer: any = null

const loadJobs = async (silent = false) => {
  if (!silent) loading.value = true
  try {
    const data = await getJobs(
      page.value,
      limit.value,
      jobTypeFilter.value || undefined,
      statusFilter.value || undefined
    )
    jobs.value = data.jobs
  } catch (err: any) {
    if (!silent) toast.error('Gagal Memuat Antrean', err.message || 'Terjadi kesalahan.')
  } finally {
    if (!silent) loading.value = false
  }
}

const handleRetry = async (jobId: string) => {
  actionInProgress.value = jobId
  try {
    await retryJob(jobId)
    await loadJobs(true)
  } catch (err: any) {
    toast.error('Gagal Mengulang Tugas', err.message || 'Terjadi kesalahan.')
  } finally {
    actionInProgress.value = null
  }
}

const handleCancel = async (jobId: string) => {
  actionInProgress.value = jobId
  try {
    await cancelJob(jobId)
    await loadJobs(true)
  } catch (err: any) {
    toast.error('Gagal Membatalkan', err.message || 'Terjadi kesalahan.')
  } finally {
    actionInProgress.value = null
  }
}

const formatDate = (iso?: string | null) => {
  if (!iso) return '-'
  const d = new Date(iso)
  return d.toLocaleDateString('id-ID', {
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit',
    day: 'numeric',
    month: 'short'
  })
}

onMounted(() => {
  loadJobs()
  autoRefreshTimer = setInterval(() => {
    loadJobs(true)
  }, 5000)
})

onUnmounted(() => {
  if (autoRefreshTimer) clearInterval(autoRefreshTimer)
})
</script>

<template>
  <div class="min-h-screen bg-canvas pb-16">
    <AdminNav />

    <div class="max-w-6xl mx-auto px-4 sm:px-6">
      <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4 mb-6">
        <div>
          <div class="flex items-center gap-2">
            <h2 class="text-xl font-bold text-ink-primary">Monitor Antrean Tugas (Job Queue)</h2>
            <span class="w-2 h-2 rounded-full bg-accent-400 animate-pulse" title="Auto-refresh setiap 5 detik" />
          </div>
          <p class="text-xs sm:text-sm text-ink-secondary mt-0.5">
            Pantau proses latar belakang riset dan generasi dokumen. Anda dapat mengulang tugas gagal atau membatalkan tugas yang menggantung.
          </p>
        </div>

        <button
          type="button"
          class="px-3.5 py-2 text-xs font-medium bg-surface-elevated hover:bg-surface-hover border border-border text-ink-primary rounded-lg transition-colors inline-flex items-center gap-1.5 self-start sm:self-auto"
          @click="loadJobs(false)"
        >
          <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
          </svg>
          <span>Segarkan Sekarang</span>
        </button>
      </div>

      <!-- Filters Bar -->
      <div class="bg-surface border border-border p-4 rounded-xl mb-6 flex flex-wrap gap-3 items-center">
        <div>
          <label class="text-[11px] font-mono text-ink-muted block mb-1 uppercase">Tipe Tugas</label>
          <select
            v-model="jobTypeFilter"
            class="px-3 py-1.5 text-xs bg-surface-elevated border border-border rounded-lg text-ink-primary focus:outline-none focus:border-accent-500"
            @change="loadJobs(false)"
          >
            <option value="">Semua Tipe</option>
            <option value="research">Riset (Research)</option>
            <option value="generation">Generasi Dokumen (Generation)</option>
          </select>
        </div>

        <div>
          <label class="text-[11px] font-mono text-ink-muted block mb-1 uppercase">Status</label>
          <select
            v-model="statusFilter"
            class="px-3 py-1.5 text-xs bg-surface-elevated border border-border rounded-lg text-ink-primary focus:outline-none focus:border-accent-500"
            @change="loadJobs(false)"
          >
            <option value="">Semua Status</option>
            <option value="pending">Pending (Menunggu)</option>
            <option value="running">Running (Berjalan)</option>
            <option value="succeeded">Succeeded (Berhasil)</option>
            <option value="failed">Failed (Gagal)</option>
            <option value="cancelled">Cancelled (Dibatalkan)</option>
          </select>
        </div>
      </div>

      <!-- Jobs Table -->
      <div class="bg-surface border border-border rounded-xl overflow-hidden shadow-subtle">
        <div v-if="loading" class="p-12 text-center">
          <div class="inline-block animate-spin w-8 h-8 rounded-full border-2 border-accent-500 border-t-transparent" />
          <p class="text-xs text-ink-muted mt-2 font-mono">Memuat antrean tugas...</p>
        </div>

        <div v-else-if="jobs.length === 0" class="p-12 text-center text-ink-muted">
          <div class="text-sm font-medium text-ink-primary">Tidak ada antrean tugas saat ini</div>
          <div class="text-xs text-ink-muted mt-0.5">Semua proses latar belakang telah selesai dieksekusi.</div>
        </div>

        <div v-else class="overflow-x-auto">
          <table class="w-full text-left border-collapse text-xs">
            <thead>
              <tr class="border-b border-border bg-surface-elevated/80 font-mono text-[11px] text-ink-muted uppercase">
                <th class="py-3 px-4">Tugas & Laporan</th>
                <th class="py-3 px-4">Tipe</th>
                <th class="py-3 px-4">Status</th>
                <th class="py-3 px-4">Percobaan</th>
                <th class="py-3 px-4">Waktu Dibuat</th>
                <th class="py-3 px-4">Error / Keterangan</th>
                <th class="py-3 px-4 text-right">Aksi</th>
              </tr>
            </thead>
            <tbody class="divide-y divide-border/60 font-mono text-[11px]">
              <tr v-for="j in jobs" :key="j.job_id" class="hover:bg-surface-elevated/40 transition-colors">
                <!-- Job ID & Title -->
                <td class="py-3 px-4 max-w-[200px]">
                  <NuxtLink :to="`/reports/${j.report_id}`" class="font-medium text-ink-primary hover:text-accent-400 font-sans text-xs truncate block">
                    {{ j.report_title || 'Laporan PKL' }}
                  </NuxtLink>
                  <span class="text-[10px] text-ink-muted truncate block">{{ j.job_id }}</span>
                </td>

                <!-- Type -->
                <td class="py-3 px-4">
                  <span
                    class="px-2 py-0.5 rounded text-[10px] uppercase font-bold"
                    :class="j.job_type === 'research' ? 'bg-sky-500/15 text-sky-400' : 'bg-purple-500/15 text-purple-400'"
                  >
                    {{ j.job_type }}
                  </span>
                </td>

                <!-- Status -->
                <td class="py-3 px-4">
                  <span
                    class="px-2 py-0.5 rounded text-[10px] uppercase font-bold border inline-block"
                    :class="[
                      j.status === 'succeeded'
                        ? 'bg-accent-500/10 text-accent-400 border-accent-500/20'
                        : j.status === 'running'
                        ? 'bg-amber-500/15 text-amber-300 border-amber-500/30 animate-pulse'
                        : j.status === 'failed'
                        ? 'bg-rose-500/10 text-rose-400 border-rose-500/20'
                        : 'bg-surface-elevated text-ink-muted border-border'
                    ]"
                  >
                    {{ j.status }}
                  </span>
                </td>

                <!-- Attempts -->
                <td class="py-3 px-4 text-ink-secondary">
                  {{ j.attempts }} / {{ j.max_attempts }}
                </td>

                <!-- Created At -->
                <td class="py-3 px-4 text-ink-muted whitespace-nowrap">
                  {{ formatDate(j.created_at) }}
                </td>

                <!-- Error message if any -->
                <td class="py-3 px-4 max-w-[220px]">
                  <span v-if="j.error_message" class="text-rose-400 truncate block text-[10px]" :title="j.error_message">
                    [{{ j.error_code }}] {{ j.error_message }}
                  </span>
                  <span v-else class="text-ink-muted text-[10px]">-</span>
                </td>

                <!-- Actions -->
                <td class="py-3 px-4 text-right">
                  <div class="flex items-center justify-end gap-1.5 font-sans">
                    <!-- Retry Button -->
                    <button
                      type="button"
                      :disabled="actionInProgress === j.job_id"
                      class="px-2 py-1 text-[10px] font-medium rounded bg-surface-elevated hover:bg-surface-hover text-ink-primary border border-border transition-colors inline-flex items-center gap-1"
                      title="Ulangi Pekerjaan"
                      @click="handleRetry(j.job_id)"
                    >
                      <svg class="w-3 h-3 text-accent-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
                      </svg>
                      <span>Retry</span>
                    </button>

                    <!-- Cancel Button -->
                    <button
                      v-if="j.status === 'pending' || j.status === 'running'"
                      type="button"
                      :disabled="actionInProgress === j.job_id"
                      class="px-2 py-1 text-[10px] font-medium rounded bg-rose-500/10 hover:bg-rose-500/20 text-rose-400 border border-rose-500/30 transition-colors inline-flex items-center gap-1"
                      title="Batalkan Tugas"
                      @click="handleCancel(j.job_id)"
                    >
                      <span>Batal</span>
                    </button>
                  </div>
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
    </div>
  </div>
</template>

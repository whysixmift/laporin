<script setup lang="ts">
import type { JobInfo, Payment, Report } from '~/types/api'

const route = useRoute()
const router = useRouter()
const reportId = route.params.id as string

const { getReport, downloadDocx, updateReport } = useReports()
const { startResearch, getResearchStatus, startGeneration, getGenerationStatus, pollJob } = useReportJob()
const { createPayment, getPayment, pollPaymentStatus } = usePayment()
const toast = useToast()

const report = ref<Report | null>(null)
const loading = ref(true)
const error = ref('')
const activeJob = ref<JobInfo | null>(null)
const isJobProcessing = ref(false)
const isPaying = ref(false)
const currentPayment = ref<Payment | null>(null)
const isDownloading = ref(false)

// Edit mode state for draft
const isEditModalOpen = ref(false)
const isSavingEdit = ref(false)
const editForm = ref({
  student: {
    full_name: '',
    student_id: '',
    school: '',
    major: '',
    semester: ''
  },
  internship: {
    company_name: '',
    company_address: '',
    department: '',
    role: '',
    start_date: '',
    end_date: '',
    company_supervisor: '',
    school_supervisor: '',
    description: ''
  }
})

const handleOpenEditModal = () => {
  if (!report.value) return
  editForm.value = {
    student: {
      full_name: report.value.student?.full_name || '',
      student_id: report.value.student?.student_id || '',
      school: report.value.student?.school || '',
      major: report.value.student?.major || '',
      semester: report.value.student?.semester || ''
    },
    internship: {
      company_name: report.value.internship?.company_name || '',
      company_address: report.value.internship?.company_address || '',
      department: report.value.internship?.department || '',
      role: report.value.internship?.role || '',
      start_date: report.value.internship?.start_date || '',
      end_date: report.value.internship?.end_date || '',
      company_supervisor: report.value.internship?.company_supervisor || '',
      school_supervisor: report.value.internship?.school_supervisor || '',
      description: report.value.internship?.description || ''
    }
  }
  isEditModalOpen.value = true
}

const handleSaveEdit = async () => {
  isSavingEdit.value = true
  try {
    await updateReport(reportId, {
      student: {
        full_name: editForm.value.student.full_name.trim(),
        student_id: editForm.value.student.student_id.trim(),
        school: editForm.value.student.school.trim(),
        major: editForm.value.student.major.trim() || null,
        semester: editForm.value.student.semester.trim() || null
      },
      internship: {
        company_name: editForm.value.internship.company_name.trim(),
        company_address: editForm.value.internship.company_address.trim() || null,
        department: editForm.value.internship.department.trim() || null,
        role: editForm.value.internship.role.trim() || null,
        start_date: editForm.value.internship.start_date || null,
        end_date: editForm.value.internship.end_date || null,
        company_supervisor: editForm.value.internship.company_supervisor.trim() || null,
        school_supervisor: editForm.value.internship.school_supervisor.trim() || null,
        description: editForm.value.internship.description.trim() || null
      }
    })
    toast.success('Perubahan Disimpan', 'Data laporan berhasil diperbarui.')
    isEditModalOpen.value = false
    await fetchReportData()
  } catch (err: unknown) {
    const apiErr = err as { message?: string }
    toast.error('Gagal Menyimpan', apiErr.message || 'Tidak dapat memperbarui data laporan.')
  } finally {
    isSavingEdit.value = false
  }
}

const fetchReportData = async () => {
  loading.value = true
  error.value = ''
  try {
    const data = await getReport(reportId)
    report.value = data

    // Resume polling if active job states
    if (data.status === 'researching') {
      resumeResearchPolling()
    } else if (data.status === 'generating') {
      resumeGenerationPolling()
    } else if (data.status === 'payment_pending') {
      // If returning from payment gateway
      if (route.query.payment_return) {
        toast.info('Verifikasi Pembayaran', 'Memeriksa status pembayaran dari sistem Mayar...')
      }
    }
  } catch (err: unknown) {
    const apiErr = err as { message?: string; status?: number }
    if (apiErr.status === 401) {
      router.replace(`/login?redirect=/reports/${reportId}`)
      return
    }
    error.value = apiErr.message || 'Gagal memuat detail laporan.'
  } finally {
    loading.value = false
  }
}

onMounted(() => {
  fetchReportData()
})

// === RESEARCH FLOW ===
const handleStartResearch = async () => {
  if (!report.value) return
  isJobProcessing.value = true
  error.value = ''

  try {
    const job = await startResearch(reportId)
    activeJob.value = job
    report.value.status = 'researching'
    toast.info('Riset Dimulai', 'Sistem sedang menelusuri data profil perusahaan...')

    await pollJob(
      () => getResearchStatus(reportId),
      (updatedJob) => {
        activeJob.value = updatedJob
      }
    )

    // On success
    toast.success('Riset Selesai', 'Informasi perusahaan berhasil dikumpulkan dan diverifikasi.')
    await fetchReportData()
  } catch (err: unknown) {
    const apiErr = err as { message?: string }
    error.value = apiErr.message || 'Proses riset gagal dijalankan.'
    if (report.value) report.value.status = 'failed'
  } finally {
    isJobProcessing.value = false
  }
}

const resumeResearchPolling = async () => {
  isJobProcessing.value = true
  try {
    await pollJob(
      () => getResearchStatus(reportId),
      (updatedJob) => {
        activeJob.value = updatedJob
      }
    )
    await fetchReportData()
  } catch (err: unknown) {
    const apiErr = err as { message?: string }
    error.value = apiErr.message || 'Riset gagal diselesaikan.'
  } finally {
    isJobProcessing.value = false
  }
}

// === GENERATION FLOW ===
const handleStartGeneration = async () => {
  if (!report.value) return
  isJobProcessing.value = true
  error.value = ''

  try {
    const job = await startGeneration(reportId)
    activeJob.value = job
    report.value.status = 'generating'
    toast.info('Penyusunan Dimulai', 'Dokumen laporan sedang dirangkai menjadi bab akademik...')

    await pollJob(
      () => getGenerationStatus(reportId),
      (updatedJob) => {
        activeJob.value = updatedJob
      }
    )

    toast.success('Laporan Siap', 'Dokumen laporan dan pratinjau berhasil disusun.')
    await fetchReportData()
  } catch (err: unknown) {
    const apiErr = err as { message?: string }
    error.value = apiErr.message || 'Penyusunan laporan gagal.'
    if (report.value) report.value.status = 'failed'
  } finally {
    isJobProcessing.value = false
  }
}

const resumeGenerationPolling = async () => {
  isJobProcessing.value = true
  try {
    await pollJob(
      () => getGenerationStatus(reportId),
      (updatedJob) => {
        activeJob.value = updatedJob
      }
    )
    await fetchReportData()
  } catch (err: unknown) {
    const apiErr = err as { message?: string }
    error.value = apiErr.message || 'Penyusunan laporan gagal diselesaikan.'
  } finally {
    isJobProcessing.value = false
  }
}

// === PAYMENT FLOW ===
const handleInitiatePayment = async () => {
  if (!report.value) return
  isPaying.value = true
  error.value = ''

  try {
    const paymentRes = await createPayment(reportId)
    if (paymentRes.payment_url) {
      // Update report state to payment_pending
      report.value.status = 'payment_pending'

      // Open Mayar payment portal
      window.open(paymentRes.payment_url, '_blank')
      toast.info('Halaman Pembayaran Dibuka', 'Selesaikan pembayaran di tab Mayar yang terbuka.')

      // Start background polling for webhook unlock
      const isUnlocked = await pollPaymentStatus(
        paymentRes.payment_id,
        reportId,
        (status) => {
          if (status === 'unlocked' && report.value) {
            report.value.status = 'unlocked'
          }
        }
      )

      if (isUnlocked) {
        toast.success('Pembayaran Dikonfirmasi', 'Laporan telah terbuka! Anda dapat langsung mengunduh file DOCX.')
        await fetchReportData()
      }
    }
  } catch (err: unknown) {
    const apiErr = err as { message?: string }
    error.value = apiErr.message || 'Gagal memulai transaksi pembayaran.'
  } finally {
    isPaying.value = false
  }
}

// === DOWNLOAD FLOW ===
const handleDownloadDocx = async () => {
  if (!report.value) return
  isDownloading.value = true
  try {
    const filename = `laporan-pkl-${report.value.student?.full_name?.toLowerCase().replace(/\s+/g, '-') || 'lengkap'}.docx`
    await downloadDocx(reportId, filename)
    toast.success('Unduhan Dimulai', 'File DOCX laporan PKL Anda berhasil diunduh.')
  } catch (err: unknown) {
    const apiErr = err as { message?: string }
    toast.error('Unduhan Gagal', apiErr.message || 'Gagal mengunduh dokumen. Pastikan laporan telah terbuka.')
  } finally {
    isDownloading.value = false
  }
}
</script>

<template>
  <div class="max-w-5xl mx-auto px-4 sm:px-6 py-8 sm:py-12 space-y-8">
    <!-- Breadcrumb & Status Topbar -->
    <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4 pb-4 border-b border-border">
      <div class="space-y-1">
        <div class="flex items-center gap-2 text-xs text-ink-muted">
          <NuxtLink to="/dashboard" class="hover:text-ink-secondary">Laporan Saya</NuxtLink>
          <span>/</span>
          <span class="font-mono text-[11px] text-ink-muted truncate max-w-[200px]">ID: {{ reportId.slice(0, 8) }}</span>
        </div>

        <h1 class="text-xl sm:text-2xl font-bold text-ink-primary line-clamp-1">
          {{ report?.title || 'Laporan PKL' }}
        </h1>
      </div>

      <div class="flex items-center gap-3 shrink-0">
        <BaseBadge v-if="report" :status="report.status" size="md" />

        <BaseButton
          v-if="report && (report.status === 'unlocked' || report.status === 'paid')"
          variant="primary"
          size="sm"
          :loading="isDownloading"
          @click="handleDownloadDocx"
        >
          <template #leading>
            <svg class="w-4 h-4" viewBox="0 0 20 20" fill="currentColor">
              <path fill-rule="evenodd" d="M3 17a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm3.293-7.707a1 1 0 011.414 0L9 10.586V3a1 1 0 112 0v7.586l1.293-1.293a1 1 0 111.414 1.414l-3 3a1 1 0 01-1.414 0l-3-3a1 1 0 010-1.414z" clip-rule="evenodd" />
            </svg>
          </template>
          Unduh DOCX
        </BaseButton>
      </div>
    </div>

    <!-- Stepper indicator -->
    <ReportStatusIndicator v-if="report" :status="report.status" />

    <!-- Error Alert -->
    <BaseAlert v-if="error" type="error" dismissible @dismiss="error = ''">
      {{ error }}
    </BaseAlert>

    <!-- Loading Skeleton -->
    <div v-if="loading" class="p-12 text-center space-y-4">
      <div class="inline-block animate-spin w-8 h-8 rounded-full border-2 border-accent-500 border-t-transparent" />
      <p class="text-xs text-ink-muted">Memuat data laporan...</p>
    </div>

    <template v-else-if="report">
      <!-- 1. DRAFT STATE -->
      <div v-if="report.status === 'draft'" class="space-y-6">
        <div class="p-6 bg-surface rounded-xl border border-border space-y-6">
          <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4 pb-4 border-b border-border">
            <div>
              <h2 class="text-lg font-bold text-ink-primary">Draf Laporan Disimpan</h2>
              <p class="text-xs text-ink-secondary">Data siswa dan kegiatan PKL telah tersimpan. Mulai riset untuk mengumpulkan profil resmi perusahaan.</p>
            </div>

            <div class="flex items-center gap-3">
              <BaseButton
                variant="secondary"
                size="md"
                @click="handleOpenEditModal"
              >
                <template #leading>
                  <svg class="w-4 h-4" viewBox="0 0 20 20" fill="currentColor">
                    <path d="M13.586 3.586a2 2 0 112.828 2.828l-.793.793-2.828-2.828.793-.793zM11.379 5.793L3 14.172V17h2.828l8.38-8.379-2.83-2.828z" />
                  </svg>
                </template>
                Edit Data Draf
              </BaseButton>

              <BaseButton
                variant="primary"
                size="lg"
                :loading="isJobProcessing"
                @click="handleStartResearch"
              >
                <template #leading>
                  <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <circle cx="11" cy="11" r="8" />
                    <line x1="21" y1="21" x2="16.65" y2="16.65" />
                  </svg>
                </template>
                Mulai Riset Profil Perusahaan
              </BaseButton>
            </div>
          </div>

          <!-- Data Summary -->
          <div class="grid grid-cols-1 md:grid-cols-2 gap-6 text-xs">
            <div class="p-4 bg-surface-subtle rounded-lg border border-border-subtle space-y-3">
              <span class="font-mono text-[10px] uppercase tracking-wider text-accent-400 font-bold">Identitas Siswa</span>
              <div class="space-y-1.5">
                <div class="text-ink-primary font-semibold text-sm">{{ report.student.full_name }}</div>
                <div class="text-ink-muted">NIS / NIM: <span class="text-ink-secondary">{{ report.student.student_id }}</span></div>
                <div class="text-ink-muted">Institusi: <span class="text-ink-secondary">{{ report.student.school }}</span></div>
                <div v-if="report.student.major" class="text-ink-muted">Jurusan: <span class="text-ink-secondary">{{ report.student.major }}</span></div>
              </div>
            </div>

            <div class="p-4 bg-surface-subtle rounded-lg border border-border-subtle space-y-3">
              <span class="font-mono text-[10px] uppercase tracking-wider text-accent-400 font-bold">Tempat PKL</span>
              <div class="space-y-1.5">
                <div class="text-ink-primary font-semibold text-sm">{{ report.internship.company_name }}</div>
                <div v-if="report.internship.company_address" class="text-ink-muted">Alamat: <span class="text-ink-secondary">{{ report.internship.company_address }}</span></div>
                <div v-if="report.internship.department" class="text-ink-muted">Divisi: <span class="text-ink-secondary">{{ report.internship.department }}</span></div>
              </div>
            </div>

            <div class="md:col-span-2 p-4 bg-surface-subtle rounded-lg border border-border-subtle space-y-2">
              <span class="font-mono text-[10px] uppercase tracking-wider text-accent-400 font-bold">Catatan Jurnal Kegiatan</span>
              <p class="text-ink-secondary whitespace-pre-line leading-relaxed">
                {{ report.internship.description || 'Tidak ada deskripsi kegiatan.' }}
              </p>
            </div>
          </div>
        </div>
      </div>

      <!-- 2. RESEARCHING STATE -->
      <div v-else-if="report.status === 'researching'" class="p-8 bg-surface rounded-xl border border-ochre-500/30 text-center space-y-4">
        <div class="w-12 h-12 rounded-full bg-ochre-500/10 border border-ochre-500/30 mx-auto flex items-center justify-center text-ochre-400">
          <svg class="w-6 h-6 animate-spin" fill="none" viewBox="0 0 24 24">
            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" />
            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z" />
          </svg>
        </div>

        <div class="space-y-1">
          <h3 class="text-lg font-bold text-ink-primary">
            Menelusuri Informasi Resmi Perusahaan
          </h3>
          <p class="text-xs text-ink-secondary max-w-md mx-auto leading-relaxed">
            Sistem sedang mencari website resmi, profil bisnis, visi, misi, dan struktur operasional <strong class="text-ink-primary">{{ report.internship.company_name }}</strong>.
          </p>
        </div>

        <div class="pt-2">
          <span class="text-[11px] font-mono text-ink-muted bg-surface-elevated px-3 py-1 rounded border border-border">
            Status Pekerjaan: {{ activeJob?.status || 'Sedang berjalan' }}
          </span>
        </div>
      </div>

      <!-- 3. RESEARCH COMPLETED STATE -->
      <div v-else-if="report.status === 'research_completed'" class="space-y-6">
        <div class="p-6 bg-surface rounded-xl border border-border space-y-6">
          <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4 pb-4 border-b border-border">
            <div class="space-y-1">
              <div class="flex items-center gap-2">
                <span class="w-2 h-2 rounded-full bg-emerald-400" />
                <h2 class="text-lg font-bold text-ink-primary">Riset Perusahaan Berhasil</h2>
              </div>
              <p class="text-xs text-ink-secondary">
                Fakta terverifikasi telah siap. Lanjutkan untuk menyusun seluruh bab laporan akademik secara otomatis.
              </p>
            </div>

            <BaseButton
              variant="primary"
              size="lg"
              :loading="isJobProcessing"
              @click="handleStartGeneration"
            >
              <template #leading>
                <svg class="w-4 h-4" viewBox="0 0 20 20" fill="currentColor">
                  <path d="M17.414 2.586a2 2 0 00-2.828 0L7 10.172V13h2.828l7.586-7.586a2 2 0 000-2.828z" />
                  <path fill-rule="evenodd" d="M2 6a2 2 0 012-2h4a1 1 0 010 2H4v10h10v-4a1 1 0 112 0v4a2 2 0 01-2 2H4a2 2 0 01-2-2V6z" clip-rule="evenodd" />
                </svg>
              </template>
              Susun Laporan PKL Sekarang
            </BaseButton>
          </div>

          <!-- Fact items -->
          <div v-if="report.research_facts?.length" class="space-y-3">
            <div class="text-xs font-semibold text-ink-primary">
              Fakta Profil yang Ditemukan ({{ report.research_facts.length }})
            </div>
            <FactItem
              v-for="(fact, index) in report.research_facts"
              :key="index"
              :fact="fact"
              :index="index"
            />
          </div>
          <div v-else class="text-xs text-ink-muted italic p-4 bg-surface-subtle rounded border border-border-subtle">
            Data profil dasar perusahaan telah dicatat dan siap disusun ke dalam Bab II.
          </div>
        </div>
      </div>

      <!-- 4. GENERATING STATE -->
      <div v-else-if="report.status === 'generating'" class="p-8 bg-surface rounded-xl border border-sky-500/30 text-center space-y-4">
        <div class="w-12 h-12 rounded-full bg-sky-500/10 border border-sky-500/30 mx-auto flex items-center justify-center text-sky-400">
          <svg class="w-6 h-6 animate-spin" fill="none" viewBox="0 0 24 24">
            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" />
            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z" />
          </svg>
        </div>

        <div class="space-y-1">
          <h3 class="text-lg font-bold text-ink-primary">
            Menyusun Bab & Format Dokumen
          </h3>
          <p class="text-xs text-ink-secondary max-w-md mx-auto leading-relaxed">
            Menghubungkan data aktivitas magang kamu dengan profil perusahaan, menyusun Bab I hingga Bab IV, dan menyiapkan pratinjau dokumen.
          </p>
        </div>

        <div class="pt-2">
          <span class="text-[11px] font-mono text-ink-muted bg-surface-elevated px-3 py-1 rounded border border-border">
            Proses Penyusunan: {{ activeJob?.status || 'Sedang berlangsung' }}
          </span>
        </div>
      </div>

      <!-- 5. PREVIEW READY & PAYMENT PENDING STATES (PAYWALL VIEW) -->
      <div v-else-if="['preview_ready', 'payment_pending'].includes(report.status)" class="space-y-8">
        <!-- Paywall Callout -->
        <PaywallOverlay
          :report-id="report.id"
          :loading="isPaying"
          @pay="handleInitiatePayment"
        />

        <!-- Payment pending refresh status -->
        <div v-if="report.status === 'payment_pending'" class="p-4 bg-ochre-500/10 border border-ochre-500/30 rounded-lg flex items-center justify-between text-xs text-ochre-300">
          <div class="flex items-center gap-2">
            <span class="w-2 h-2 rounded-full bg-ochre-400 animate-pulse" />
            <span>Menunggu konfirmasi pembayaran otomatis via webhook Mayar...</span>
          </div>
          <button
            type="button"
            class="px-3 py-1 rounded bg-ochre-500/20 hover:bg-ochre-500/30 text-ochre-200 font-medium transition-colors"
            @click="fetchReportData"
          >
            Periksa Status Sekarang
          </button>
        </div>

        <!-- Academic Document Viewer with Watermark -->
        <DocumentViewer
          :report="report"
          :is-watermarked="true"
        />
      </div>

      <!-- 6. UNLOCKED / PAID STATE -->
      <div v-else-if="report.status === 'unlocked' || report.status === 'paid'" class="space-y-8">
        <!-- Unlocked Celebration Banner -->
        <div class="p-6 bg-accent-500/10 border border-accent-500/40 rounded-xl flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4">
          <div class="space-y-1">
            <div class="flex items-center gap-2 text-accent-400 font-bold text-sm">
              <svg class="w-5 h-5 shrink-0" viewBox="0 0 20 20" fill="currentColor">
                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd" />
              </svg>
              <span>Dokumen Laporan Telah Terbuka Sepenuhnya</span>
            </div>
            <p class="text-xs text-ink-secondary">
              File Microsoft Word (.docx) Anda siap diunduh dan digunakan tanpa tanda air.
            </p>
          </div>

          <BaseButton
            variant="primary"
            size="lg"
            :loading="isDownloading"
            @click="handleDownloadDocx"
          >
            <template #leading>
              <svg class="w-4 h-4" viewBox="0 0 20 20" fill="currentColor">
                <path fill-rule="evenodd" d="M3 17a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm3.293-7.707a1 1 0 011.414 0L9 10.586V3a1 1 0 112 0v7.586l1.293-1.293a1 1 0 111.414 1.414l-3 3a1 1 0 01-1.414 0l-3-3a1 1 0 010-1.414z" clip-rule="evenodd" />
              </svg>
            </template>
            Unduh File DOCX
          </BaseButton>
        </div>

        <!-- Academic Document Viewer without Watermark -->
        <DocumentViewer
          :report="report"
          :is-watermarked="false"
        />
      </div>

      <!-- 7. FAILED STATE -->
      <div v-else-if="report.status === 'failed'" class="p-8 bg-surface rounded-xl border border-danger-500/30 text-center space-y-4">
        <div class="w-12 h-12 rounded-full bg-danger-500/10 border border-danger-500/30 mx-auto flex items-center justify-center text-danger-400">
          <svg class="w-6 h-6" viewBox="0 0 20 20" fill="currentColor">
            <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7 4a1 1 0 11-2 0 1 1 0 012 0zm-1-9a1 1 0 00-1 1v4a1 1 0 102 0V6a1 1 0 00-1-1z" clip-rule="evenodd" />
          </svg>
        </div>

        <div class="space-y-1">
          <h3 class="text-lg font-bold text-ink-primary">
            Terjadi Kendala pada Proses
          </h3>
          <p class="text-xs text-ink-secondary max-w-md mx-auto leading-relaxed">
            Pekerjaan riset atau penyusunan tidak dapat diselesaikan. Anda dapat mencoba kembali langkah sebelumnya.
          </p>
        </div>

        <div class="flex items-center justify-center gap-3 pt-2">
          <BaseButton variant="subtle" @click="fetchReportData">
            Muat Ulang
          </BaseButton>
          <BaseButton variant="primary" @click="handleStartResearch">
            Coba Riset Ulang
          </BaseButton>
        </div>
      </div>
    </template>

    <!-- Edit Draft Modal -->
    <BaseModal
      v-model="isEditModalOpen"
      title="Edit Data Laporan PKL"
      max-width="xl"
    >
      <form class="space-y-6" @submit.prevent="handleSaveEdit">
        <!-- Student Information -->
        <div class="space-y-4">
          <div class="flex items-center gap-2 pb-2 border-b border-border">
            <span class="font-mono text-[10px] uppercase tracking-wider text-accent-400 font-bold">1. Identitas Siswa / Mahasiswa</span>
          </div>

          <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
            <BaseInput
              v-model="editForm.student.full_name"
              label="Nama Lengkap Siswa *"
              placeholder="Contoh: Budi Pratama"
              required
            />
            <BaseInput
              v-model="editForm.student.student_id"
              label="NIS / NIM *"
              placeholder="Contoh: 210401050"
              required
            />
          </div>

          <BaseInput
            v-model="editForm.student.school"
            label="Sekolah / Universitas *"
            placeholder="Contoh: SMK Negeri 1 Jakarta"
            required
          />

          <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
            <BaseInput
              v-model="editForm.student.major"
              label="Jurusan / Program Studi"
              placeholder="Contoh: Teknik Komputer & Jaringan"
            />
            <BaseInput
              v-model="editForm.student.semester"
              label="Kelas / Semester"
              placeholder="Contoh: Kelas XII / Semester 5"
            />
          </div>
        </div>

        <!-- Internship Information -->
        <div class="space-y-4">
          <div class="flex items-center gap-2 pb-2 border-b border-border">
            <span class="font-mono text-[10px] uppercase tracking-wider text-accent-400 font-bold">2. Tempat & Waktu PKL</span>
          </div>

          <BaseInput
            v-model="editForm.internship.company_name"
            label="Nama Perusahaan / Instansi *"
            placeholder="Contoh: PT Telkom Indonesia"
            required
          />

          <BaseInput
            v-model="editForm.internship.company_address"
            label="Alamat Perusahaan"
            placeholder="Contoh: Jl. Jend. Sudirman Kav. 52-53, Jakarta Selatan"
          />

          <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
            <BaseInput
              v-model="editForm.internship.department"
              label="Divisi / Departemen"
              placeholder="Contoh: IT Network Infrastructure"
            />
            <BaseInput
              v-model="editForm.internship.role"
              label="Peran / Posisi"
              placeholder="Contoh: Junior Network Engineer Intern"
            />
          </div>

          <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
            <BaseInput
              v-model="editForm.internship.start_date"
              type="date"
              label="Tanggal Mulai"
            />
            <BaseInput
              v-model="editForm.internship.end_date"
              type="date"
              label="Tanggal Selesai"
            />
          </div>

          <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
            <BaseInput
              v-model="editForm.internship.company_supervisor"
              label="Pembimbing Lapangan"
              placeholder="Contoh: Hendra Wijaya, S.T."
            />
            <BaseInput
              v-model="editForm.internship.school_supervisor"
              label="Guru / Dosen Pembimbing"
              placeholder="Contoh: Dra. Sri Wahyuni, M.Pd."
            />
          </div>
        </div>

        <!-- Journal & Activities -->
        <div class="space-y-4">
          <div class="flex items-center gap-2 pb-2 border-b border-border">
            <span class="font-mono text-[10px] uppercase tracking-wider text-accent-400 font-bold">3. Jurnal & Catatan Kegiatan</span>
          </div>

          <BaseTextarea
            v-model="editForm.internship.description"
            label="Ringkasan Aktivitas dan Tugas PKL"
            placeholder="Tuliskan aktivitas utama yang dikerjakan selama PKL..."
            :rows="5"
          />
        </div>
      </form>

      <template #footer>
        <BaseButton
          variant="subtle"
          :disabled="isSavingEdit"
          @click="isEditModalOpen = false"
        >
          Batal
        </BaseButton>
        <BaseButton
          variant="primary"
          :loading="isSavingEdit"
          @click="handleSaveEdit"
        >
          Simpan Perubahan
        </BaseButton>
      </template>
    </BaseModal>
  </div>
</template>

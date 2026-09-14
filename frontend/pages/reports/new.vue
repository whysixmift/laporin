<script setup lang="ts">
import type { ReportCreate } from '~/types/api'

const { createReport } = useReports()
const router = useRouter()
const toast = useToast()

const step = ref(1)
const loading = ref(false)
const errorMessage = ref('')

// Form State
const title = ref('')
const student = ref({
  full_name: '',
  student_id: '',
  school: '',
  major: '',
  semester: ''
})

const internship = ref({
  company_name: '',
  company_address: '',
  department: '',
  role: '',
  start_date: '',
  end_date: '',
  company_supervisor: '',
  school_supervisor: '',
  description: ''
})

// Validation for each step
const validateStep1 = () => {
  errorMessage.value = ''
  if (!student.value.full_name.trim()) {
    errorMessage.value = 'Nama lengkap siswa/mahasiswa wajib diisi.'
    return false
  }
  if (!student.value.student_id.trim()) {
    errorMessage.value = 'NIS / NIM wajib diisi.'
    return false
  }
  if (!student.value.school.trim()) {
    errorMessage.value = 'Nama sekolah / universitas wajib diisi.'
    return false
  }
  return true
}

const validateStep2 = () => {
  errorMessage.value = ''
  if (!internship.value.company_name.trim()) {
    errorMessage.value = 'Nama instansi / perusahaan tempat PKL wajib diisi.'
    return false
  }
  return true
}

const validateStep3 = () => {
  errorMessage.value = ''
  if (!internship.value.description.trim()) {
    errorMessage.value = 'Deskripsi kegiatan nyata selama PKL wajib diisi.'
    return false
  }
  return true
}

const handleNext = () => {
  if (step.value === 1 && validateStep1()) {
    step.value = 2
  } else if (step.value === 2 && validateStep2()) {
    step.value = 3
  } else if (step.value === 3 && validateStep3()) {
    // Auto-generate title if empty
    if (!title.value.trim()) {
      title.value = `Laporan PKL di ${internship.value.company_name.trim()}`
    }
    step.value = 4
  }
}

const handlePrev = () => {
  errorMessage.value = ''
  if (step.value > 1) {
    step.value -= 1
  }
}

const handleSubmit = async () => {
  errorMessage.value = ''
  loading.value = true

  const payload: ReportCreate = {
    title: title.value.trim() || `Laporan PKL - ${student.value.full_name}`,
    student: {
      full_name: student.value.full_name.trim(),
      student_id: student.value.student_id.trim(),
      school: student.value.school.trim(),
      major: student.value.major.trim() || null,
      semester: student.value.semester.trim() || null
    },
    internship: {
      company_name: internship.value.company_name.trim(),
      company_address: internship.value.company_address.trim() || null,
      department: internship.value.department.trim() || null,
      role: internship.value.role.trim() || null,
      start_date: internship.value.start_date || null,
      end_date: internship.value.end_date || null,
      company_supervisor: internship.value.company_supervisor.trim() || null,
      school_supervisor: internship.value.school_supervisor.trim() || null,
      description: internship.value.description.trim() || null
    }
  }

  try {
    const report = await createReport(payload)
    toast.success('Draf Dibuat', 'Draf laporan PKL berhasil dibuat. Lanjutkan ke tahap riset.')
    router.push(`/reports/${report.id}`)
  } catch (err: unknown) {
    const apiErr = err as { message?: string }
    errorMessage.value = apiErr.message || 'Gagal menyimpan laporan. Silakan periksa kembali data Anda.'
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <div class="max-w-3xl mx-auto px-4 sm:px-6 py-8 sm:py-12 space-y-8">
    <!-- Header -->
    <div class="space-y-1">
      <div class="flex items-center gap-2 text-xs text-ink-muted">
        <NuxtLink to="/dashboard" class="hover:text-ink-secondary">Laporan Saya</NuxtLink>
        <span>/</span>
        <span class="text-ink-primary">Buat Draf Baru</span>
      </div>
      <h1 class="text-2xl font-bold text-ink-primary">
        Formulir Laporan PKL
      </h1>
      <p class="text-xs sm:text-sm text-ink-secondary">
        Lengkapi data berikut untuk menyusun laporan akademik yang terstruktur.
      </p>
    </div>

    <!-- Stepper Navigation -->
    <div class="grid grid-cols-2 sm:grid-cols-4 gap-2 text-xs">
      <div
        v-for="s in [
          { num: 1, label: 'Data Siswa' },
          { num: 2, label: 'Tempat PKL' },
          { num: 3, label: 'Jurnal Kegiatan' },
          { num: 4, label: 'Konfirmasi' }
        ]"
        :key="s.num"
        :class="[
          'p-2.5 sm:p-3 rounded-lg border flex flex-col gap-0.5 sm:gap-1 transition-colors',
          step === s.num
            ? 'bg-surface-elevated border-accent-500/50 text-ink-primary'
            : step > s.num
              ? 'bg-surface border-border text-ink-secondary'
              : 'bg-surface-subtle border-border-subtle text-ink-muted'
        ]"
      >
        <span class="font-mono text-[9px] sm:text-[10px] uppercase font-bold text-accent-400">Langkah 0{{ s.num }}</span>
        <span class="font-medium text-xs truncate">{{ s.label }}</span>
      </div>
    </div>

    <!-- Error Alert -->
    <BaseAlert v-if="errorMessage" type="error" dismissible @dismiss="errorMessage = ''">
      {{ errorMessage }}
    </BaseAlert>

    <!-- STEP 1: Student Information -->
    <div v-if="step === 1" class="p-5 sm:p-8 rounded-xl bg-surface border border-border space-y-6">
      <div class="space-y-1 pb-4 border-b border-border">
        <h2 class="text-base font-semibold text-ink-primary">1. Data Siswa / Mahasiswa</h2>
        <p class="text-xs text-ink-secondary">Informasi ini dicantumkan pada lembar sampul dan pengesahan laporan.</p>
      </div>

      <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
        <div class="sm:col-span-2">
          <BaseInput
            v-model="student.full_name"
            label="Nama Lengkap Siswa / Mahasiswa"
            placeholder="Contoh: Muhammad Farhan"
            required
          />
        </div>

        <BaseInput
          v-model="student.student_id"
          label="Nomor Induk Siswa / NIM"
          placeholder="Contoh: 202100412"
          required
        />

        <BaseInput
          v-model="student.school"
          label="Nama Sekolah / Kampus"
          placeholder="Contoh: SMK Negeri 1 Jakarta"
          required
        />

        <BaseInput
          v-model="student.major"
          label="Jurusan / Program Keahlian"
          placeholder="Contoh: Rekayasa Perangkat Lunak"
          hint="Opsional"
        />

        <BaseInput
          v-model="student.semester"
          label="Kelas / Semester"
          placeholder="Contoh: XII RPL 1"
          hint="Opsional"
        />
      </div>

      <div class="pt-4 flex flex-col sm:flex-row justify-end">
        <BaseButton variant="primary" class="w-full sm:w-auto justify-center" @click="handleNext">
          Lanjut ke Data Tempat PKL
        </BaseButton>
      </div>
    </div>

    <!-- STEP 2: Company & Supervisors -->
    <div v-if="step === 2" class="p-5 sm:p-8 rounded-xl bg-surface border border-border space-y-6">
      <div class="space-y-1 pb-4 border-b border-border">
        <h2 class="text-base font-semibold text-ink-primary">2. Tempat PKL & Pembimbing</h2>
        <p class="text-xs text-ink-secondary">Nama instansi/perusahaan akan diteliti secara otomatis untuk profil perusahaan.</p>
      </div>

      <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
        <div class="sm:col-span-2">
          <BaseInput
            v-model="internship.company_name"
            label="Nama Perusahaan / Instansi"
            placeholder="Contoh: PT Telkom Indonesia"
            required
          />
        </div>

        <div class="sm:col-span-2">
          <BaseInput
            v-model="internship.company_address"
            label="Alamat Perusahaan"
            placeholder="Contoh: Jl. Japati No. 1, Bandung"
            hint="Opsional"
          />
        </div>

        <BaseInput
          v-model="internship.department"
          label="Divisi / Departemen"
          placeholder="Contoh: Divisi IT Infrastructure"
          hint="Opsional"
        />

        <BaseInput
          v-model="internship.role"
          label="Posisi / Peran Magang"
          placeholder="Contoh: Network Operations Intern"
          hint="Opsional"
        />

        <BaseInput
          v-model="internship.start_date"
          label="Tanggal Mulai PKL"
          type="date"
          hint="Opsional"
        />

        <BaseInput
          v-model="internship.end_date"
          label="Tanggal Selesai PKL"
          type="date"
          hint="Opsional"
        />

        <BaseInput
          v-model="internship.company_supervisor"
          label="Nama Pembimbing Lapangan (DU/DI)"
          placeholder="Contoh: Ir. Budi Santoso"
          hint="Opsional"
        />

        <BaseInput
          v-model="internship.school_supervisor"
          label="Nama Guru / Dosen Pembimbing"
          placeholder="Contoh: Dra. Siti Rahmawati, M.Pd"
          hint="Opsional"
        />
      </div>

      <div class="pt-4 flex flex-col-reverse sm:flex-row justify-between gap-3">
        <BaseButton variant="subtle" class="w-full sm:w-auto justify-center" @click="handlePrev">
          Kembali
        </BaseButton>
        <BaseButton variant="primary" class="w-full sm:w-auto justify-center" @click="handleNext">
          Lanjut ke Jurnal Kegiatan
        </BaseButton>
      </div>
    </div>

    <!-- STEP 3: Real Activities Log -->
    <div v-if="step === 3" class="p-5 sm:p-8 rounded-xl bg-surface border border-border space-y-6">
      <div class="space-y-1 pb-4 border-b border-border">
        <h2 class="text-base font-semibold text-ink-primary">3. Jurnal Kegiatan & Tanggung Jawab Nyata</h2>
        <p class="text-xs text-ink-secondary leading-relaxed">
          Tuliskan secara jelas apa yang benar-benar kamu kerjakan selama magang. Data kegiatan ini akan diformulasikan ke dalam <strong>Bab III (Pelaksanaan Kegiatan PKL)</strong>.
        </p>
      </div>

      <div class="space-y-4">
        <BaseTextarea
          v-model="internship.description"
          label="Catatan Kegiatan & Tanggung Jawab"
          placeholder="Contoh:
1. Melakukan konfigurasi router MikroTik dan switch managed.
2. Membantu troubleshooting koneksi jaringan lokal (LAN) di lantai 2 dan 3.
3. Melakukan backup rutin basis data server operasional setiap hari Jumat.
4. Membuat dokumentasi inventaris perangkat jaringan kantor."
          :rows="8"
          required
          hint="Tuliskan poin-poin kegiatan atau paragraf kegiatan harian"
        />
      </div>

      <div class="pt-4 flex flex-col-reverse sm:flex-row justify-between gap-3">
        <BaseButton variant="subtle" class="w-full sm:w-auto justify-center" @click="handlePrev">
          Kembali
        </BaseButton>
        <BaseButton variant="primary" class="w-full sm:w-auto justify-center" @click="handleNext">
          Review & Konfirmasi Draf
        </BaseButton>
      </div>
    </div>

    <!-- STEP 4: Review & Finalize Draft -->
    <div v-if="step === 4" class="p-5 sm:p-8 rounded-xl bg-surface border border-border space-y-6">
      <div class="space-y-1 pb-4 border-b border-border">
        <h2 class="text-base font-semibold text-ink-primary">4. Konfirmasi Data Laporan</h2>
        <p class="text-xs text-ink-secondary">Periksa kembali data Anda sebelum menyimpan draf dan memulai proses riset.</p>
      </div>

      <div class="space-y-4">
        <BaseInput
          v-model="title"
          label="Judul Laporan"
          placeholder="Contoh: Laporan Praktik Kerja Lapangan pada PT Telkom"
          required
        />

        <!-- Summary Card -->
        <div class="p-4 bg-surface-subtle rounded-lg border border-border-subtle space-y-3 text-xs">
          <div class="grid grid-cols-1 sm:grid-cols-3 gap-1 sm:gap-2">
            <span class="text-ink-muted">Siswa</span>
            <span class="sm:col-span-2 text-ink-primary font-medium">: {{ student.full_name }} ({{ student.student_id }})</span>
          </div>
          <div class="grid grid-cols-1 sm:grid-cols-3 gap-1 sm:gap-2">
            <span class="text-ink-muted">Sekolah / Kampus</span>
            <span class="sm:col-span-2 text-ink-primary">: {{ student.school }}</span>
          </div>
          <div class="grid grid-cols-1 sm:grid-cols-3 gap-1 sm:gap-2">
            <span class="text-ink-muted">Tempat PKL</span>
            <span class="sm:col-span-2 text-ink-primary font-medium">: {{ internship.company_name }}</span>
          </div>
          <div v-if="internship.department" class="grid grid-cols-1 sm:grid-cols-3 gap-1 sm:gap-2">
            <span class="text-ink-muted">Divisi</span>
            <span class="sm:col-span-2 text-ink-secondary">: {{ internship.department }}</span>
          </div>
          <div v-if="internship.role" class="grid grid-cols-1 sm:grid-cols-3 gap-1 sm:gap-2">
            <span class="text-ink-muted">Posisi</span>
            <span class="sm:col-span-2 text-ink-secondary">: {{ internship.role }}</span>
          </div>
          <div v-if="internship.description" class="pt-2 border-t border-border-subtle">
            <span class="text-ink-muted block mb-1">Catatan Kegiatan:</span>
            <p class="text-ink-secondary whitespace-pre-line line-clamp-3">{{ internship.description }}</p>
          </div>
        </div>
      </div>

      <div class="pt-4 flex flex-col-reverse sm:flex-row justify-between gap-3">
        <BaseButton variant="subtle" class="w-full sm:w-auto justify-center" @click="handlePrev">
          Kembali
        </BaseButton>
        <BaseButton
          variant="primary"
          size="lg"
          class="w-full sm:w-auto justify-center"
          :loading="loading"
          @click="handleSubmit"
        >
          Simpan Draf Laporan PKL
        </BaseButton>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Report } from '~/types/api'

interface Props {
  report: Report
  isWatermarked?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  isWatermarked: true
})

const activeTab = ref<'document' | 'pdf'>('document')
const api = useApi()
const previewPdfUrl = computed(() => `${api.apiBase}/reports/${props.report.id}/preview`)

// Format dates
const formatDateRange = (start?: string | null, end?: string | null) => {
  if (!start && !end) return 'Sesuai Masa Kerja Praktik'
  if (start && end) return `${start} s.d. ${end}`
  return start || end || ''
}
</script>

<template>
  <div class="w-full flex flex-col items-center">
    <!-- View Mode Switcher if PDF is ready -->
    <div class="mb-4 flex items-center justify-between w-full max-w-3xl px-2">
      <div class="flex items-center gap-2 text-xs text-ink-muted">
        <span class="font-mono text-[11px] uppercase tracking-wider text-ink-secondary">Format Dokumen Akademik</span>
        <span>·</span>
        <span>A4 Standar Laporan PKL</span>
      </div>

      <div class="flex items-center gap-1 bg-surface-elevated p-0.5 rounded-md border border-border">
        <button
          type="button"
          :class="[
            'px-2.5 py-1 text-xs rounded font-medium transition-colors',
            activeTab === 'document'
              ? 'bg-surface text-ink-primary shadow-subtle border border-border-subtle'
              : 'text-ink-muted hover:text-ink-secondary'
          ]"
          @click="activeTab = 'document'"
        >
          Tampilan Halaman
        </button>
        <button
          type="button"
          :class="[
            'px-2.5 py-1 text-xs rounded font-medium transition-colors',
            activeTab === 'pdf'
              ? 'bg-surface text-ink-primary shadow-subtle border border-border-subtle'
              : 'text-ink-muted hover:text-ink-secondary'
          ]"
          @click="activeTab = 'pdf'"
        >
          PDF Render
        </button>
      </div>
    </div>

    <!-- PDF Iframe View -->
    <div
      v-if="activeTab === 'pdf'"
      class="w-full max-w-3xl h-[500px] sm:h-[750px] md:h-[850px] bg-surface rounded-lg border border-border overflow-hidden shadow-doc relative"
    >
      <iframe
        :src="previewPdfUrl"
        class="w-full h-full border-0"
        title="PDF Preview"
      />
    </div>

    <!-- Structured Academic Document Sheets View -->
    <div
      v-else
      class="w-full max-w-3xl flex flex-col gap-6 sm:gap-8 select-text relative"
    >
      <!-- PAGE 1: COVER SHEET -->
      <article
        :class="[
          'doc-sheet w-full p-5 sm:p-10 md:p-16 rounded-lg text-ink-primary relative overflow-hidden flex flex-col justify-between doc-sheet-page',
          isWatermarked ? 'doc-watermark' : ''
        ]"
      >
        <!-- Watermark Label -->
        <div
          v-if="isWatermarked"
          class="absolute inset-0 flex items-center justify-center pointer-events-none select-none z-10"
        >
          <span class="text-3xl sm:text-5xl md:text-6xl font-black font-sans tracking-widest text-danger-500/10 -rotate-30 uppercase">
            Pratinjau Laporin
          </span>
        </div>

        <div class="text-center space-y-4 sm:space-y-6">
          <div class="space-y-1.5 sm:space-y-2">
            <p class="text-[10px] sm:text-xs uppercase font-mono tracking-widest text-ink-muted">
              Laporan Praktik Kerja Lapangan (PKL)
            </p>
            <h1 class="text-xl sm:text-2xl md:text-3xl font-bold font-serif text-ink-primary leading-tight">
              {{ report.title || 'LAPORAN PRAKTIK KERJA LAPANGAN' }}
            </h1>
          </div>

          <div class="py-2 sm:py-4">
            <p class="text-xs sm:text-sm font-serif italic text-ink-secondary">
              Disusun sebagai salah satu syarat penyelesaian Praktik Kerja Lapangan
            </p>
          </div>
        </div>

        <!-- Middle: Student Details -->
        <div class="my-6 sm:my-8 py-4 sm:py-6 border-y border-border-subtle/40 space-y-2.5 sm:space-y-3 max-w-md mx-auto w-full font-serif text-xs sm:text-sm">
          <div class="grid grid-cols-1 sm:grid-cols-3 gap-0.5 sm:gap-2">
            <span class="text-ink-muted">Nama Siswa</span>
            <span class="sm:col-span-2 font-semibold text-ink-primary">: {{ report.student?.full_name }}</span>
          </div>
          <div class="grid grid-cols-1 sm:grid-cols-3 gap-0.5 sm:gap-2">
            <span class="text-ink-muted">NIS / NIM</span>
            <span class="sm:col-span-2 font-semibold text-ink-primary">: {{ report.student?.student_id }}</span>
          </div>
          <div class="grid grid-cols-1 sm:grid-cols-3 gap-0.5 sm:gap-2">
            <span class="text-ink-muted">Program Keahlian</span>
            <span class="sm:col-span-2 text-ink-secondary">: {{ report.student?.major || 'Teknik Komputer & Jaringan' }}</span>
          </div>
          <div class="grid grid-cols-1 sm:grid-cols-3 gap-0.5 sm:gap-2">
            <span class="text-ink-muted">Tempat PKL</span>
            <span class="sm:col-span-2 font-semibold text-ink-primary">: {{ report.internship?.company_name }}</span>
          </div>
          <div class="grid grid-cols-1 sm:grid-cols-3 gap-0.5 sm:gap-2">
            <span class="text-ink-muted">Periode</span>
            <span class="sm:col-span-2 text-ink-secondary">: {{ formatDateRange(report.internship?.start_date, report.internship?.end_date) }}</span>
          </div>
        </div>

        <!-- Bottom: Institution & Year -->
        <div class="text-center space-y-1 font-serif">
          <p class="text-sm sm:text-base font-bold uppercase tracking-wide text-ink-primary">
            {{ report.student?.school }}
          </p>
          <p class="text-xs text-ink-muted font-mono">
            {{ new Date().getFullYear() }}
          </p>
        </div>
      </article>

      <!-- PAGE 2: BAB I & BAB II -->
      <article
        :class="[
          'doc-sheet w-full p-5 sm:p-10 md:p-14 rounded-lg text-ink-primary relative overflow-hidden font-serif space-y-6 sm:space-y-8',
          isWatermarked ? 'doc-watermark' : ''
        ]"
      >
        <!-- Watermark Label -->
        <div
          v-if="isWatermarked"
          class="absolute inset-0 flex items-center justify-center pointer-events-none select-none z-10"
        >
          <span class="text-3xl sm:text-5xl md:text-6xl font-black font-sans tracking-widest text-danger-500/10 -rotate-30 uppercase">
            Pratinjau Laporin
          </span>
        </div>

        <!-- BAB I -->
        <section class="space-y-3 sm:space-y-4">
          <div class="text-center space-y-1 pb-2 border-b border-border-subtle/30">
            <h2 class="text-base sm:text-lg font-bold text-ink-primary tracking-wide">BAB I</h2>
            <h3 class="text-sm sm:text-base font-semibold text-ink-secondary">PENDAHULUAN</h3>
          </div>

          <div class="text-xs sm:text-sm leading-relaxed text-ink-secondary space-y-3 text-justify">
            <div v-if="report.generated_sections?.introduction">
              <p class="whitespace-pre-line">{{ report.generated_sections.introduction }}</p>
            </div>
            <div v-else class="space-y-2">
              <p>
                Praktik Kerja Lapangan (PKL) merupakan program pelatihan dan pembelajaran kejuruan yang diselenggarakan untuk meningkatkan kompetensi keahlian siswa sesuai dengan kebutuhan dunia usaha dan dunia industri.
              </p>
              <p>
                Pelaksanaan PKL di <strong class="text-ink-primary">{{ report.internship?.company_name }}</strong> memberikan kesempatan nyata bagi penulis untuk memahami dinamika operasional, etika kerja, serta penerapan teori kejuruan dalam proyek kerja harian.
              </p>
            </div>
          </div>
        </section>

        <!-- BAB II -->
        <section class="space-y-3 sm:space-y-4 pt-6 border-t border-border-subtle/30">
          <div class="text-center space-y-1 pb-2 border-b border-border-subtle/30">
            <h2 class="text-base sm:text-lg font-bold text-ink-primary tracking-wide">BAB II</h2>
            <h3 class="text-sm sm:text-base font-semibold text-ink-secondary">PROFIL DAN TINJAUAN PERUSAHAAN</h3>
          </div>

          <div class="text-xs sm:text-sm leading-relaxed text-ink-secondary space-y-3 text-justify">
            <div v-if="report.generated_sections?.company_profile">
              <p class="whitespace-pre-line">{{ report.generated_sections.company_profile }}</p>
            </div>
            <div v-else class="space-y-2">
              <p>
                <strong class="text-ink-primary">{{ report.internship?.company_name }}</strong> merupakan institusi/perusahaan yang berlokasi di {{ report.internship?.company_address || 'wilayah operasional terkait' }}, bergerak di bidang penyediaan layanan profesional dan teknologi.
              </p>
              <p v-if="report.internship?.department">
                Penulis ditempatkan pada divisi/departemen <span class="text-ink-primary">{{ report.internship.department }}</span> yang bertanggung jawab atas tata kelola dan eksekusi teknis harian.
              </p>
            </div>
          </div>
        </section>
      </article>

      <!-- PAGE 3: BAB III & BAB IV -->
      <article
        :class="[
          'doc-sheet w-full p-5 sm:p-10 md:p-14 rounded-lg text-ink-primary relative overflow-hidden font-serif space-y-6 sm:space-y-8',
          isWatermarked ? 'doc-watermark' : ''
        ]"
      >
        <!-- Watermark Label -->
        <div
          v-if="isWatermarked"
          class="absolute inset-0 flex items-center justify-center pointer-events-none select-none z-10"
        >
          <span class="text-3xl sm:text-5xl md:text-6xl font-black font-sans tracking-widest text-danger-500/10 -rotate-30 uppercase">
            Pratinjau Laporin
          </span>
        </div>

        <!-- BAB III -->
        <section class="space-y-3 sm:space-y-4">
          <div class="text-center space-y-1 pb-2 border-b border-border-subtle/30">
            <h2 class="text-base sm:text-lg font-bold text-ink-primary tracking-wide">BAB III</h2>
            <h3 class="text-sm sm:text-base font-semibold text-ink-secondary">PELAKSANAAN KEGIATAN PKL</h3>
          </div>

          <div class="text-xs sm:text-sm leading-relaxed text-ink-secondary space-y-3 text-justify">
            <div v-if="report.generated_sections?.activities">
              <p class="whitespace-pre-line">{{ report.generated_sections.activities }}</p>
            </div>
            <div v-else class="space-y-2">
              <p>
                Selama melaksanakan Praktik Kerja Lapangan terhitung sejak {{ formatDateRange(report.internship?.start_date, report.internship?.end_date) }}, kegiatan yang dilakukan meliputi:
              </p>
              <p v-if="report.internship?.description" class="bg-surface-subtle/50 p-3 sm:p-4 rounded border border-border-subtle/30 text-ink-primary">
                {{ report.internship.description }}
              </p>
            </div>
          </div>
        </section>

        <!-- BAB IV -->
        <section class="space-y-3 sm:space-y-4 pt-6 border-t border-border-subtle/30">
          <div class="text-center space-y-1 pb-2 border-b border-border-subtle/30">
            <h2 class="text-base sm:text-lg font-bold text-ink-primary tracking-wide">BAB IV</h2>
            <h3 class="text-sm sm:text-base font-semibold text-ink-secondary">KESIMPULAN DAN SARAN</h3>
          </div>

          <div class="text-xs sm:text-sm leading-relaxed text-ink-secondary space-y-3 text-justify">
            <div v-if="report.generated_sections?.conclusion">
              <p class="whitespace-pre-line">{{ report.generated_sections.conclusion }}</p>
            </div>
            <div v-else class="space-y-2">
              <p>
                Berdasarkan pelaksanaan Praktik Kerja Lapangan di <strong class="text-ink-primary">{{ report.internship?.company_name }}</strong>, penulis menyimpulkan bahwa program ini sangat efektif dalam menjembatani kesiapan siswa menuju standar profesional.
              </p>
            </div>
          </div>
        </section>
      </article>
    </div>
  </div>
</template>

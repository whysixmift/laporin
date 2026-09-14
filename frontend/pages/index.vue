<script setup lang="ts">
const { isAuthenticated } = useAuth()

const activeChapter = ref<'bab1' | 'bab2' | 'bab3' | 'bab4'>('bab1')

const chapters = {
  bab1: {
    number: 'BAB I',
    title: 'Pendahuluan & Tujuan Praktik',
    desc: 'Memuat landasan pelaksanaan PKL, tujuan umum dan khusus magang, serta batasan waktu dan tempat pelaksanaan sesuai kurikulum vokasi/perguruan tinggi.',
    items: ['Latar Belakang Praktik Kerja Lapangan', 'Maksud dan Tujuan Magang', 'Manfaat bagi Siswa, Sekolah, dan DU/DI', 'Waktu dan Lokasi Pelaksanaan']
  },
  bab2: {
    number: 'BAB II',
    title: 'Gambaran Umum Perusahaan (DU/DI)',
    desc: 'Otomatis menelusuri profil resmi tempat magang, sejarah singkat instansi, visi dan misi perusahaan, hingga bagan struktur organisasi.',
    items: ['Sejarah Singkat dan Profil DU/DI', 'Visi, Misi, dan Nilai Inti Perusahaan', 'Struktur Organisasi & Divisi Kerja', 'Tata Tertib dan Keselamatan Kerja (K3)']
  },
  bab3: {
    number: 'BAB III',
    title: 'Pelaksanaan & Pembahasan Praktik Kerja',
    desc: 'Menguraikan jurnal kegiatan harian, tugas teknis yang diselesaikan, kendala di tempat kerja, serta solusi profesional yang diterapkan.',
    items: ['Deskripsi Bidang Tugas Magang', 'Uraian Jurnal Kegiatan Praktik', 'Kendala Teknis yang Dihadapi', 'Analisis Masalah & Solusi Kerja']
  },
  bab4: {
    number: 'BAB IV',
    title: 'Penutup, Saran & Lampiran',
    desc: 'Ringkasan capaian kompetensi magang, saran konstruktif untuk sekolah dan pihak industri, serta format lampiran foto kegiatan.',
    items: ['Kesimpulan Akhir Magang', 'Saran untuk Pihak Sekolah/Kampus', 'Saran Pengembangan bagi DU/DI', 'Daftar Pustaka & Lampiran']
  }
}
</script>

<template>
  <div class="space-y-20 sm:space-y-28 py-8 sm:py-16">
    <!-- HERO SECTION: Academic Craft & Editorial Excellence -->
    <section class="max-w-6xl mx-auto px-4 sm:px-6">
      <div class="grid grid-cols-1 lg:grid-cols-12 gap-12 lg:gap-14 items-center">
        <!-- Left: Editorial Value Proposition -->
        <div class="lg:col-span-7 space-y-6">
          <div class="inline-flex items-center gap-2 px-3 py-1.5 rounded-full bg-surface border border-border text-xs text-ink-secondary">
            <span class="w-2 h-2 rounded-full bg-emerald-400" />
            <span class="font-mono text-[11px]">Standar Margin 4-4-3-3 & Pedoman Resmi PKL Indonesia</span>
          </div>

          <h1 class="text-3xl sm:text-4xl lg:text-[46px] font-bold tracking-tight text-ink-primary leading-[1.2]">
            Laporan PKL kamu, <br />
            <span class="font-serif italic font-normal text-emerald-400">tersusun rapi dan siap jilid.</span>
          </h1>

          <p class="text-base sm:text-lg text-ink-secondary leading-relaxed max-w-xl font-normal">
            Cukup tuliskan catatan kegiatan harian magangmu. Laporin menyusun profil tempat PKL, merapikan Bab I hingga Bab IV dengan tata bahasa baku, dan mengekspor dokumen Word (.docx) siap cetak.
          </p>

          <!-- Primary Actions -->
          <div class="flex flex-col sm:flex-row items-stretch sm:items-center gap-3 pt-2">
            <BaseButton
              v-if="isAuthenticated"
              to="/reports/new"
              size="lg"
              variant="primary"
              class="w-full sm:w-auto justify-center font-semibold"
            >
              <template #leading>
                <svg class="w-4 h-4" viewBox="0 0 20 20" fill="currentColor">
                  <path fill-rule="evenodd" d="M10 3a1 1 0 011 1v5h5a1 1 0 110 2h-5v5a1 1 0 11-2 0v-5H4a1 1 0 110-2h5V4a1 1 0 011-1z" clip-rule="evenodd" />
                </svg>
              </template>
              Susun Laporan PKL Baru
            </BaseButton>

            <BaseButton
              v-else
              to="/register"
              size="lg"
              variant="primary"
              class="w-full sm:w-auto justify-center font-semibold"
            >
              Mulai Susun Laporan
            </BaseButton>

            <BaseButton
              to="#struktur"
              size="lg"
              variant="subtle"
              class="w-full sm:w-auto justify-center"
            >
              Pelajari Struktur Bab
            </BaseButton>
          </div>

          <!-- Trust & Value Details -->
          <div class="flex flex-wrap items-center gap-3 sm:gap-5 pt-4 text-xs text-ink-muted border-t border-border/70">
            <div class="flex items-center gap-1.5 text-ink-primary font-mono font-bold">
              <span>Rp15.000</span>
              <span class="font-sans font-normal text-ink-secondary">/ file Word (.docx)</span>
            </div>
            <span class="text-border">|</span>
            <div>Tanpa langganan bulanan</div>
            <span class="text-border">|</span>
            <div>Pratinjau lengkap gratis sebelum bayar</div>
          </div>
        </div>

        <!-- Right: Realistic Physical Document Sheet -->
        <div class="lg:col-span-5 relative">
          <div class="w-full bg-[#181c24] border border-border-strong rounded-xl p-6 shadow-doc relative overflow-hidden">
            <!-- Paper Header -->
            <div class="flex items-center justify-between pb-3 border-b border-border/80 text-xs text-ink-muted">
              <div class="flex items-center gap-2">
                <span class="w-2.5 h-2.5 rounded-full bg-emerald-400" />
                <span class="font-mono text-[11px] text-ink-primary">LAPORAN_PKL_FINAL.DOCX</span>
              </div>
              <span class="font-mono text-[10px] bg-canvas px-2 py-0.5 rounded border border-border text-ink-muted">
                Format A4 · Margin 4-4-3-3
              </span>
            </div>

            <!-- Authentic Printed Document Sheet Simulation -->
            <div class="mt-4 p-5 sm:p-6 bg-[#0f1217] rounded-lg border border-border/60 shadow-inner space-y-4">
              <div class="text-center pb-3 border-b border-border/40 space-y-1">
                <div class="text-[11px] font-mono tracking-widest text-ink-muted uppercase">
                  LAPORAN PRAKTIK KERJA LAPANGAN
                </div>
                <div class="text-sm font-serif font-bold text-ink-primary">
                  PT TEKNOLOGI NUSANTARA MANDIRI
                </div>
                <div class="text-[10px] text-ink-secondary">
                  Tahun Ajaran 2025/2026 · Divisi Rekayasa Perangkat Lunak
                </div>
              </div>

              <!-- Excerpt Body with Serif Typography -->
              <div class="space-y-3 font-serif text-xs leading-relaxed text-ink-secondary text-justify">
                <div class="font-sans font-bold text-[10px] text-emerald-400 uppercase tracking-wider">
                  BAB I · PENDAHULUAN
                </div>
                <p class="indent-6">
                  Praktik Kerja Lapangan (PKL) merupakan program pembelajaran yang mengintegrasikan teori akademik di sekolah dengan praktik kerja nyata di Dunia Usaha dan Dunia Industri (DU/DI). Kegiatan ini bertujuan membekali peserta didik dengan etos kerja profesional serta keterampilan teknis yang relevan dengan standar industri.
                </p>
                <p class="indent-6">
                  Selama masa pelaksanaan magang, peserta didik berkesempatan mengamati dan terlibat langsung dalam alur kerja pengembangan sistem informasi dan pemeliharaan infrastruktur...
                </p>
              </div>

              <!-- Verified Badge -->
              <div class="pt-3 border-t border-border/40 flex items-center justify-between text-[11px] font-sans">
                <span class="text-emerald-400 font-medium flex items-center gap-1.5">
                  <svg class="w-3.5 h-3.5" viewBox="0 0 20 20" fill="currentColor">
                    <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd" />
                  </svg>
                  Format Bab Lengkap & Terstruktur
                </span>
                <span class="font-mono text-ink-muted text-[10px]">Hal. 1 dari 18</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- SECTION 2: Interactive Chapter Inspector (Academic Structure) -->
    <section id="struktur" class="max-w-6xl mx-auto px-4 sm:px-6">
      <div class="space-y-3 mb-8 sm:mb-10 text-center sm:text-left">
        <h2 class="text-xs uppercase font-mono tracking-widest text-emerald-400">Struktur Dokumen</h2>
        <h3 class="text-2xl sm:text-3xl font-bold text-ink-primary">
          Format Bab Lengkap Sesuai Pedoman PKL
        </h3>
        <p class="text-sm text-ink-secondary max-w-2xl">
          Setiap bab dirancang mengikuti struktur baku laporan praktik kerja lapangan yang diakui oleh SMK dan perguruan tinggi vokasi di Indonesia.
        </p>
      </div>

      <div class="grid grid-cols-1 lg:grid-cols-12 gap-6 items-start">
        <!-- Tab Selector List -->
        <div class="lg:col-span-4 space-y-2">
          <button
            v-for="(chap, key) in chapters"
            :key="key"
            type="button"
            class="w-full text-left p-4 rounded-xl border transition-all cursor-pointer flex items-center justify-between"
            :class="activeChapter === key
              ? 'bg-surface-elevated border-emerald-500/50 shadow-subtle'
              : 'bg-surface border-border hover:bg-surface-elevated/60 text-ink-secondary'"
            @click="activeChapter = key"
          >
            <div>
              <div class="text-[11px] font-mono font-bold uppercase" :class="activeChapter === key ? 'text-emerald-400' : 'text-ink-muted'">
                {{ chap.number }}
              </div>
              <div class="text-sm font-semibold text-ink-primary mt-0.5">
                {{ chap.title }}
              </div>
            </div>
            <svg
              class="w-4 h-4 shrink-0 transition-transform"
              :class="activeChapter === key ? 'text-emerald-400 translate-x-0.5' : 'text-ink-muted'"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
            </svg>
          </button>
        </div>

        <!-- Chapter Detail Card -->
        <div class="lg:col-span-8 p-6 sm:p-8 bg-surface rounded-xl border border-border space-y-6">
          <div class="space-y-2">
            <div class="inline-flex items-center gap-2 font-mono text-xs font-bold text-emerald-400 uppercase">
              <span>{{ chapters[activeChapter].number }}</span>
              <span>·</span>
              <span>{{ chapters[activeChapter].title }}</span>
            </div>
            <p class="text-sm sm:text-base text-ink-secondary leading-relaxed">
              {{ chapters[activeChapter].desc }}
            </p>
          </div>

          <div class="pt-4 border-t border-border/80">
            <div class="text-xs font-bold text-ink-primary uppercase font-mono tracking-wider mb-3">
              Sub-Bab & Pokok Bahasan:
            </div>
            <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
              <div
                v-for="(item, idx) in chapters[activeChapter].items"
                :key="idx"
                class="p-3 bg-surface-elevated rounded-lg border border-border/60 text-xs text-ink-primary flex items-start gap-2.5"
              >
                <span class="w-4 h-4 rounded-full bg-emerald-500/10 text-emerald-400 flex items-center justify-center font-mono font-bold text-[10px] shrink-0 mt-0.5">
                  ✓
                </span>
                <span>{{ item }}</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- SECTION 3: Real Comparison (Catatan Harian vs Laporan Siap Jilid) -->
    <section class="max-w-6xl mx-auto px-4 sm:px-6">
      <div class="p-8 sm:p-10 bg-surface rounded-2xl border border-border space-y-8">
        <div class="space-y-2 text-center max-w-2xl mx-auto">
          <h2 class="text-xs uppercase font-mono tracking-widest text-emerald-400">Transformasi Dokumen</h2>
          <h3 class="text-2xl sm:text-3xl font-bold text-ink-primary">
            Dari Catatan Kasar Menjadi Format Akademik
          </h3>
          <p class="text-xs sm:text-sm text-ink-secondary">
            Tidak perlu pusing memikirkan kalimat pengantar formal, margin kertas, atau format nomor halaman.
          </p>
        </div>

        <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
          <!-- Left: Siswa Input -->
          <div class="p-5 bg-canvas rounded-xl border border-border/80 space-y-3">
            <div class="flex items-center justify-between pb-2 border-b border-border text-xs">
              <span class="font-mono text-ink-muted">Input Catatan Siswa</span>
              <span class="text-[10px] px-2 py-0.5 rounded bg-surface-elevated text-amber-400 font-mono">Bahasa Sehari-hari</span>
            </div>
            <div class="text-xs font-mono text-ink-secondary leading-relaxed bg-surface/50 p-4 rounded border border-border-subtle">
              "Minggu ke-3: Gw disuruh instalasi ulang 5 PC kantor pake OS Windows 11, abis itu konfigurasi IP address lokal sama setting sharing printer biar divisi accounting bisa ngeprint bareng."
            </div>
          </div>

          <!-- Right: Laporin Formal Output -->
          <div class="p-5 bg-surface-elevated rounded-xl border border-emerald-500/40 space-y-3">
            <div class="flex items-center justify-between pb-2 border-b border-border text-xs">
              <span class="font-mono text-emerald-400 font-semibold">Hasil Rumusan Bab III</span>
              <span class="text-[10px] px-2 py-0.5 rounded bg-emerald-500/10 text-emerald-300 font-mono">Baku & Akademik</span>
            </div>
            <div class="text-xs font-serif text-ink-primary leading-relaxed bg-[#0f1217] p-4 rounded border border-border-subtle">
              "3.2.3 Pemeliharaan Perangkat Keras dan Konfigurasi Jaringan Lokal<br>
              Pada minggu ketiga pelaksanaan praktik kerja, praktikan melaksanakan instalasi sistem operasi Windows 11 pada 5 (lima) unit komputer kerja serta melakukan konfigurasi alamat IP statis dan integrasi network printer sharing untuk kebutuhan operasional divisi keuangan."
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- SECTION 4: Transparent Pricing & Direct Admin Contact -->
    <section class="max-w-6xl mx-auto px-4 sm:px-6">
      <div class="p-8 sm:p-12 bg-surface-elevated rounded-2xl border border-border flex flex-col md:flex-row items-start md:items-center justify-between gap-8">
        <div class="space-y-4 max-w-xl">
          <div class="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-emerald-500/10 text-emerald-400 text-xs font-semibold">
            Buka Kunci Dokumen
          </div>
          <h3 class="text-2xl sm:text-3xl font-bold text-ink-primary">
            Satu kali bayar per dokumen. Pratinjau bebas sebelum bayar.
          </h3>
          <p class="text-sm text-ink-secondary leading-relaxed">
            Periksa seluruh susunan laporanmu secara gratis. Jika kamu sudah puas dengan isi draft laporan, buka kunci berkas Microsoft Word asli untuk pengeditan dan pencetakan akhir.
          </p>
        </div>

        <div class="p-6 bg-surface rounded-xl border border-border text-center sm:text-right shrink-0 w-full md:w-80 space-y-4">
          <div>
            <span class="text-xs text-ink-muted block uppercase font-mono tracking-wider">Tarif Unduh Dokumen</span>
            <div class="text-3xl font-black font-mono text-ink-primary my-1">
              Rp15.000
            </div>
            <span class="text-xs text-emerald-400 block font-medium">QRIS, Virtual Account, & Bank Transfer</span>
          </div>

          <BaseButton
            v-if="isAuthenticated"
            to="/reports/new"
            variant="primary"
            size="lg"
            class="w-full justify-center font-semibold"
          >
            Buat Laporan Sekarang
          </BaseButton>
          <BaseButton
            v-else
            to="/register"
            variant="primary"
            size="lg"
            class="w-full justify-center font-semibold"
          >
            Daftar & Mulai
          </BaseButton>

          <div class="pt-3 border-t border-border text-[11px] text-ink-muted text-center space-y-1">
            <div>Kendala atau ingin transfer manual langsung?</div>
            <div class="flex items-center justify-center gap-2 font-mono">
              <a href="https://wa.me/6285117206413" target="_blank" rel="noopener noreferrer" class="text-emerald-400 hover:underline">WA 1</a>
              <span>·</span>
              <a href="https://wa.me/6288809028653" target="_blank" rel="noopener noreferrer" class="text-emerald-400 hover:underline">WA 2</a>
              <span>·</span>
              <a href="mailto:miftasigma11@gmail.com" class="text-sky-400 hover:underline">Email</a>
            </div>
          </div>
        </div>
      </div>
    </section>
  </div>
</template>

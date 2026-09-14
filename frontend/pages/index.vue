<script setup lang="ts">
const { isAuthenticated } = useAuth()
</script>

<template>
  <div class="space-y-20 sm:space-y-32 pb-24 overflow-hidden">
    <!-- ========================================== -->
    <!-- ACT I: CINEMATIC 3D DOCUMENT HERO           -->
    <!-- ========================================== -->
    <section class="relative pt-6 sm:pt-12">
      <!-- Ambient Studio Perspective Floor Grid -->
      <div class="absolute inset-0 pointer-events-none opacity-20 bg-[linear-gradient(to_right,#1f293d_1px,transparent_1px),linear-gradient(to_bottom,#1f293d_1px,transparent_1px)] bg-[size:4rem_4rem] [mask-image:radial-gradient(ellipse_60%_50%_at_50%_30%,#000_70%,transparent_100%)]" />

      <div class="max-w-6xl mx-auto px-4 sm:px-6 relative z-10">
        <!-- Hero Header -->
        <div class="text-center space-y-5 max-w-3xl mx-auto">
          <div class="inline-flex items-center gap-2 px-3.5 py-1.5 rounded-full bg-[#141a24] border border-border text-xs text-ink-secondary shadow-subtle">
            <span class="w-2 h-2 rounded-full bg-emerald-400 animate-pulse" />
            <span class="font-mono text-[11px] text-ink-primary font-semibold">Format Resmi Kemendikbud & DU/DI Vokasi</span>
          </div>

          <h1 class="text-3xl sm:text-5xl lg:text-6xl font-extrabold tracking-tight text-ink-primary leading-[1.15] font-sans">
            Laporan PKL kamu, <br />
            <span class="font-serif italic font-normal text-emerald-400">tersusun rapi & siap jilid.</span>
          </h1>

          <p class="text-sm sm:text-lg text-ink-secondary leading-relaxed font-normal max-w-2xl mx-auto">
            Ubah catatan kegiatan magang yang berantakan menjadi dokumen laporan resmi Bab I s.d. Bab IV dengan margin 4-4-3-3, format Word (.docx), dan pratinjau instan.
          </p>

          <!-- Primary Actions -->
          <div class="flex flex-col sm:flex-row items-center justify-center gap-3 pt-2">
            <BaseButton
              v-if="isAuthenticated"
              to="/reports/new"
              size="lg"
              variant="primary"
              class="w-full sm:w-auto justify-center font-semibold px-8 shadow-elevated"
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
              class="w-full sm:w-auto justify-center font-semibold px-8 shadow-elevated"
            >
              Mulai Susun Laporan
            </BaseButton>

            <BaseButton
              to="#alur"
              size="lg"
              variant="subtle"
              class="w-full sm:w-auto justify-center"
            >
              Pelajari Alur Kerja
            </BaseButton>
          </div>
        </div>

        <!-- 3D Interactive WebGL Document Viewport -->
        <div class="mt-4 sm:mt-6">
          <HeroDocumentScene />
        </div>
      </div>
    </section>

    <!-- ========================================== -->
    <!-- ACT II: TRANSFORMATION STORY (RAW -> FINAL) -->
    <!-- ========================================== -->
    <section id="alur" class="max-w-6xl mx-auto px-4 sm:px-6">
      <TransformationVisualizer />
    </section>

    <!-- ========================================== -->
    <!-- ACT III: 3D CHAPTER LAYER ATELIER          -->
    <!-- ========================================== -->
    <section class="max-w-6xl mx-auto px-4 sm:px-6">
      <div class="text-center space-y-3 max-w-3xl mx-auto mb-6">
        <div class="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-emerald-500/10 border border-emerald-500/30 text-xs text-emerald-400 font-mono">
          ARSITEKTUR DOKUMEN 3D
        </div>
        <h2 class="text-3xl sm:text-4xl font-bold text-ink-primary font-sans tracking-tight">
          Struktur Lengkap Bab I s.d. Bab IV
        </h2>
        <p class="text-sm text-ink-secondary">
          Setiap lembar disusun dengan hierarki judul, nomor halaman, dan rincian sub-bab yang baku.
        </p>
      </div>

      <ChapterStack3D />
    </section>

    <!-- ========================================== -->
    <!-- ACT IV: ACADEMIC FORMAT BLUEPRINT           -->
    <!-- ========================================== -->
    <section class="max-w-6xl mx-auto px-4 sm:px-6">
      <div class="p-8 sm:p-12 bg-[#121620] border border-border-strong rounded-2xl space-y-8 shadow-doc">
        <div class="text-center sm:text-left space-y-2">
          <div class="text-xs font-mono font-bold text-emerald-400 uppercase">STANDAR PEDOMAN NASIONAL</div>
          <h3 class="text-2xl sm:text-3xl font-bold text-ink-primary font-sans">
            Spesifikasi Dokumen Siap Cetak
          </h3>
          <p class="text-xs sm:text-sm text-ink-secondary max-w-2xl">
            Laporin menerapkan standarisasi penulisan karya tulis ilmiah dan laporan vokasi sesuai panduan resmi institusi.
          </p>
        </div>

        <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
          <div class="p-5 rounded-xl bg-surface border border-border space-y-2">
            <div class="text-xs font-mono text-emerald-400 font-bold">01 · MARGIN KERTAS</div>
            <div class="text-base font-bold text-ink-primary">4-4-3-3 cm</div>
            <p class="text-xs text-ink-secondary">
              Kiri 4 cm (ruang jilid), Atas 4 cm, Kanan 3 cm, Bawah 3 cm pada kertas A4 80 gsm.
            </p>
          </div>

          <div class="p-5 rounded-xl bg-surface border border-border space-y-2">
            <div class="text-xs font-mono text-emerald-400 font-bold">02 · TIPOGRAFI BAKU</div>
            <div class="text-base font-bold text-ink-primary">Times New Roman 12pt</div>
            <p class="text-xs text-ink-secondary">
              Spasi 1.5, teks rata kiri-kanan (justified), dan format indent paragraf 1.25 cm.
            </p>
          </div>

          <div class="p-5 rounded-xl bg-surface border border-border space-y-2">
            <div class="text-xs font-mono text-emerald-400 font-bold">03 · LEMBAR PENGESAHAN</div>
            <div class="text-base font-bold text-ink-primary">Tanda Tangan DU/DI</div>
            <p class="text-xs text-ink-secondary">
              Kolom tanda tangan pembimbing sekolah, pembimbing lapangan, dan kepala program keahlian.
            </p>
          </div>

          <div class="p-5 rounded-xl bg-surface border border-border space-y-2">
            <div class="text-xs font-mono text-emerald-400 font-bold">04 · EKSPOR ASLI</div>
            <div class="text-base font-bold text-ink-primary">Microsoft Word (.docx)</div>
            <p class="text-xs text-ink-secondary">
              File dokumen utuh tanpa proteksi yang dapat diedit langsung di Word atau LibreOffice.
            </p>
          </div>
        </div>
      </div>
    </section>

    <!-- ========================================== -->
    <!-- ACT V: TRANSPARENT PRICING & DIRECT ACCESS  -->
    <!-- ========================================== -->
    <section class="max-w-6xl mx-auto px-4 sm:px-6">
      <div class="p-8 sm:p-12 bg-gradient-to-b from-[#141923] to-[#0f1217] rounded-2xl border border-border-strong flex flex-col md:flex-row items-start md:items-center justify-between gap-8 shadow-doc">
        <div class="space-y-4 max-w-xl">
          <div class="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-emerald-500/10 text-emerald-400 text-xs font-mono font-bold">
            TARIF TRANSPARAN
          </div>
          <h3 class="text-2xl sm:text-3xl font-bold text-ink-primary font-sans leading-snug">
            Satu harga per dokumen. <br />
            Periksa seluruh isi lewat pratinjau gratis.
          </h3>
          <p class="text-sm text-ink-secondary leading-relaxed font-serif">
            Kamu dapat melihat draft pratinjau dokumen dari Bab I hingga Bab IV terlebih dahulu. Jika isi dan strukturnya sudah sesuai, buka kunci file Word (.docx) asli seharga Rp15.000 untuk pengeditan dan pencetakan akhir.
          </p>
        </div>

        <div class="p-6 bg-surface rounded-xl border border-border text-center sm:text-right shrink-0 w-full md:w-80 space-y-4 shadow-elevated">
          <div>
            <span class="text-xs text-ink-muted block uppercase font-mono tracking-wider">Total Biaya Unduh</span>
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
              <a href="https://wa.me/6285117206413" target="_blank" rel="noopener noreferrer" class="text-emerald-400 hover:underline">WA 1 (0851-1720-6413)</a>
              <span>·</span>
              <a href="https://wa.me/6288809028653" target="_blank" rel="noopener noreferrer" class="text-emerald-400 hover:underline">WA 2</a>
            </div>
          </div>
        </div>
      </div>
    </section>
  </div>
</template>

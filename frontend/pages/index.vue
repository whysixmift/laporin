<script setup lang="ts">
const { isAuthenticated } = useAuth()
</script>

<template>
  <div class="space-y-24 sm:space-y-36 pb-28 overflow-hidden text-ink-primary">
    <!-- ================================================================= -->
    <!-- HERO: ATMOSPHERIC GALLERY + SIMPLE PRODUCT MESSAGE                -->
    <!-- ================================================================= -->
    <section class="relative min-h-[580px] sm:min-h-[660px] flex items-center">
      <!-- 3D Contemporary Indonesian Geometric Ornament (Atmosphere & Visual Identity) -->
      <div class="absolute right-0 top-0 w-full sm:w-3/5 h-full opacity-60 sm:opacity-90 pointer-events-auto z-0">
        <IndonesianOrnamentScene />
      </div>

      <!-- Subtle Geometric Linework Accent (Woven pattern background grid) -->
      <div class="absolute inset-0 pointer-events-none opacity-10 bg-[radial-gradient(#8a7a60_1px,transparent_1px)] [background-size:24px_24px]" />

      <!-- Left Hero Typography & Direct Action -->
      <div class="max-w-6xl mx-auto px-4 sm:px-6 w-full relative z-10">
        <div class="max-w-xl space-y-6">
          <div class="inline-flex items-center gap-2.5 text-xs font-mono text-ink-muted">
            <span class="w-1.5 h-1.5 rounded-full bg-emerald-400" />
            <span class="uppercase tracking-widest text-[11px] text-ink-secondary font-semibold">
              Platform Penyusun Laporan PKL
            </span>
          </div>

          <h1 class="text-4xl sm:text-6xl font-serif font-normal tracking-tight text-ink-primary leading-[1.1]">
            Laporin<span class="text-emerald-400 font-sans">.</span>
          </h1>

          <p class="text-base sm:text-lg text-ink-secondary leading-relaxed font-sans font-normal">
            Alat bantu menyusun laporan Praktik Kerja Lapangan (PKL) dan magang. Dari catatan kegiatan harian menjadi naskah akademik siap cetak.
          </p>

          <div class="pt-2 flex flex-col sm:flex-row items-stretch sm:items-center gap-4">
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
              Buat Laporan Baru
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

            <NuxtLink
              to="#cara-kerja"
              class="text-xs sm:text-sm font-mono text-ink-muted hover:text-ink-primary transition-colors flex items-center justify-center sm:justify-start gap-1.5 py-2.5"
            >
              <span>Pelajari Alur Kerja</span>
              <span>↓</span>
            </NuxtLink>
          </div>

          <div class="pt-4 border-t border-border/60 flex items-center gap-4 text-xs font-mono text-ink-muted">
            <span>Rp15.000 / file Word (.docx)</span>
            <span>·</span>
            <span>Pratinjau gratis</span>
          </div>
        </div>
      </div>
    </section>

    <!-- ================================================================= -->
    <!-- WORKFLOW DEMO: REAL DEMONSTRATION (NO FAKE MARKETING CARDS)       -->
    <!-- ================================================================= -->
    <section id="cara-kerja" class="max-w-5xl mx-auto px-4 sm:px-6 space-y-16">
      <div class="space-y-2 border-b border-border/80 pb-4">
        <span class="text-xs font-mono text-emerald-400 font-bold uppercase tracking-widest">ALUR DOKUMEN</span>
        <h2 class="text-2xl sm:text-3xl font-serif font-normal text-ink-primary">
          Bagaimana Laporin bekerja
        </h2>
      </div>

      <!-- Step 01 -->
      <div class="grid grid-cols-1 md:grid-cols-12 gap-6 items-start">
        <div class="md:col-span-4 space-y-1">
          <span class="text-xs font-mono text-ink-muted">01 / INPUT</span>
          <h3 class="text-lg font-bold text-ink-primary">Tulis catatan harian kamu</h3>
          <p class="text-xs sm:text-sm text-ink-secondary leading-relaxed">
            Cukup masukkan nama tempat magang dan ringkasan apa saja yang kamu kerjakan dalam bahasa santai sehari-hari.
          </p>
        </div>

        <div class="md:col-span-8 p-5 bg-[#121620] border border-border rounded-xl font-mono text-xs text-ink-secondary space-y-2 shadow-subtle">
          <div class="text-[11px] text-ink-muted flex items-center justify-between pb-2 border-b border-border-subtle">
            <span>Contoh Catatan Masukan Siswa:</span>
            <span class="text-amber-400">Bahasa Sehari-hari</span>
          </div>
          <p class="text-ink-primary leading-relaxed">
            "Minggu ke-3: Gw disuruh instalasi ulang 5 PC kantor pake OS Windows 11, abis itu konfigurasi IP address lokal sama setting sharing printer biar divisi accounting bisa ngeprint bareng."
          </p>
        </div>
      </div>

      <!-- Step 02 -->
      <div class="grid grid-cols-1 md:grid-cols-12 gap-6 items-start">
        <div class="md:col-span-4 space-y-1">
          <span class="text-xs font-mono text-ink-muted">02 / RISET</span>
          <h3 class="text-lg font-bold text-ink-primary">Profil perusahaan diriset otomatis</h3>
          <p class="text-xs sm:text-sm text-ink-secondary leading-relaxed">
            Laporin menelusuri data resmi instansi untuk melengkapi Bab II: sejarah perusahaan, visi misi, dan struktur kerja.
          </p>
        </div>

        <div class="md:col-span-8 p-5 bg-[#121620] border border-border rounded-xl font-mono text-xs text-ink-secondary space-y-2 shadow-subtle">
          <div class="text-[11px] text-emerald-400 font-bold flex items-center justify-between pb-2 border-b border-border-subtle">
            <span>Hasil Ekstraksi Data Perusahaan:</span>
            <span class="px-2 py-0.5 rounded bg-emerald-500/10 text-[10px]">Terverifikasi</span>
          </div>
          <div class="space-y-1.5 text-ink-primary font-sans text-xs leading-relaxed">
            <div><strong>Nama Instansi:</strong> PT Teknologi Nusantara Mandiri</div>
            <div><strong>Bidang Usaha:</strong> Infrastruktur Jaringan & Pengembangan Perangkat Lunak</div>
            <div><strong>Visi & Misi:</strong> Menyediakan konektivitas stabil dan sistem manajemen terintegrasi.</div>
          </div>
        </div>
      </div>

      <!-- Step 03 -->
      <div class="grid grid-cols-1 md:grid-cols-12 gap-6 items-start">
        <div class="md:col-span-4 space-y-1">
          <span class="text-xs font-mono text-ink-muted">03 / FORMULASI</span>
          <h3 class="text-lg font-bold text-ink-primary">Rapikan jadi naskah akademik utuh</h3>
          <p class="text-xs sm:text-sm text-ink-secondary leading-relaxed">
            Seluruh bab diformulasikan dengan tata bahasa baku PUEBI/EYD, nomor halaman, dan margin 4-4-3-3 standar.
          </p>
        </div>

        <div class="md:col-span-8 p-6 bg-[#0c0f14] border border-border-strong rounded-xl font-serif text-xs text-ink-primary leading-relaxed space-y-3 shadow-doc text-justify">
          <div class="font-sans text-[10px] font-bold text-emerald-400 uppercase tracking-wider font-mono">
            Kutipan Bab III (Pelaksanaan Praktik Kerja)
          </div>
          <p class="indent-6">
            3.2.1 Pemeliharaan Perangkat Keras dan Konfigurasi Jaringan Lokal<br>
            Pada minggu ketiga pelaksanaan praktik kerja lapangan, praktikan melaksanakan instalasi sistem operasi Windows 11 pada 5 (lima) unit komputer kerja serta melakukan konfigurasi alamat IP statis dan integrasi network printer sharing guna menunjang kelancaran operasional divisi keuangan.
          </p>
        </div>
      </div>
    </section>

    <!-- ================================================================= -->
    <!-- ACADEMIC SPECIFICATION: PURE EDITORIAL PRECISION                  -->
    <!-- ================================================================= -->
    <section class="max-w-5xl mx-auto px-4 sm:px-6">
      <div class="p-8 sm:p-10 bg-[#121620] border border-border rounded-2xl space-y-6 shadow-subtle">
        <div class="space-y-1">
          <span class="text-xs font-mono text-emerald-400 font-bold uppercase">STANDAR PENULISAN</span>
          <h3 class="text-xl sm:text-2xl font-serif font-normal text-ink-primary">
            Spesifikasi Dokumen Laporan PKL
          </h3>
        </div>

        <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-4 gap-4 text-xs">
          <div class="space-y-1 p-3.5 bg-surface rounded-lg border border-border/60">
            <div class="font-mono text-ink-muted">Margin Halaman</div>
            <div class="font-bold text-ink-primary text-sm font-mono">4-4-3-3 cm</div>
            <div class="text-ink-secondary text-[11px]">Kiri 4cm (ruang jilid), Atas 4cm, Kanan 3cm, Bawah 3cm.</div>
          </div>

          <div class="space-y-1 p-3.5 bg-surface rounded-lg border border-border/60">
            <div class="font-mono text-ink-muted">Tipografi</div>
            <div class="font-bold text-ink-primary text-sm font-serif">Times New Roman</div>
            <div class="text-ink-secondary text-[11px]">Ukuran 12pt, spasi 1.5, dan indentasi paragraf baku.</div>
          </div>

          <div class="space-y-1 p-3.5 bg-surface rounded-lg border border-border/60">
            <div class="font-mono text-ink-muted">Kelengkapan Bab</div>
            <div class="font-bold text-ink-primary text-sm font-mono">Bab I s.d. Bab IV</div>
            <div class="text-ink-secondary text-[11px]">Cover, Lembar Pengesahan, Bab I-IV, dan Lampiran.</div>
          </div>

          <div class="space-y-1 p-3.5 bg-surface rounded-lg border border-border/60">
            <div class="font-mono text-ink-muted">Format Ekspor</div>
            <div class="font-bold text-ink-primary text-sm font-mono">Word (.docx) & PDF</div>
            <div class="text-ink-secondary text-[11px]">Dapat diedit bebas di Microsoft Word atau Google Docs.</div>
          </div>
        </div>
      </div>
    </section>

    <!-- ================================================================= -->
    <!-- PRICING & DIRECT ACCESS: TRANSPARENT & QUIET                     -->
    <!-- ================================================================= -->
    <section class="max-w-5xl mx-auto px-4 sm:px-6">
      <div class="p-8 sm:p-12 bg-gradient-to-r from-[#141924] via-[#10141d] to-[#0c0e14] border border-border-strong rounded-2xl flex flex-col md:flex-row md:items-center justify-between gap-8 shadow-doc">
        <div class="space-y-3 max-w-lg">
          <span class="text-xs font-mono text-emerald-400 font-bold uppercase">AKSES DOKUMEN</span>
          <h3 class="text-2xl sm:text-3xl font-serif font-normal text-ink-primary leading-snug">
            Periksa seluruh draf secara gratis. Buka kunci dokumen saat siap cetak.
          </h3>
          <p class="text-xs sm:text-sm text-ink-secondary leading-relaxed font-sans">
            Kamu dapat meninjau seluruh isi bab laporan terlebih dahulu. Pembayaran Rp15.000 hanya dilakukan saat mengunduh berkas Microsoft Word asli tanpa watermark.
          </p>
        </div>

        <div class="p-6 bg-[#0f131a] rounded-xl border border-border text-center sm:text-right shrink-0 w-full md:w-72 space-y-4">
          <div>
            <span class="text-[11px] font-mono text-ink-muted block uppercase">Biaya Unduh</span>
            <div class="text-3xl font-black font-mono text-ink-primary my-1">
              Rp15.000
            </div>
            <span class="text-xs text-emerald-400 font-sans">Sekali bayar · Tanpa langganan</span>
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
            <div>Bantuan / Transfer Manual:</div>
            <div class="flex items-center justify-center gap-2 font-mono">
              <a href="https://wa.me/6285117206413" target="_blank" rel="noopener noreferrer" class="text-emerald-400 hover:underline">0851-1720-6413</a>
              <span>·</span>
              <a href="https://wa.me/6288809028653" target="_blank" rel="noopener noreferrer" class="text-emerald-400 hover:underline">0888-0902-8653</a>
            </div>
          </div>
        </div>
      </div>
    </section>
  </div>
</template>

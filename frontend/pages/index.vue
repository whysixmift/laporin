<script setup lang="ts">
const { isAuthenticated } = useAuth()
</script>

<template>
  <div class="space-y-32 sm:space-y-48 pb-32 text-ink-primary">
    <!-- ================================================================= -->
    <!-- HERO: DIRECT, RESTRAINED, CONFIDENT                               -->
    <!-- ================================================================= -->
    <section class="pt-16 sm:pt-28 max-w-5xl mx-auto px-4 sm:px-6">
      <div class="max-w-2xl space-y-8">
        <div class="space-y-4">
          <h1 class="text-4xl sm:text-6xl font-serif font-normal tracking-tight text-ink-primary leading-[1.08]">
            Laporin<span class="text-accent-500 font-sans">.</span>
          </h1>

          <p class="text-lg sm:text-xl text-ink-secondary leading-relaxed font-sans font-normal max-w-xl">
            Alat bantu menyusun laporan Praktik Kerja Lapangan (PKL) dan magang SMK/vokasi.
          </p>
        </div>

        <div class="pt-2 flex flex-col sm:flex-row items-stretch sm:items-center gap-4">
          <NuxtLink
            v-if="isAuthenticated"
            to="/reports/new"
            class="inline-flex items-center justify-center px-6 py-3 rounded bg-ink-primary text-canvas hover:bg-white font-medium text-sm transition-all shadow-subtle text-center"
          >
            Buat Laporan
          </NuxtLink>

          <NuxtLink
            v-else
            to="/register"
            class="inline-flex items-center justify-center px-6 py-3 rounded bg-ink-primary text-canvas hover:bg-white font-medium text-sm transition-all shadow-subtle text-center"
          >
            Buat Laporan
          </NuxtLink>

          <NuxtLink
            to="#alur-kerja"
            class="inline-flex items-center justify-center px-4 py-3 text-xs font-mono text-ink-muted hover:text-ink-primary transition-colors text-center"
          >
            Lihat cara kerja ↓
          </NuxtLink>
        </div>

        <div class="pt-6 border-t border-border/40 flex items-center gap-4 text-xs font-mono text-ink-muted">
          <span>Rp15.000 / naskah Word (.docx)</span>
          <span class="text-ink-faint">·</span>
          <span>Pratinjau draf gratis</span>
        </div>
      </div>
    </section>

    <!-- ================================================================= -->
    <!-- PRODUCT STORY: REAL TRANSFORMATION (NO FAKE SAAS CARDS)          -->
    <!-- ================================================================= -->
    <section id="alur-kerja" class="max-w-5xl mx-auto px-4 sm:px-6 space-y-20">
      <div class="space-y-2 border-b border-border/40 pb-4">
        <span class="text-[11px] font-mono text-ink-muted uppercase tracking-wider">Transformasi Dokumen</span>
        <h2 class="text-2xl sm:text-3xl font-serif font-normal text-ink-primary">
          Dari catatan mentah menjadi naskah baku
        </h2>
      </div>

      <!-- Step 01: Raw Student Notes -->
      <div class="grid grid-cols-1 md:grid-cols-12 gap-8 items-start">
        <div class="md:col-span-4 space-y-2">
          <span class="text-xs font-mono text-ink-muted">01</span>
          <h3 class="text-base font-semibold text-ink-primary">Catatan harian siswa</h3>
          <p class="text-xs sm:text-sm text-ink-secondary leading-relaxed">
            Cukup tuliskan nama tempat magang dan ringkasan apa saja yang kamu kerjakan dalam bahasa sehari-hari.
          </p>
        </div>

        <div class="md:col-span-8 p-5 bg-surface rounded border border-border/60 font-mono text-xs text-ink-secondary space-y-3">
          <div class="flex items-center justify-between text-[11px] text-ink-muted border-b border-border/40 pb-2">
            <span>Input Praktikan:</span>
            <span>Bahasa Santai</span>
          </div>
          <p class="text-ink-primary leading-relaxed">
            "Minggu ke-3: Gw disuruh instalasi ulang 5 PC kantor pake OS Windows 11, abis itu konfigurasi IP address lokal sama setting sharing printer biar divisi accounting bisa ngeprint bareng."
          </p>
        </div>
      </div>

      <!-- Step 02: Research Extraction -->
      <div class="grid grid-cols-1 md:grid-cols-12 gap-8 items-start">
        <div class="md:col-span-4 space-y-2">
          <span class="text-xs font-mono text-ink-muted">02</span>
          <h3 class="text-base font-semibold text-ink-primary">Riset profil instansi</h3>
          <p class="text-xs sm:text-sm text-ink-secondary leading-relaxed">
            Laporin menelusuri data resmi instansi untuk melengkapi Bab II: sejarah perusahaan, visi misi, dan struktur kerja.
          </p>
        </div>

        <div class="md:col-span-8 p-5 bg-surface rounded border border-border/60 text-xs space-y-3">
          <div class="flex items-center justify-between text-[11px] font-mono text-ink-muted border-b border-border/40 pb-2">
            <span>Ekstraksi Data Perusahaan:</span>
            <span class="text-accent-400">Terverifikasi</span>
          </div>
          <div class="space-y-1.5 text-ink-secondary leading-relaxed">
            <div><span class="text-ink-muted font-mono">Nama:</span> <strong class="text-ink-primary">PT Teknologi Nusantara Mandiri</strong></div>
            <div><span class="text-ink-muted font-mono">Bidang:</span> Infrastruktur Jaringan & Rekayasa Perangkat Lunak</div>
            <div><span class="text-ink-muted font-mono">Visi:</span> Mengembangkan konektivitas digital terintegrasi untuk industri nasional.</div>
          </div>
        </div>
      </div>

      <!-- Step 03: Final Academic Formulation -->
      <div class="grid grid-cols-1 md:grid-cols-12 gap-8 items-start">
        <div class="md:col-span-4 space-y-2">
          <span class="text-xs font-mono text-ink-muted">03</span>
          <h3 class="text-base font-semibold text-ink-primary">Naskah laporan utuh</h3>
          <p class="text-xs sm:text-sm text-ink-secondary leading-relaxed">
            Seluruh bab diformulasikan dengan tata bahasa baku PUEBI/EYD, nomor halaman, dan margin 4-4-3-3 standar.
          </p>
        </div>

        <div class="md:col-span-8 p-6 bg-[#0f1115] rounded border border-border/80 font-serif text-xs text-ink-primary leading-relaxed space-y-3 text-justify shadow-subtle">
          <div class="font-sans text-[10px] font-mono text-accent-400 uppercase tracking-wider">
            Kutipan Bab III — Pelaksanaan Praktik Kerja
          </div>
          <p class="indent-6">
            3.2.1 Pemeliharaan Perangkat Keras dan Konfigurasi Jaringan Lokal<br>
            Pada minggu ketiga pelaksanaan praktik kerja lapangan, praktikan melaksanakan instalasi sistem operasi Windows 11 pada 5 (lima) unit komputer kerja serta melakukan konfigurasi alamat IP statis dan integrasi network printer sharing guna menunjang kelancaran operasional divisi keuangan.
          </p>
        </div>
      </div>
    </section>

    <!-- ================================================================= -->
    <!-- SPECIFICATIONS: EDITORIAL RESTRAINT                               -->
    <!-- ================================================================= -->
    <section class="max-w-5xl mx-auto px-4 sm:px-6">
      <div class="border-t border-border/40 pt-12 space-y-8">
        <div class="space-y-1">
          <span class="text-[11px] font-mono text-ink-muted uppercase tracking-wider">Standar Dokumen</span>
          <h3 class="text-xl sm:text-2xl font-serif font-normal text-ink-primary">
            Spesifikasi format laporan PKL
          </h3>
        </div>

        <div class="grid grid-cols-2 md:grid-cols-4 gap-6 text-xs">
          <div class="space-y-1">
            <div class="font-mono text-ink-muted text-[11px]">Margin Halaman</div>
            <div class="text-ink-primary font-medium">4-4-3-3 cm</div>
            <div class="text-ink-muted text-[11px] leading-relaxed">Kiri 4cm (jilid), Atas 4cm, Kanan 3cm, Bawah 3cm.</div>
          </div>

          <div class="space-y-1">
            <div class="font-mono text-ink-muted text-[11px]">Tipografi</div>
            <div class="text-ink-primary font-medium font-serif">Times New Roman</div>
            <div class="text-ink-muted text-[11px] leading-relaxed">Ukuran 12pt, spasi 1.5, dan perataan teks justified.</div>
          </div>

          <div class="space-y-1">
            <div class="font-mono text-ink-muted text-[11px]">Kelengkapan Bab</div>
            <div class="text-ink-primary font-medium">Bab I s.d. Bab IV</div>
            <div class="text-ink-muted text-[11px] leading-relaxed">Cover, lembar pengesahan, pendahuluan, profil, isi, & saran.</div>
          </div>

          <div class="space-y-1">
            <div class="font-mono text-ink-muted text-[11px]">Format Berkas</div>
            <div class="text-ink-primary font-medium">Word (.docx)</div>
            <div class="text-ink-muted text-[11px] leading-relaxed">Dapat diedit bebas di Microsoft Word dan Google Docs.</div>
          </div>
        </div>
      </div>
    </section>

    <!-- ================================================================= -->
    <!-- PRICING & CALL TO ACTION: HONEST & QUIET                          -->
    <!-- ================================================================= -->
    <section class="max-w-5xl mx-auto px-4 sm:px-6">
      <div class="p-8 sm:p-12 bg-surface rounded border border-border/60 flex flex-col md:flex-row md:items-center justify-between gap-8">
        <div class="space-y-3 max-w-lg">
          <span class="text-[11px] font-mono text-ink-muted uppercase tracking-wider">Akses Dokumen</span>
          <h3 class="text-2xl sm:text-3xl font-serif font-normal text-ink-primary leading-snug">
            Periksa draf secara gratis. Unduh saat siap cetak.
          </h3>
          <p class="text-xs sm:text-sm text-ink-secondary leading-relaxed font-sans">
            Kamu dapat meninjau seluruh isi bab laporan terlebih dahulu. Pembayaran hanya dilakukan saat mengunduh berkas Microsoft Word asli tanpa watermark.
          </p>
        </div>

        <div class="shrink-0 space-y-4 md:text-right">
          <div>
            <span class="text-[11px] font-mono text-ink-muted block uppercase">Biaya Unduh</span>
            <div class="text-3xl font-black font-mono text-ink-primary my-1">
              Rp15.000
            </div>
            <span class="text-xs text-ink-muted">Sekali bayar · Tanpa langganan</span>
          </div>

          <div>
            <NuxtLink
              v-if="isAuthenticated"
              to="/reports/new"
              class="inline-flex items-center justify-center px-6 py-2.5 rounded bg-ink-primary text-canvas hover:bg-white font-medium text-xs transition-all text-center w-full md:w-auto"
            >
              Mulai Susun Laporan
            </NuxtLink>
            <NuxtLink
              v-else
              to="/register"
              class="inline-flex items-center justify-center px-6 py-2.5 rounded bg-ink-primary text-canvas hover:bg-white font-medium text-xs transition-all text-center w-full md:w-auto"
            >
              Mulai Susun Laporan
            </NuxtLink>
          </div>
        </div>
      </div>
    </section>
  </div>
</template>


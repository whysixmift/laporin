<script setup lang="ts">
const { isAuthenticated } = useAuth()
</script>

<template>
  <div class="space-y-16 sm:space-y-24 py-10 sm:py-16">
    <!-- HERO SECTION: Narrative of Document Transformation -->
    <section class="max-w-5xl mx-auto px-4 sm:px-6">
      <div class="grid grid-cols-1 lg:grid-cols-12 gap-10 lg:gap-12 items-center">
        <!-- Left: Clear Value Statement -->
        <div class="lg:col-span-7 space-y-6">
          <div class="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-surface-elevated border border-border text-xs text-ink-secondary">
            <span class="w-2 h-2 rounded-full bg-accent-400" />
            <span class="font-mono text-[11px]">Format Standar Pendidikan Indonesia (SMK / Vokasi / S1)</span>
          </div>

          <h1 class="text-3xl sm:text-4xl lg:text-5xl font-bold tracking-tight text-ink-primary leading-[1.15]">
            Laporan PKL kamu, <br />
            <span class="font-serif italic font-normal text-accent-300">beres dan terstruktur.</span>
          </h1>

          <p class="text-base sm:text-lg text-ink-secondary leading-relaxed max-w-xl">
            Cukup masukkan catatan kegiatan magang kamu. Laporin meneliti profil perusahaan, menyusun Bab I hingga Bab IV, dan menyiapkan dokumen Word (.docx) siap cetak.
          </p>

          <!-- Primary Actions -->
          <div class="flex flex-col sm:flex-row items-stretch sm:items-center gap-3 pt-2">
            <BaseButton
              v-if="isAuthenticated"
              to="/reports/new"
              size="lg"
              variant="primary"
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
            >
              Mulai Susun Laporan
            </BaseButton>

            <BaseButton
              to="#cara-kerja"
              size="lg"
              variant="subtle"
            >
              Lihat Alur Kerja
            </BaseButton>
          </div>

          <div class="flex items-center gap-6 pt-4 text-xs text-ink-muted border-t border-border-subtle">
            <div class="flex items-center gap-1.5">
              <span class="font-bold text-ink-secondary font-mono">Rp15.000</span>
              <span>/ file DOCX</span>
            </div>
            <span>·</span>
            <div>Tanpa langganan bulanan</div>
            <span>·</span>
            <div>Pratinjau gratis sebelum bayar</div>
          </div>
        </div>

        <!-- Right: Realistic Document Preview Card -->
        <div class="lg:col-span-5 relative">
          <div class="w-full bg-surface-elevated border border-border-strong rounded-xl p-6 shadow-doc relative overflow-hidden group">
            <div class="flex items-center justify-between pb-3 border-b border-border-subtle text-xs text-ink-muted">
              <span class="font-mono text-[11px] text-accent-400">preview_laporan.docx</span>
              <span class="font-mono text-[10px] bg-surface px-2 py-0.5 rounded border border-border">A4 Formatted</span>
            </div>

            <div class="mt-4 space-y-4 font-serif text-xs leading-relaxed text-ink-secondary">
              <!-- Mini Page Mock -->
              <div class="p-4 bg-canvas/60 rounded border border-border-subtle/60 space-y-3">
                <div class="text-center space-y-1">
                  <p class="font-bold font-serif text-ink-primary text-sm tracking-wide uppercase">
                    LAPORAN PRAKTIK KERJA LAPANGAN
                  </p>
                  <p class="text-[10px] text-ink-muted font-sans uppercase">
                    PADA PT TEKNOLOGI NUSANTARA
                  </p>
                </div>

                <div class="text-[11px] text-justify space-y-2 pt-2 border-t border-border-subtle/40">
                  <p class="text-ink-primary font-semibold font-sans text-[10px] uppercase text-accent-400">
                    BAB I · PENDAHULUAN
                  </p>
                  <p>
                    Praktik Kerja Lapangan (PKL) diselenggarakan guna membekali peserta didik dengan kompetensi teknis dan budaya kerja profesional...
                  </p>
                </div>
              </div>

              <!-- Fact annotation pill -->
              <div class="p-2.5 bg-surface rounded border border-border flex items-center justify-between text-[11px]">
                <div class="flex items-center gap-2">
                  <span class="w-2 h-2 rounded-full bg-emerald-400" />
                  <span class="text-ink-primary font-sans font-medium">5 Fakta Perusahaan Terverifikasi</span>
                </div>
                <span class="text-ink-muted font-mono text-[10px]">9Router Checked</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- WORKFLOW BREAKDOWN: Real Process, No AI Fluff -->
    <section id="cara-kerja" class="max-w-5xl mx-auto px-4 sm:px-6">
      <div class="space-y-4 mb-10">
        <h2 class="text-xs uppercase font-mono tracking-widest text-accent-400">Alur Penyusunan</h2>
        <h3 class="text-2xl sm:text-3xl font-bold text-ink-primary">
          Dari catatan kegiatan kamu menjadi laporan rapi.
        </h3>
      </div>

      <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
        <!-- Step 1 -->
        <div class="p-6 bg-surface rounded-xl border border-border flex flex-col justify-between space-y-4">
          <div class="space-y-3">
            <span class="w-7 h-7 rounded bg-surface-elevated border border-border-strong flex items-center justify-center font-mono text-xs font-bold text-accent-400">
              01
            </span>
            <h4 class="font-semibold text-base text-ink-primary">1. Masukkan Data PKL</h4>
            <p class="text-sm text-ink-secondary leading-relaxed">
              Tulis nama siswa, sekolah, perusahaan tempat magang, dan ringkasan aktivitas nyata yang kamu kerjakan selama masa PKL.
            </p>
          </div>
          <div class="text-xs text-ink-muted font-mono pt-2 border-t border-border-subtle">
            Input: Pengalaman asli kamu
          </div>
        </div>

        <!-- Step 2 -->
        <div class="p-6 bg-surface rounded-xl border border-border flex flex-col justify-between space-y-4">
          <div class="space-y-3">
            <span class="w-7 h-7 rounded bg-surface-elevated border border-border-strong flex items-center justify-center font-mono text-xs font-bold text-accent-400">
              02
            </span>
            <h4 class="font-semibold text-base text-ink-primary">2. Riset & Penyusunan Bab</h4>
            <p class="text-sm text-ink-secondary leading-relaxed">
              Laporin menelusuri data resmi perusahaan (sejarah, profil, visi & misi) dan menyusun draft dokumen Bab I s.d. Bab IV secara sistematis.
            </p>
          </div>
          <div class="text-xs text-ink-muted font-mono pt-2 border-t border-border-subtle">
            Proses: Riset web + Formulasi bab
          </div>
        </div>

        <!-- Step 3 -->
        <div class="p-6 bg-surface rounded-xl border border-border flex flex-col justify-between space-y-4">
          <div class="space-y-3">
            <span class="w-7 h-7 rounded bg-surface-elevated border border-border-strong flex items-center justify-center font-mono text-xs font-bold text-accent-400">
              03
            </span>
            <h4 class="font-semibold text-base text-ink-primary">3. Pratinjau & Unduh DOCX</h4>
            <p class="text-sm text-ink-secondary leading-relaxed">
              Baca seluruh isi dokumen sebelum bayar. Jika sudah sesuai, buka kunci file Microsoft Word asli seharga Rp15.000 untuk pengeditan akhir.
            </p>
          </div>
          <div class="text-xs text-ink-muted font-mono pt-2 border-t border-border-subtle">
            Output: File .docx siap pakai
          </div>
        </div>
      </div>
    </section>

    <!-- TRANSPARENT PRICING & ETHICAL PROMISE -->
    <section class="max-w-5xl mx-auto px-4 sm:px-6">
      <div class="p-8 sm:p-10 bg-surface-elevated rounded-xl border border-border flex flex-col md:flex-row items-start md:items-center justify-between gap-8">
        <div class="space-y-3 max-w-xl">
          <span class="text-xs font-mono uppercase tracking-wider text-accent-400">Harga Transparan</span>
          <h3 class="text-2xl font-bold text-ink-primary">
            Satu harga per laporan. Tanpa jebakan langganan.
          </h3>
          <p class="text-sm text-ink-secondary leading-relaxed">
            Tidak ada perpanjangan otomatis atau biaya tersembunyi. Kamu hanya membayar ketika laporan sudah selesai diriset, disusun, dan diperiksa lewat pratinjau.
          </p>
        </div>

        <div class="p-6 bg-canvas rounded-lg border border-border-strong text-center sm:text-right shrink-0 w-full md:w-auto">
          <span class="text-xs text-ink-muted block">Total Biaya Unduh</span>
          <div class="text-3xl font-black font-mono text-ink-primary my-1">
            Rp15.000
          </div>
          <span class="text-xs text-accent-300 block mb-4">Pembayaran via QRIS & VA Bank</span>

          <BaseButton
            v-if="isAuthenticated"
            to="/reports/new"
            variant="primary"
            class="w-full"
          >
            Mulai Buat Laporan
          </BaseButton>
          <BaseButton
            v-else
            to="/register"
            variant="primary"
            class="w-full"
          >
            Daftar Akun Sekarang
          </BaseButton>
        </div>
      </div>
    </section>
  </div>
</template>

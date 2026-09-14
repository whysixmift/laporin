<script setup lang="ts">
const currentStage = ref(1)

const stages = [
  {
    stage: 1,
    title: 'Catatan Harian Kasar',
    subtitle: 'Input Bahasa Santai / Poin WhatsApp',
    badge: '01 · RAW INPUT',
    desc: 'Kamu cukup memasukkan poin aktivitas harian selama PKL dalam bahasa sehari-hari tanpa perlu memikirkan format akademis yang kaku.'
  },
  {
    stage: 2,
    title: 'Penelusuran Data & Riset DU/DI',
    subtitle: 'Verifikasi Profil & Nilai Perusahaan',
    badge: '02 · FACT EXTRACTION',
    desc: 'Sistem menelusuri data resmi tempat magang (sejarah, profil, visi & misi, divisi, dan standar keselamatan kerja) sebagai landasan Bab II.'
  },
  {
    stage: 3,
    title: 'Formulasi Bab Akademik',
    subtitle: 'Penyusunan Bab I s.d. Bab IV Baku',
    badge: '03 · STRUCTURING',
    desc: 'Seluruh catatan dan riset dirangkai menjadi bab-bab terstruktur dengan kaidah bahasa baku PUEBI/EYD, nomor halaman, dan rincian sub-bab.'
  },
  {
    stage: 4,
    title: 'Dokumen Word (.docx) & PDF Siap Jilid',
    subtitle: 'Margin Standar 4-4-3-3 Siap Cetak',
    badge: '04 · PHYSICAL EXPORT',
    desc: 'Dokumen utuh Microsoft Word terkompilasi siap diunduh, diedit kembali, dan dicetak langsung untuk sidang PKL.'
  }
]
</script>

<template>
  <div class="w-full py-12 space-y-12">
    <!-- Header -->
    <div class="text-center space-y-3 max-w-3xl mx-auto">
      <div class="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-emerald-500/10 border border-emerald-500/30 text-xs text-emerald-400 font-mono">
        ALUR TRANSFORMASI DOKUMEN
      </div>
      <h2 class="text-3xl sm:text-4xl font-bold text-ink-primary tracking-tight font-sans">
        Dari Informasi Mentah Menjadi Dokumen Utuh
      </h2>
      <p class="text-sm sm:text-base text-ink-secondary leading-relaxed">
        Lihat bagaimana catatan harian sederhana kamu diolah secara bertahap menjadi laporan PKL berstandar akademik.
      </p>
    </div>

    <!-- Interactive Stage Navigation Tabs -->
    <div class="flex items-center justify-center gap-2 max-w-4xl mx-auto overflow-x-auto pb-2 px-4">
      <button
        v-for="s in stages"
        :key="s.stage"
        type="button"
        class="px-4 py-2.5 rounded-xl border text-xs font-mono font-bold transition-all shrink-0 cursor-pointer flex items-center gap-2"
        :class="currentStage === s.stage
          ? 'bg-emerald-500 text-white border-emerald-400 shadow-elevated scale-105'
          : 'bg-surface hover:bg-surface-elevated text-ink-secondary border-border'"
        @click="currentStage = s.stage"
      >
        <span class="w-5 h-5 rounded-full bg-black/20 flex items-center justify-center text-[10px]">
          {{ s.stage }}
        </span>
        <span>{{ s.badge }}</span>
      </button>
    </div>

    <!-- 3D Dimensional Stage Transformation Stage -->
    <div class="max-w-5xl mx-auto px-4">
      <div class="grid grid-cols-1 lg:grid-cols-12 gap-8 items-center bg-[#111620] border border-border-strong rounded-2xl p-6 sm:p-10 shadow-doc relative overflow-hidden">
        <!-- Left: Stage Description -->
        <div class="lg:col-span-5 space-y-4">
          <div class="font-mono text-xs font-bold text-emerald-400 uppercase">
            {{ stages[currentStage - 1].badge }}
          </div>
          <h3 class="text-2xl font-bold text-ink-primary font-sans leading-snug">
            {{ stages[currentStage - 1].title }}
          </h3>
          <div class="text-xs font-mono text-ink-muted">
            {{ stages[currentStage - 1].subtitle }}
          </div>
          <p class="text-sm text-ink-secondary leading-relaxed font-serif">
            {{ stages[currentStage - 1].desc }}
          </p>

          <div class="pt-4 flex items-center gap-3 text-xs">
            <button
              type="button"
              class="px-3 py-1.5 rounded-lg bg-surface border border-border text-ink-secondary hover:text-ink-primary disabled:opacity-40 cursor-pointer"
              :disabled="currentStage === 1"
              @click="currentStage--"
            >
              ← Tahap Sebelumnya
            </button>
            <button
              type="button"
              class="px-3 py-1.5 rounded-lg bg-surface border border-border text-ink-secondary hover:text-ink-primary disabled:opacity-40 cursor-pointer"
              :disabled="currentStage === 4"
              @click="currentStage++"
            >
              Tahap Selanjutnya →
            </button>
          </div>
        </div>

        <!-- Right: Visual Transformation Mock in 3D Perspective -->
        <div class="lg:col-span-7 flex items-center justify-center p-4">
          <!-- STAGE 1: Raw Sticky Notes / Chat -->
          <div v-if="currentStage === 1" class="w-full space-y-3 font-mono text-xs animate-fadeIn">
            <div class="p-4 rounded-xl bg-amber-500/10 border border-amber-500/30 text-amber-200 transform -rotate-1 shadow-elevated">
              <div class="text-[10px] text-amber-400 font-bold mb-1">📝 CATATAN WHATSAPP · 12 AGUSTUS</div>
              "Hari ini bantuin mas Dimas konfigurasi switch cisco di ruang server lantai 2, trus rapiin kabel LAN patch cord ke patch panel."
            </div>
            <div class="p-4 rounded-xl bg-sky-500/10 border border-sky-500/30 text-sky-200 transform rotate-1 shadow-elevated">
              <div class="text-[10px] text-sky-400 font-bold mb-1">📝 CATATAN WHATSAPP · 15 AGUSTUS</div>
              "Monitoring trafik jaringan pake Wireshark sama catet log packet loss di router gateway."
            </div>
          </div>

          <!-- STAGE 2: Fact Extraction -->
          <div v-else-if="currentStage === 2" class="w-full space-y-3 font-mono text-xs animate-fadeIn">
            <div class="p-4 rounded-xl bg-surface border border-emerald-500/50 space-y-2 shadow-elevated">
              <div class="flex items-center justify-between text-[11px] text-emerald-400 font-bold">
                <span>🌐 PROFIL PERUSAHAAN TERVERIFIKASI</span>
                <span class="px-2 py-0.5 rounded bg-emerald-500/20 text-[10px]">9ROUTER ENGINE</span>
              </div>
              <div class="text-ink-primary font-sans text-xs">
                PT Teknologi Nusantara Mandiri — Didirikan tahun 2018 sebagai penyedia infrastruktur jaringan telekomunikasi dan sistem otomasi cloud.
              </div>
              <div class="pt-2 border-t border-border flex items-center gap-2 text-[10px] text-ink-muted">
                <span>Visi Misi: Terlampir</span>
                <span>·</span>
                <span>Struktur Organisasi: 4 Divisi</span>
              </div>
            </div>
          </div>

          <!-- STAGE 3: Formal Academic Drafting -->
          <div v-else-if="currentStage === 3" class="w-full font-serif text-xs leading-relaxed animate-fadeIn">
            <div class="p-6 rounded-xl bg-[#0f1217] border border-border space-y-3 shadow-doc text-justify">
              <div class="font-sans font-bold text-[11px] text-emerald-400 uppercase tracking-wider">
                BAB III · PELAKSANAAN PRAKTIK KERJA LAPANGAN
              </div>
              <p class="text-ink-primary text-xs indent-6">
                3.2.1 Manajemen Infrastruktur Jaringan Lokal dan Penataan Kabel Terstruktur<br>
                Praktikan melakukan instalasi dan pengujian perangkat switch jaringan pada sentra server serta melakukan standarisasi patch cord menuju patch panel guna memastikan stabilitas throughput transmisi data antar divisi.
              </p>
            </div>
          </div>

          <!-- STAGE 4: Final Word (.docx) & PDF Ready -->
          <div v-else class="w-full font-sans text-xs animate-fadeIn">
            <div class="p-6 rounded-xl bg-[#131924] border-2 border-emerald-500 shadow-doc text-center space-y-4">
              <div class="w-12 h-12 rounded-xl bg-emerald-500/20 border border-emerald-500/40 flex items-center justify-center text-emerald-400 mx-auto">
                <svg class="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
                </svg>
              </div>
              <div>
                <div class="text-base font-bold text-ink-primary">Laporan PKL Siap Jilid (.docx)</div>
                <div class="text-xs text-ink-muted font-mono mt-1">18 Halaman · Margin 4-4-3-3 · Bab I s.d. Bab IV</div>
              </div>
              <div class="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-emerald-500/10 text-emerald-400 font-mono text-[11px] font-bold">
                100% Siap Cetak & Sidang
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
@keyframes fadeIn {
  from { opacity: 0; transform: translateY(8px); }
  to { opacity: 1; transform: translateY(0); }
}
.animate-fadeIn {
  animation: fadeIn 0.35s cubic-bezier(0.16, 1, 0.3, 1) forwards;
}
</style>

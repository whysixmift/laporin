<script setup lang="ts">
const activeIndex = ref(0)
const hoveredIndex = ref<number | null>(null)

const chapters = [
  {
    code: 'COVER',
    num: '00',
    title: 'Cover & Lembar Pengesahan',
    desc: 'Halaman judul resmi, logo instansi, nama siswa/NISN, data pembimbing sekolah, dan tanda tangan pimpinan DU/DI.',
    details: ['Kop Surat dan Judul Baku', 'Format Pengesahan DU/DI', 'Identitas Lengkap Praktikan'],
    color: 'emerald'
  },
  {
    code: 'BAB I',
    num: '01',
    title: 'Pendahuluan & Tujuan Praktik',
    desc: 'Landasan yuridis PKL, latar belakang kompetensi keahlian, maksud dan tujuan umum/khusus, serta jadwal pelaksanaan.',
    details: ['Latar Belakang Vokasi', 'Tujuan Teknis & Softskill', 'Waktu & Tempat Pelaksanaan'],
    color: 'emerald'
  },
  {
    code: 'BAB II',
    num: '02',
    title: 'Gambaran Umum DU/DI',
    desc: 'Profil lengkap perusahaan tempat magang, sejarah pendirian, visi misi, tata nilai kerja, dan struktur organisasi divisi.',
    details: ['Sejarah Singkat DU/DI', 'Visi, Misi & Budaya Kerja', 'Struktur Organisasi Divisi'],
    color: 'emerald'
  },
  {
    code: 'BAB III',
    num: '03',
    title: 'Pelaksanaan & Pembahasan Teknis',
    desc: 'Uraian sistematis jurnal kegiatan harian, tugas pemeliharaan/pengembangan sistem, kendala teknis, dan solusi pemecahan.',
    details: ['Jurnal Aktivitas Berkala', 'Analisis Masalah Kerja', 'Solusi & Implementasi Teknis'],
    color: 'emerald'
  },
  {
    code: 'BAB IV',
    num: '04',
    title: 'Penutup, Saran & Lampiran',
    desc: 'Kesimpulan hasil praktik kerja, saran konstruktif untuk kurikulum sekolah dan DU/DI, serta dokumentasi foto kegiatan.',
    details: ['Kesimpulan Evaluasi PKL', 'Saran untuk Sekolah & DU/DI', 'Daftar Pustaka & Foto Kegiatan'],
    color: 'emerald'
  }
]
</script>

<template>
  <div class="w-full py-12">
    <div class="grid grid-cols-1 lg:grid-cols-12 gap-8 lg:gap-12 items-center">
      <!-- Left: Interactive 3D Perspective Document Staging -->
      <div class="lg:col-span-6 flex items-center justify-center p-6 sm:p-10 perspective-1000 select-none">
        <div class="relative w-full max-w-[340px] sm:max-w-[380px] h-[460px] sm:h-[500px] transform-style-3d">
          <div
            v-for="(chap, idx) in chapters"
            :key="idx"
            class="absolute inset-0 rounded-xl p-6 transition-all duration-500 cursor-pointer shadow-doc border"
            :style="{
              transform: `
                translateY(${idx === activeIndex ? -24 : (idx - activeIndex) * 16}px)
                translateX(${(idx - activeIndex) * 8}px)
                translateZ(${idx === activeIndex ? 60 : -(Math.abs(idx - activeIndex) * 28)}px)
                rotateX(12deg)
                rotateY(-18deg)
                rotateZ(${idx === activeIndex ? -1 : (idx - activeIndex) * 1.5}deg)
              `,
              zIndex: idx === activeIndex ? 30 : 20 - idx,
              backgroundColor: idx === activeIndex ? '#181e29' : idx < activeIndex ? '#0f131a' : '#141821',
              borderColor: idx === activeIndex ? '#10b981' : '#2d3748',
              opacity: idx === activeIndex ? 1 : Math.max(0.4, 1 - Math.abs(idx - activeIndex) * 0.25)
            }"
            @click="activeIndex = idx"
            @mouseenter="hoveredIndex = idx"
            @mouseleave="hoveredIndex = null"
          >
            <!-- Physical Paper Tab Header -->
            <div class="flex items-center justify-between pb-3 border-b border-border/80">
              <span class="font-mono text-xs font-bold px-2 py-0.5 rounded bg-surface border border-border" :class="idx === activeIndex ? 'text-emerald-400 border-emerald-500/40' : 'text-ink-muted'">
                {{ chap.code }}
              </span>
              <span class="font-mono text-[10px] text-ink-muted">A4 · SHEET {{ chap.num }}</span>
            </div>

            <!-- Paper Content Preview -->
            <div class="mt-4 space-y-3">
              <h4 class="font-bold text-sm sm:text-base text-ink-primary font-sans leading-snug">
                {{ chap.title }}
              </h4>
              <p class="text-xs text-ink-secondary line-clamp-3 leading-relaxed font-serif">
                {{ chap.desc }}
              </p>

              <!-- Academic Lines Preview -->
              <div class="pt-4 space-y-2 border-t border-border/50">
                <div class="h-2 bg-surface-elevated rounded w-full" />
                <div class="h-2 bg-surface-elevated rounded w-4/5" />
                <div class="h-2 bg-surface-elevated rounded w-3/5" />
              </div>
            </div>

            <!-- Sheet Footer -->
            <div class="absolute bottom-4 inset-x-6 pt-3 border-t border-border/40 flex items-center justify-between text-[10px] font-mono text-ink-muted">
              <span>LAPORIN DOC-CORE</span>
              <span v-if="idx === activeIndex" class="text-emerald-400 font-bold">AKTIF</span>
              <span v-else>KLIK UNTUK LIHAT</span>
            </div>
          </div>
        </div>
      </div>

      <!-- Right: Detailed Chapter Specifications -->
      <div class="lg:col-span-6 space-y-6">
        <div class="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-emerald-500/10 border border-emerald-500/30 text-xs text-emerald-400 font-mono">
          <span>LAPISAN DOKUMEN 3D</span>
          <span>·</span>
          <span>{{ chapters[activeIndex].code }}</span>
        </div>

        <div class="space-y-2">
          <h3 class="text-2xl sm:text-3xl font-bold text-ink-primary tracking-tight font-sans">
            {{ chapters[activeIndex].title }}
          </h3>
          <p class="text-sm sm:text-base text-ink-secondary leading-relaxed font-normal">
            {{ chapters[activeIndex].desc }}
          </p>
        </div>

        <div class="p-5 rounded-xl bg-surface border border-border space-y-3">
          <div class="text-xs font-mono uppercase font-bold text-ink-muted">
            Komponen & Butir Pembahasan Baku:
          </div>
          <div class="space-y-2">
            <div
              v-for="(detail, dIdx) in chapters[activeIndex].details"
              :key="dIdx"
              class="flex items-center gap-3 p-2.5 rounded-lg bg-surface-elevated border border-border/60 text-xs sm:text-sm text-ink-primary"
            >
              <span class="w-5 h-5 rounded-full bg-emerald-500/10 text-emerald-400 font-mono font-bold flex items-center justify-center text-xs shrink-0">
                {{ dIdx + 1 }}
              </span>
              <span>{{ detail }}</span>
            </div>
          </div>
        </div>

        <!-- Chapter Selector Pills -->
        <div class="flex flex-wrap items-center gap-2 pt-2">
          <button
            v-for="(chap, cIdx) in chapters"
            :key="cIdx"
            type="button"
            class="px-3.5 py-1.5 rounded-lg text-xs font-mono font-bold transition-all border cursor-pointer"
            :class="activeIndex === cIdx
              ? 'bg-emerald-500 text-white border-emerald-400 shadow-subtle'
              : 'bg-surface hover:bg-surface-elevated text-ink-secondary border-border'"
            @click="activeIndex = cIdx"
          >
            {{ chap.code }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.perspective-1000 {
  perspective: 1200px;
}
.transform-style-3d {
  transform-style: preserve-3d;
}
</style>

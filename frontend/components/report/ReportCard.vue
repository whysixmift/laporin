<script setup lang="ts">
import type { Report } from '~/types/api'

interface Props {
  report: Report
}

const props = defineProps<Props>()

const formattedDate = computed(() => {
  if (!props.report.created_at) return ''
  const date = new Date(props.report.created_at)
  return new Intl.DateTimeFormat('id-ID', {
    day: 'numeric',
    month: 'short',
    year: 'numeric'
  }).format(date)
})

const statusMeta = computed(() => {
  switch (props.report.status) {
    case 'draft':
      return { label: 'Mulai Riset Profil', to: `/reports/${props.report.id}`, color: 'text-amber-400', border: 'border-amber-500/30', bg: 'bg-amber-500/10', tag: 'DRAF' }
    case 'researching':
      return { label: 'Pantau Riset', to: `/reports/${props.report.id}`, color: 'text-sky-400', border: 'border-sky-500/30', bg: 'bg-sky-500/10', tag: 'RISET DU/DI' }
    case 'research_completed':
      return { label: 'Susun Dokumen Bab', to: `/reports/${props.report.id}`, color: 'text-emerald-400', border: 'border-emerald-500/30', bg: 'bg-emerald-500/10', tag: 'RISET SELESAI' }
    case 'generating':
      return { label: 'Menyusun DOCX...', to: `/reports/${props.report.id}`, color: 'text-emerald-400', border: 'border-emerald-500/30', bg: 'bg-emerald-500/10', tag: 'GENERATING' }
    case 'preview_ready':
    case 'payment_pending':
      return { label: 'Lihat Pratinjau 3D', to: `/reports/${props.report.id}`, color: 'text-emerald-400', border: 'border-emerald-500/50', bg: 'bg-emerald-500/15', tag: 'PRATINJAU SIAP' }
    case 'unlocked':
    case 'paid':
      return { label: 'Unduh Microsoft Word', to: `/reports/${props.report.id}`, color: 'text-emerald-300', border: 'border-emerald-500', bg: 'bg-emerald-500/20', tag: 'TERBUKA PENUH' }
    case 'failed':
      return { label: 'Coba Ulang', to: `/reports/${props.report.id}`, color: 'text-rose-400', border: 'border-rose-500/30', bg: 'bg-rose-500/10', tag: 'TERKENDALA' }
    default:
      return { label: 'Buka Dokumen', to: `/reports/${props.report.id}`, color: 'text-ink-secondary', border: 'border-border', bg: 'bg-surface', tag: 'DOKUMEN' }
  }
})
</script>

<template>
  <div class="relative group perspective-800">
    <!-- Physical 3D Folio Card -->
    <div
      class="p-6 rounded-2xl bg-[#121620] border border-border-strong group-hover:border-emerald-500/60 shadow-elevated transition-all duration-300 transform group-hover:-translate-y-1.5 group-hover:rotate-x-2 flex flex-col justify-between space-y-6 relative overflow-hidden"
    >
      <!-- Top Paper Seam & Binder Accent -->
      <div class="flex items-center justify-between pb-3 border-b border-border/80 text-xs">
        <div class="flex items-center gap-2">
          <span class="w-2 h-2 rounded-full" :class="report.status === 'unlocked' || report.status === 'preview_ready' ? 'bg-emerald-400 animate-pulse' : 'bg-amber-400'" />
          <span class="font-mono text-[11px] font-bold uppercase tracking-wider" :class="statusMeta.color">
            {{ statusMeta.tag }}
          </span>
        </div>
        <span class="text-[11px] font-mono text-ink-muted">{{ formattedDate }}</span>
      </div>

      <!-- Report Metadata & Excerpt -->
      <div class="space-y-3">
        <h3 class="font-bold text-ink-primary text-base sm:text-lg group-hover:text-emerald-300 transition-colors line-clamp-2 leading-snug font-sans">
          {{ report.title || 'Laporan Praktik Kerja Lapangan' }}
        </h3>

        <div class="p-3.5 rounded-xl bg-[#0b0e14] border border-border-subtle/80 space-y-1.5 font-mono text-xs">
          <div class="flex items-center justify-between text-ink-primary font-semibold">
            <span class="truncate">{{ report.internship?.company_name || 'Instansi DU/DI' }}</span>
            <span class="text-[10px] text-ink-muted shrink-0">A4 · DOCX</span>
          </div>
          <div class="text-[11px] text-ink-muted truncate">
            Praktikan: {{ report.student?.full_name || 'Siswa' }} ({{ report.student?.school || 'Sekolah' }})
          </div>
        </div>
      </div>

      <!-- Footer Action Button -->
      <div class="pt-3 border-t border-border/80 flex items-center justify-between">
        <span class="text-[10px] font-mono text-ink-muted">
          REF: {{ report.id.slice(0, 8).toUpperCase() }}
        </span>

        <NuxtLink
          :to="statusMeta.to"
          class="px-4 py-2 text-xs font-semibold rounded-lg transition-all flex items-center gap-1.5 cursor-pointer"
          :class="report.status === 'unlocked' || report.status === 'preview_ready'
            ? 'bg-emerald-500 hover:bg-emerald-400 text-white shadow-subtle'
            : 'bg-surface-elevated hover:bg-surface-hover text-ink-primary border border-border'"
        >
          <span>{{ statusMeta.label }}</span>
          <svg class="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
          </svg>
        </NuxtLink>
      </div>
    </div>
  </div>
</template>

<style scoped>
.perspective-800 {
  perspective: 800px;
}
.group:hover .rotate-x-2 {
  transform: rotateX(2deg) translateY(-4px);
}
</style>

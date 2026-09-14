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
      return { label: 'Mulai Riset →', to: `/reports/${props.report.id}`, tag: 'Draf' }
    case 'researching':
      return { label: 'Pantau Riset →', to: `/reports/${props.report.id}`, tag: 'Riset' }
    case 'research_completed':
      return { label: 'Susun Laporan →', to: `/reports/${props.report.id}`, tag: 'Riset Selesai' }
    case 'generating':
      return { label: 'Menyusun...', to: `/reports/${props.report.id}`, tag: 'Penyusunan' }
    case 'preview_ready':
    case 'payment_pending':
      return { label: 'Pratinjau & Unduh →', to: `/reports/${props.report.id}`, tag: 'Pratinjau' }
    case 'unlocked':
    case 'paid':
      return { label: 'Unduh DOCX →', to: `/reports/${props.report.id}`, tag: 'Terbuka' }
    case 'failed':
      return { label: 'Coba Ulang →', to: `/reports/${props.report.id}`, tag: 'Terkendala' }
    default:
      return { label: 'Buka →', to: `/reports/${props.report.id}`, tag: 'Dokumen' }
  }
})
</script>

<template>
  <div class="p-5 bg-surface rounded border border-border/60 flex flex-col justify-between space-y-4 hover:border-border transition-colors">
    <div class="space-y-2">
      <div class="flex items-center justify-between text-[11px] font-mono text-ink-muted border-b border-border/40 pb-2">
        <span>{{ statusMeta.tag }}</span>
        <span>{{ formattedDate }}</span>
      </div>

      <h3 class="font-serif text-base text-ink-primary line-clamp-2 leading-snug">
        {{ report.title || 'Laporan Praktik Kerja Lapangan' }}
      </h3>

      <div class="text-xs text-ink-muted space-y-0.5">
        <div>{{ report.internship?.company_name || 'Instansi DU/DI' }}</div>
        <div class="text-[11px]">{{ report.student?.full_name || 'Siswa' }}</div>
      </div>
    </div>

    <div class="pt-3 border-t border-border/40 flex items-center justify-between">
      <span class="text-[10px] font-mono text-ink-muted">
        REF: {{ report.id.slice(0, 8).toUpperCase() }}
      </span>

      <NuxtLink
        :to="statusMeta.to"
        class="text-xs font-medium text-ink-primary hover:text-accent-400 transition-colors"
      >
        {{ statusMeta.label }}
      </NuxtLink>
    </div>
  </div>
</template>


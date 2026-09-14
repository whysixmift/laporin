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

const nextAction = computed(() => {
  switch (props.report.status) {
    case 'draft':
      return { label: 'Mulai Riset', to: `/reports/${props.report.id}`, variant: 'primary' as const }
    case 'researching':
      return { label: 'Pantau Riset', to: `/reports/${props.report.id}`, variant: 'subtle' as const }
    case 'research_completed':
      return { label: 'Susun Laporan', to: `/reports/${props.report.id}`, variant: 'primary' as const }
    case 'generating':
      return { label: 'Pantau Penyusunan', to: `/reports/${props.report.id}`, variant: 'subtle' as const }
    case 'preview_ready':
    case 'payment_pending':
      return { label: 'Lihat Pratinjau', to: `/reports/${props.report.id}`, variant: 'primary' as const }
    case 'unlocked':
    case 'paid':
      return { label: 'Buka & Unduh DOCX', to: `/reports/${props.report.id}`, variant: 'primary' as const }
    case 'failed':
      return { label: 'Coba Ulang', to: `/reports/${props.report.id}`, variant: 'danger' as const }
    default:
      return { label: 'Buka Laporan', to: `/reports/${props.report.id}`, variant: 'subtle' as const }
  }
})
</script>

<template>
  <div class="p-5 rounded-lg border border-border bg-surface hover:border-border-strong transition-colors flex flex-col justify-between group">
    <div>
      <div class="flex items-start justify-between gap-3 mb-2.5">
        <BaseBadge :status="report.status" size="sm" />
        <span class="text-[11px] font-mono text-ink-muted shrink-0">{{ formattedDate }}</span>
      </div>

      <h3 class="font-semibold text-ink-primary text-base group-hover:text-accent-300 transition-colors line-clamp-1 mb-1">
        {{ report.title || 'Laporan PKL Tanpa Judul' }}
      </h3>

      <div class="flex items-center gap-2 text-xs text-ink-secondary mb-4">
        <span class="font-medium text-ink-primary">{{ report.internship?.company_name || 'Perusahaan belum ditentukan' }}</span>
        <span v-if="report.student?.school" class="text-ink-muted">· {{ report.student.school }}</span>
      </div>
    </div>

    <div class="pt-3 border-t border-border-subtle flex items-center justify-between mt-2">
      <div class="text-[11px] text-ink-muted font-mono truncate max-w-[150px]">
        ID: {{ report.id.slice(0, 8) }}
      </div>
      <BaseButton :to="nextAction.to" :variant="nextAction.variant" size="sm">
        {{ nextAction.label }}
      </BaseButton>
    </div>
  </div>
</template>

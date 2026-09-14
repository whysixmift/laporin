<script setup lang="ts">
import type { ReportStatus } from '~/types/api'

interface Props {
  status: ReportStatus | string
  size?: 'sm' | 'md'
}

const props = withDefaults(defineProps<Props>(), {
  size: 'md'
})

interface StatusConfig {
  label: string
  bg: string
  text: string
  border: string
  dot: string
}

const statusMap: Record<string, StatusConfig> = {
  draft: {
    label: 'Draf',
    bg: 'bg-surface-elevated',
    text: 'text-ink-secondary',
    border: 'border-border',
    dot: 'bg-ink-muted'
  },
  researching: {
    label: 'Riset Informasi...',
    bg: 'bg-ochre-500/10',
    text: 'text-ochre-400',
    border: 'border-ochre-500/30',
    dot: 'bg-ochre-400 animate-pulse'
  },
  research_completed: {
    label: 'Riset Selesai',
    bg: 'bg-emerald-500/10',
    text: 'text-emerald-400',
    border: 'border-emerald-500/30',
    dot: 'bg-emerald-400'
  },
  generating: {
    label: 'Menyusun Laporan...',
    bg: 'bg-sky-500/10',
    text: 'text-sky-400',
    border: 'border-sky-500/30',
    dot: 'bg-sky-400 animate-pulse'
  },
  generated: {
    label: 'Laporan Siap',
    bg: 'bg-sky-500/10',
    text: 'text-sky-300',
    border: 'border-sky-500/30',
    dot: 'bg-sky-300'
  },
  preview_ready: {
    label: 'Pratinjau Siap',
    bg: 'bg-accent-500/10',
    text: 'text-accent-300',
    border: 'border-accent-500/30',
    dot: 'bg-accent-400'
  },
  payment_pending: {
    label: 'Menunggu Pembayaran',
    bg: 'bg-ochre-500/10',
    text: 'text-ochre-400',
    border: 'border-ochre-500/30',
    dot: 'bg-ochre-400 animate-pulse'
  },
  paid: {
    label: 'Pembayaran Terverifikasi',
    bg: 'bg-accent-500/15',
    text: 'text-accent-300',
    border: 'border-accent-500/30',
    dot: 'bg-accent-400'
  },
  unlocked: {
    label: 'DOCX Terbuka',
    bg: 'bg-accent-500/15',
    text: 'text-accent-300',
    border: 'border-accent-500/40',
    dot: 'bg-accent-400'
  },
  failed: {
    label: 'Terkendala',
    bg: 'bg-danger-500/10',
    text: 'text-danger-400',
    border: 'border-danger-500/30',
    dot: 'bg-danger-400'
  },
  cancelled: {
    label: 'Dibatalkan',
    bg: 'bg-surface',
    text: 'text-ink-muted',
    border: 'border-border-subtle',
    dot: 'bg-ink-muted'
  },
  expired: {
    label: 'Kedaluwarsa',
    bg: 'bg-danger-500/5',
    text: 'text-danger-400/80',
    border: 'border-danger-500/20',
    dot: 'bg-danger-500/50'
  }
}

const currentConfig = computed(() => {
  return statusMap[props.status] || {
    label: props.status,
    bg: 'bg-surface-elevated',
    text: 'text-ink-secondary',
    border: 'border-border',
    dot: 'bg-ink-muted'
  }
})
</script>

<template>
  <span
    :class="[
      'inline-flex items-center gap-1.5 font-medium rounded-md border select-none',
      currentConfig.bg,
      currentConfig.text,
      currentConfig.border,
      size === 'sm' ? 'px-2 py-0.5 text-xs' : 'px-2.5 py-1 text-xs'
    ]"
  >
    <span :class="['w-1.5 h-1.5 rounded-full shrink-0', currentConfig.dot]" aria-hidden="true" />
    <span>{{ currentConfig.label }}</span>
  </span>
</template>

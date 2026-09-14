<script setup lang="ts">
import type { ReportStatus } from '~/types/api'

interface Props {
  status: ReportStatus
}

const props = defineProps<Props>()

interface Step {
  id: string
  label: string
  activeStatuses: ReportStatus[]
  completedStatuses: ReportStatus[]
}

const steps: Step[] = [
  {
    id: 'data',
    label: 'Data PKL',
    activeStatuses: ['draft'],
    completedStatuses: ['researching', 'research_completed', 'generating', 'generated', 'preview_ready', 'payment_pending', 'paid', 'unlocked']
  },
  {
    id: 'research',
    label: 'Riset Sumber',
    activeStatuses: ['researching'],
    completedStatuses: ['research_completed', 'generating', 'generated', 'preview_ready', 'payment_pending', 'paid', 'unlocked']
  },
  {
    id: 'generation',
    label: 'Penyusunan',
    activeStatuses: ['research_completed', 'generating', 'generated'],
    completedStatuses: ['preview_ready', 'payment_pending', 'paid', 'unlocked']
  },
  {
    id: 'preview',
    label: 'Pratinjau & Bayar',
    activeStatuses: ['preview_ready', 'payment_pending', 'paid'],
    completedStatuses: ['unlocked']
  },
  {
    id: 'unlocked',
    label: 'Unduh DOCX',
    activeStatuses: ['unlocked'],
    completedStatuses: []
  }
]

const getStepState = (step: Step) => {
  if (step.completedStatuses.includes(props.status)) return 'completed'
  if (step.activeStatuses.includes(props.status)) return 'current'
  return 'upcoming'
}
</script>

<template>
  <div class="w-full py-3 px-4 bg-surface rounded-lg border border-border">
    <div class="flex items-center justify-between overflow-x-auto gap-2 text-xs">
      <div
        v-for="(step, index) in steps"
        :key="step.id"
        class="flex items-center gap-2 shrink-0"
      >
        <div class="flex items-center gap-2">
          <!-- Step indicator dot / number -->
          <div
            :class="[
              'w-5 h-5 rounded-full flex items-center justify-center font-mono text-[10px] font-bold transition-colors',
              getStepState(step) === 'completed'
                ? 'bg-accent-600 text-white'
                : getStepState(step) === 'current'
                  ? 'bg-surface-elevated text-accent-400 border border-accent-500/50'
                  : 'bg-surface-subtle text-ink-muted border border-border'
            ]"
          >
            <svg
              v-if="getStepState(step) === 'completed'"
              class="w-3 h-3"
              viewBox="0 0 20 20"
              fill="currentColor"
            >
              <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd" />
            </svg>
            <span v-else>{{ index + 1 }}</span>
          </div>

          <span
            :class="[
              'font-medium',
              getStepState(step) === 'current'
                ? 'text-ink-primary font-semibold'
                : getStepState(step) === 'completed'
                  ? 'text-ink-secondary'
                  : 'text-ink-muted'
            ]"
          >
            {{ step.label }}
          </span>
        </div>

        <!-- Separator arrow -->
        <span v-if="index < steps.length - 1" class="text-ink-faint mx-1" aria-hidden="true">→</span>
      </div>
    </div>
  </div>
</template>

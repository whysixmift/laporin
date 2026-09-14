<script setup lang="ts">
import type { ResearchSource } from '~/types/api'

interface Props {
  sources: ResearchSource[]
}

defineProps<Props>()
</script>

<template>
  <div class="space-y-3">
    <div class="flex items-center justify-between text-xs text-ink-muted px-1">
      <span>Daftar Sumber yang Ditelusuri (Maksimal 5)</span>
      <span class="font-mono">{{ sources.length }} Sumber Terverifikasi</span>
    </div>

    <div class="space-y-2">
      <div
        v-for="(source, index) in sources"
        :key="index"
        class="p-3 bg-surface rounded-lg border border-border flex items-center justify-between gap-4"
      >
        <div class="flex items-center gap-3 min-w-0">
          <div class="w-6 h-6 rounded bg-surface-elevated border border-border flex items-center justify-center text-[10px] font-mono font-bold text-accent-400 shrink-0">
            {{ index + 1 }}
          </div>
          <div class="min-w-0">
            <a
              :href="source.url"
              target="_blank"
              rel="noopener noreferrer"
              class="text-xs font-medium text-ink-primary hover:text-accent-300 truncate block hover:underline"
            >
              {{ source.title || source.url }}
            </a>
            <span class="text-[11px] text-ink-muted truncate block font-mono">
              {{ source.url }}
            </span>
          </div>
        </div>

        <div class="flex items-center gap-3 shrink-0 text-right">
          <span
            v-if="source.confidence !== null && source.confidence !== undefined"
            class="text-[11px] font-mono px-2 py-0.5 rounded bg-surface-elevated border border-border text-accent-300"
          >
            {{ Math.round(source.confidence * 100) }}% Relevan
          </span>
        </div>
      </div>
    </div>
  </div>
</template>

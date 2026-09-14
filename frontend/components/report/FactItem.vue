<script setup lang="ts">
import type { ResearchFact } from '~/types/api'

interface Props {
  fact: ResearchFact
  index: number
}

defineProps<Props>()

const isOpen = ref(false)
</script>

<template>
  <div class="p-4 rounded-lg bg-surface border border-border transition-colors">
    <div class="flex items-start justify-between gap-3">
      <div class="flex items-start gap-3">
        <span class="font-mono text-xs text-accent-400 font-bold mt-0.5 shrink-0">
          #{{ index + 1 }}
        </span>
        <div>
          <p class="text-sm text-ink-primary font-medium leading-relaxed">
            {{ fact.claim }}
          </p>
        </div>
      </div>

      <button
        v-if="fact.sources?.length"
        type="button"
        class="text-xs text-ink-muted hover:text-ink-secondary flex items-center gap-1 shrink-0 p-1 rounded hover:bg-surface-elevated transition-colors"
        @click="isOpen = !isOpen"
      >
        <span>{{ fact.sources.length }} Sumber</span>
        <svg
          :class="['w-3.5 h-3.5 transition-transform duration-150', isOpen ? 'rotate-180' : '']"
          viewBox="0 0 20 20"
          fill="currentColor"
        >
          <path fill-rule="evenodd" d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z" clip-rule="evenodd" />
        </svg>
      </button>
    </div>

    <!-- Sources dropdown -->
    <div v-if="isOpen && fact.sources?.length" class="mt-3 pt-3 border-t border-border-subtle pl-6 flex flex-col gap-2">
      <div
        v-for="(source, sIndex) in fact.sources"
        :key="sIndex"
        class="flex flex-col sm:flex-row sm:items-center justify-between text-xs gap-1 bg-surface-subtle p-2.5 rounded border border-border-subtle"
      >
        <div class="flex items-center gap-2 truncate">
          <svg class="w-3.5 h-3.5 text-accent-500 shrink-0" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71" />
            <path d="M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71" />
          </svg>
          <a
            :href="source.url"
            target="_blank"
            rel="noopener noreferrer"
            class="text-ink-secondary hover:text-accent-300 underline underline-offset-2 truncate"
          >
            {{ source.title || source.url }}
          </a>
        </div>

        <div class="flex items-center gap-3 text-[11px] text-ink-muted shrink-0 pl-5 sm:pl-0">
          <span v-if="source.confidence !== null && source.confidence !== undefined" class="font-mono">
            Keyakinan: {{ Math.round(source.confidence * 100) }}%
          </span>
          <span v-if="source.fetched_at" class="font-mono">
            {{ new Date(source.fetched_at).toLocaleTimeString('id-ID', { hour: '2-digit', minute: '2-digit' }) }}
          </span>
        </div>
      </div>
    </div>
  </div>
</template>

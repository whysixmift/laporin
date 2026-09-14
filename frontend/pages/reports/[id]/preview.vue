<script setup lang="ts">
import type { Report } from '~/types/api'

const route = useRoute()
const reportId = route.params.id as string

const { getReport } = useReports()

const report = ref<Report | null>(null)
const loading = ref(true)
const error = ref('')

onMounted(async () => {
  try {
    report.value = await getReport(reportId)
  } catch (err: unknown) {
    const apiErr = err as { message?: string }
    error.value = apiErr.message || 'Gagal memuat pratinjau.'
  } finally {
    loading.value = false
  }
})
</script>

<template>
  <div class="max-w-5xl mx-auto px-4 sm:px-6 py-8 sm:py-12 space-y-6">
    <div class="flex items-center justify-between pb-4 border-b border-border">
      <NuxtLink :to="`/reports/${reportId}`" class="text-xs text-ink-muted hover:text-ink-secondary flex items-center gap-1.5">
        <span>←</span>
        <span>Kembali ke Laporan</span>
      </NuxtLink>

      <span v-if="report" class="text-xs font-mono text-ink-muted">
        Status: <BaseBadge :status="report.status" size="sm" />
      </span>
    </div>

    <BaseAlert v-if="error" type="error">
      {{ error }}
    </BaseAlert>

    <div v-if="loading" class="p-12 text-center">
      <div class="inline-block animate-spin w-8 h-8 rounded-full border-2 border-accent-500 border-t-transparent" />
    </div>

    <DocumentViewer
      v-else-if="report"
      :report="report"
      :is-watermarked="report.status !== 'unlocked' && report.status !== 'paid'"
    />
  </div>
</template>

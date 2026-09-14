<script setup lang="ts">
interface Props {
  reportId: string
  loading?: boolean
}

defineProps<Props>()

const emit = defineEmits<{
  pay: []
}>()
</script>

<template>
  <div class="w-full max-w-3xl p-6 sm:p-8 bg-surface-elevated rounded-xl border border-accent-500/30 shadow-elevated">
    <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-6">
      <div class="space-y-2">
        <div class="flex items-center gap-2">
          <span class="px-2 py-0.5 rounded text-[11px] font-mono bg-accent-500/20 text-accent-300 border border-accent-500/40">
            Akses Penuh Dokumen
          </span>
          <span class="text-xs text-ink-muted">Format Microsoft Word (.docx)</span>
        </div>

        <h3 class="text-lg sm:text-xl font-bold text-ink-primary">
          Buka & Unduh Dokumen Lengkap
        </h3>

        <p class="text-xs sm:text-sm text-ink-secondary max-w-lg leading-relaxed">
          Dokumen laporan PKL Anda telah disusun dan siap digunakan. Buka kunci file DOCX asli tanpa watermark untuk dicetak atau disesuaikan dengan format khusus sekolah Anda.
        </p>

        <!-- Checklist of what unlocks -->
        <ul class="text-xs text-ink-secondary space-y-1.5 pt-2">
          <li class="flex items-center gap-2">
            <svg class="w-4 h-4 text-accent-400 shrink-0" viewBox="0 0 20 20" fill="currentColor">
              <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd" />
            </svg>
            <span>File DOCX asli (bisa diedit di MS Word / Google Docs)</span>
          </li>
          <li class="flex items-center gap-2">
            <svg class="w-4 h-4 text-accent-400 shrink-0" viewBox="0 0 20 20" fill="currentColor">
              <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd" />
            </svg>
            <span>Halaman sampul resmi, Bab I s.d. Bab IV lengkap</span>
          </li>
          <li class="flex items-center gap-2">
            <svg class="w-4 h-4 text-accent-400 shrink-0" viewBox="0 0 20 20" fill="currentColor">
              <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd" />
            </svg>
            <span>Bebas tanda air (Watermark-free)</span>
          </li>
        </ul>
      </div>

      <!-- Price & CTA Button -->
      <div class="sm:text-right shrink-0 flex flex-col justify-center sm:items-end gap-3 pt-4 sm:pt-0 border-t sm:border-t-0 border-border">
        <div>
          <span class="text-xs text-ink-muted block mb-0.5">Biaya Buka Kunci</span>
          <div class="flex items-baseline sm:justify-end gap-1">
            <span class="text-2xl sm:text-3xl font-black font-mono text-ink-primary">Rp15.000</span>
            <span class="text-xs text-ink-muted font-normal">/ laporan</span>
          </div>
          <span class="text-[11px] text-ink-muted">Pembayaran via QRIS / VA Mayar</span>
        </div>

        <BaseButton
          size="lg"
          variant="primary"
          :loading="loading"
          @click="emit('pay')"
        >
          <template #leading>
            <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <rect x="2" y="5" width="20" height="14" rx="2" />
              <line x1="2" y1="10" x2="22" y2="10" />
            </svg>
          </template>
          Buka File DOCX Sekarang
        </BaseButton>
      </div>
    </div>
  </div>
</template>

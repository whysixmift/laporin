<script setup lang="ts">
interface Props {
  reportId: string
  loading?: boolean
}

const props = defineProps<Props>()

const emit = defineEmits<{
  pay: []
  unlocked: []
}>()

const { isAdmin } = useAuth()
const api = useApi()
const toast = useToast()
const isFreeUnlocking = ref(false)

const handleFreeUnlock = async () => {
  isFreeUnlocking.value = true
  try {
    await api.post(`/reports/${props.reportId}/free-unlock`)
    toast.success('Buka Kunci Berhasil', 'Laporan dibuka gratis untuk Admin.')
    emit('unlocked')
    window.location.reload()
  } catch (err: any) {
    toast.error('Gagal Membuka Kunci', err.message || 'Terjadi kesalahan.')
  } finally {
    isFreeUnlocking.value = false
  }
}
</script>

<template>
  <div class="w-full max-w-3xl p-6 sm:p-8 bg-surface rounded border border-border/80 space-y-6">
    <div class="flex flex-col md:flex-row md:items-start justify-between gap-6">
      <div class="space-y-2 max-w-md">
        <div class="text-[11px] font-mono text-ink-muted uppercase tracking-wider">
          Akses Dokumen Final
        </div>
        <h3 class="text-xl font-serif font-normal text-ink-primary">
          Unduh berkas Word (.docx) tanpa watermark
        </h3>
        <p class="text-xs text-ink-secondary leading-relaxed font-sans">
          Naskah laporan PKL telah selesai dirangkai. Buka kunci untuk mengunduh dokumen Microsoft Word asli siap cetak.
        </p>
      </div>

      <div class="shrink-0 space-y-3 md:text-right">
        <div>
          <span class="text-[11px] font-mono text-ink-muted block uppercase">Biaya Unduh</span>
          <div class="text-2xl sm:text-3xl font-black font-mono text-ink-primary">
            Rp15.000
          </div>
          <span class="text-xs text-ink-muted">Sekali bayar · QRIS / VA</span>
        </div>

        <div class="flex flex-col gap-2">
          <button
            v-if="isAdmin"
            type="button"
            :disabled="isFreeUnlocking"
            class="px-3 py-1.5 rounded bg-ochre-500/20 hover:bg-ochre-500/30 text-ochre-300 border border-ochre-500/40 text-xs font-mono transition-colors"
            @click="handleFreeUnlock"
          >
            {{ isFreeUnlocking ? 'Membuka...' : 'Buka Kunci Gratis (Admin)' }}
          </button>

          <BaseButton
            size="md"
            variant="primary"
            :loading="loading"
            @click="emit('pay')"
          >
            Buka File DOCX Sekarang
          </BaseButton>
        </div>
      </div>
    </div>

    <!-- Manual Transfer / Direct Support -->
    <div class="pt-4 border-t border-border/40 flex flex-col sm:flex-row sm:items-center justify-between gap-3 text-xs text-ink-muted font-mono">
      <span>Bantuan / Transfer Manual Admin:</span>
      <div class="flex items-center gap-3">
        <a
          :href="`https://wa.me/6285117206413?text=Halo%20Admin%20Laporin,%20saya%20mau%20bayar%20laporan%20ID:%20${reportId}`"
          target="_blank"
          rel="noopener noreferrer"
          class="text-ink-secondary hover:text-accent-400 underline underline-offset-2"
        >
          WA: 0851-1720-6413
        </a>
        <span class="text-ink-faint">/</span>
        <a
          :href="`https://wa.me/6288809028653?text=Halo%20Admin%20Laporin,%20saya%20mau%20bayar%20laporan%20ID:%20${reportId}`"
          target="_blank"
          rel="noopener noreferrer"
          class="text-ink-secondary hover:text-accent-400 underline underline-offset-2"
        >
          0888-0902-8653
        </a>
      </div>
    </div>
  </div>
</template>



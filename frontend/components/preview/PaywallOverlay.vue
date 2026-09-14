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
    toast.success('Buka Kunci Berhasil!', 'Laporan berhasil dibuka secara gratis untuk Admin.')
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

        <div class="flex flex-col gap-2 w-full sm:w-auto">
          <!-- Admin Free Unlock Button -->
          <button
            v-if="isAdmin"
            type="button"
            :disabled="isFreeUnlocking"
            class="px-4 py-2.5 rounded-lg bg-amber-500/20 hover:bg-amber-500/30 text-amber-300 border border-amber-500/40 text-xs font-bold transition-colors inline-flex items-center justify-center gap-2 shadow-subtle"
            @click="handleFreeUnlock"
          >
            <svg class="w-4 h-4 text-amber-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 11V7a4 4 0 118 0m-4 8v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2z" />
            </svg>
            <span>{{ isFreeUnlocking ? 'Membuka Kunci...' : 'Buka Kunci Gratis (Admin)' }}</span>
          </button>

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

    <!-- Direct Payment & Manual Confirmation Support Box -->
    <div class="mt-6 pt-6 border-t border-border-subtle/50 bg-surface/50 -mx-6 -mb-6 sm:-mx-8 sm:-mb-8 p-6 rounded-b-xl space-y-3">
      <div class="flex items-center justify-between flex-wrap gap-2">
        <div class="flex items-center gap-2 text-xs font-semibold text-ink-primary">
          <span class="w-2 h-2 rounded-full bg-emerald-400 animate-pulse" />
          <span>Mau Bayar Langsung / Transfer Manual?</span>
        </div>
        <span class="text-[11px] text-ink-muted">Hubungi Admin untuk konfirmasi & aktivasi instan</span>
      </div>

      <div class="grid grid-cols-1 sm:grid-cols-3 gap-2.5 pt-1">
        <!-- WhatsApp 1 -->
        <a
          :href="`https://wa.me/6285117206413?text=Halo%20Admin%20Laporin,%20saya%20mau%20bayar/buka%20kunci%20laporan%20PKL%20(ID:%20${reportId})`"
          target="_blank"
          rel="noopener noreferrer"
          class="flex items-center gap-2.5 p-2.5 rounded-lg bg-emerald-500/10 hover:bg-emerald-500/20 border border-emerald-500/30 text-emerald-400 transition-colors text-xs font-medium"
        >
          <svg class="w-4 h-4 shrink-0 text-emerald-400" viewBox="0 0 24 24" fill="currentColor">
            <path d="M12.031 6.172c-3.181 0-5.767 2.586-5.768 5.766-.001 1.298.38 2.27 1.019 3.287l-.582 2.128 2.182-.573c.978.58 1.911.928 3.145.929 3.178 0 5.767-2.587 5.768-5.766.001-3.187-2.575-5.771-5.764-5.771zm3.392 8.244c-.144.405-.837.774-1.17.824-.299.045-.677.063-1.092-.069-.252-.08-.575-.187-.988-.365-1.739-.751-2.874-2.502-2.961-2.617-.087-.116-.708-.94-.708-1.793s.448-1.273.607-1.446c.159-.173.346-.217.462-.217l.332.006c.106.005.249-.04.39.298.144.347.491 1.2.534 1.287.043.087.072.188.014.304-.058.116-.087.188-.173.289l-.26.304c-.087.086-.177.18-.076.354.101.174.449.741.964 1.201.662.591 1.221.774 1.394.86s.275.072.376-.043c.101-.116.433-.506.549-.68.116-.173.231-.145.39-.087s1.011.477 1.184.564.289.13.332.202c.043.073.043.419-.101.824z" />
          </svg>
          <div class="flex flex-col truncate">
            <span class="text-[10px] text-ink-muted">WhatsApp 1</span>
            <span class="font-mono text-ink-primary">0851-1720-6413</span>
          </div>
        </a>

        <!-- WhatsApp 2 -->
        <a
          :href="`https://wa.me/6288809028653?text=Halo%20Admin%20Laporin,%20saya%20mau%20bayar/buka%20kunci%20laporan%20PKL%20(ID:%20${reportId})`"
          target="_blank"
          rel="noopener noreferrer"
          class="flex items-center gap-2.5 p-2.5 rounded-lg bg-emerald-500/10 hover:bg-emerald-500/20 border border-emerald-500/30 text-emerald-400 transition-colors text-xs font-medium"
        >
          <svg class="w-4 h-4 shrink-0 text-emerald-400" viewBox="0 0 24 24" fill="currentColor">
            <path d="M12.031 6.172c-3.181 0-5.767 2.586-5.768 5.766-.001 1.298.38 2.27 1.019 3.287l-.582 2.128 2.182-.573c.978.58 1.911.928 3.145.929 3.178 0 5.767-2.587 5.768-5.766.001-3.187-2.575-5.771-5.764-5.771zm3.392 8.244c-.144.405-.837.774-1.17.824-.299.045-.677.063-1.092-.069-.252-.08-.575-.187-.988-.365-1.739-.751-2.874-2.502-2.961-2.617-.087-.116-.708-.94-.708-1.793s.448-1.273.607-1.446c.159-.173.346-.217.462-.217l.332.006c.106.005.249-.04.39.298.144.347.491 1.2.534 1.287.043.087.072.188.014.304-.058.116-.087.188-.173.289l-.26.304c-.087.086-.177.18-.076.354.101.174.449.741.964 1.201.662.591 1.221.774 1.394.86s.275.072.376-.043c.101-.116.433-.506.549-.68.116-.173.231-.145.39-.087s1.011.477 1.184.564.289.13.332.202c.043.073.043.419-.101.824z" />
          </svg>
          <div class="flex flex-col truncate">
            <span class="text-[10px] text-ink-muted">WhatsApp 2</span>
            <span class="font-mono text-ink-primary">0888-0902-8653</span>
          </div>
        </a>

        <!-- Email Direct -->
        <a
          :href="`mailto:miftasigma11@gmail.com?subject=Konfirmasi%20Pembayaran%20Laporin%20-%20ID%20${reportId}&body=Halo%20Admin,%20saya%20telah%20melakukan%20pembayaran%20manual%20untuk%20laporan%20PKL%20dengan%20ID:%20${reportId}`"
          class="flex items-center gap-2.5 p-2.5 rounded-lg bg-sky-500/10 hover:bg-sky-500/20 border border-sky-500/30 text-sky-400 transition-colors text-xs font-medium"
        >
          <svg class="w-4 h-4 shrink-0 text-sky-400" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path stroke-linecap="round" stroke-linejoin="round" d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
          </svg>
          <div class="flex flex-col truncate">
            <span class="text-[10px] text-ink-muted">Email Konfirmasi</span>
            <span class="font-mono text-ink-primary truncate">miftasigma11@gmail.com</span>
          </div>
        </a>
      </div>
    </div>
  </div>
</template>


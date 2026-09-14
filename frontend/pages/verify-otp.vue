<script setup lang="ts">
const { verifyOtp, resendOtp } = useAuth()
const route = useRoute()
const router = useRouter()
const toast = useToast()

const email = ref((route.query.email as string) || '')
const otp = ref((route.query.preview_otp as string) || '')
const loading = ref(false)
const resending = ref(false)
const errorMessage = ref('')
const countdown = ref(0)
let timer: ReturnType<typeof setInterval> | null = null

const startCountdown = (seconds = 60) => {
  countdown.value = seconds
  if (timer) clearInterval(timer)
  timer = setInterval(() => {
    if (countdown.value > 0) {
      countdown.value--
    } else if (timer) {
      clearInterval(timer)
      timer = null
    }
  }, 1000)
}

onMounted(() => {
  if (route.query.preview_otp) {
    toast.info('Kode OTP Ditemukan', `Kode OTP otomatis terisi: ${route.query.preview_otp}`)
  }
})

onUnmounted(() => {
  if (timer) clearInterval(timer)
})

const handleVerify = async () => {
  errorMessage.value = ''

  if (!email.value || !otp.value) {
    errorMessage.value = 'Email dan kode OTP wajib diisi.'
    return
  }

  loading.value = true
  try {
    const res = await verifyOtp({
      email: email.value.trim(),
      otp: otp.value.trim()
    })
    toast.success('Verifikasi Berhasil', 'Selamat datang di Laporin!')
    if (res && res.role === 'admin') {
      router.push('/admin')
    } else {
      router.push('/dashboard')
    }
  } catch (err: unknown) {
    const apiErr = err as { message?: string }
    errorMessage.value = apiErr.message || 'Kode OTP tidak valid atau telah kedaluwarsa.'
  } finally {
    loading.value = false
  }
}

const handleResend = async () => {
  if (countdown.value > 0 || resending.value) return
  if (!email.value.trim()) {
    errorMessage.value = 'Masukkan alamat email Anda terlebih dahulu.'
    return
  }

  resending.value = true
  errorMessage.value = ''
  try {
    const res = await resendOtp(email.value.trim())
    if (res.preview_otp) {
      otp.value = res.preview_otp
      toast.info('OTP Baru', `Kode OTP baru: ${res.preview_otp}`)
    } else {
      toast.success('OTP Terkirim', res.message || 'Kode OTP baru telah dikirimkan ke email Anda.')
    }
    startCountdown(60)
  } catch (err: unknown) {
    const apiErr = err as { message?: string }
    errorMessage.value = apiErr.message || 'Gagal mengirim ulang kode OTP. Coba beberapa saat lagi.'
  } finally {
    resending.value = false
  }
}

// Auto submit when 6 chars typed
watch(otp, (newVal) => {
  if (newVal && newVal.trim().length === 6 && !loading.value) {
    handleVerify()
  }
})
</script>

<template>
  <div class="flex-1 flex items-center justify-center px-4 py-12">
    <div class="w-full max-w-md space-y-6">
      <div class="space-y-1 text-center sm:text-left">
        <div class="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-accent-500/10 text-accent-400 text-xs font-semibold mb-2">
          <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z" />
          </svg>
          Verifikasi Akun
        </div>
        <h1 class="text-2xl font-bold text-ink-primary">
          Verifikasi Kode OTP
        </h1>
        <p class="text-xs sm:text-sm text-ink-secondary">
          Masukkan 6 digit kode verifikasi yang dikirimkan untuk mengaktifkan akun Anda.
        </p>
      </div>

      <!-- Error Alert -->
      <BaseAlert v-if="errorMessage" type="error" dismissible @dismiss="errorMessage = ''">
        {{ errorMessage }}
      </BaseAlert>

      <form class="space-y-4" @submit.prevent="handleVerify">
        <BaseInput
          v-model="email"
          label="Alamat Email"
          type="email"
          placeholder="nama@email.com"
          required
        />

        <div class="space-y-1.5">
          <label class="block text-xs font-semibold text-ink-secondary uppercase tracking-wider">
            Kode OTP (6 Digit)
          </label>
          <input
            v-model="otp"
            type="text"
            maxlength="6"
            placeholder="• • • • • •"
            required
            class="w-full text-center text-2xl font-mono font-bold tracking-widest px-4 py-3 bg-surface-elevated border border-border rounded-xl text-ink-primary focus:outline-none focus:border-accent-400 transition-colors"
          />
          <p class="text-[11px] text-ink-muted text-center">
            Periksa juga folder Spam/Promotions jika email tidak muncul di Kotak Masuk.
          </p>
        </div>

        <BaseButton
          type="submit"
          variant="primary"
          size="lg"
          class="w-full justify-center"
          :loading="loading"
        >
          Verifikasi & Masuk Sekarang
        </BaseButton>
      </form>

      <!-- Resend & Help Actions -->
      <div class="p-4 rounded-xl bg-surface border border-border space-y-3">
        <div class="flex items-center justify-between text-xs">
          <span class="text-ink-secondary">Belum menerima kode OTP?</span>
          <button
            type="button"
            class="font-semibold text-accent-400 hover:text-accent-300 disabled:opacity-50 disabled:cursor-not-allowed cursor-pointer transition-colors"
            :disabled="countdown > 0 || resending"
            @click="handleResend"
          >
            <span v-if="resending">Mengirim...</span>
            <span v-else-if="countdown > 0">Kirim ulang ({{ countdown }}s)</span>
            <span v-else>Kirim Ulang Kode</span>
          </button>
        </div>

        <div class="pt-2 border-t border-border flex items-center justify-between text-xs text-ink-muted">
          <span>Kendala verifikasi?</span>
          <a
            href="https://wa.me/6285117206413?text=Halo%20Admin%20Laporin,%20saya%20butuh%20bantuan%20kode%20OTP%20pendaftaran"
            target="_blank"
            rel="noopener noreferrer"
            class="text-emerald-400 hover:text-emerald-300 font-semibold inline-flex items-center gap-1"
          >
            Hubungi WhatsApp Admin
          </a>
        </div>
      </div>

      <div class="text-center pt-2 text-xs text-ink-secondary">
        Sudah punya akun aktif?
        <NuxtLink to="/login" class="text-accent-400 hover:text-accent-300 font-semibold ml-1 underline underline-offset-2">
          Masuk di sini
        </NuxtLink>
      </div>
    </div>
  </div>
</template>

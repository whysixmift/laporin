<script setup lang="ts">
const { register, getGoogleOAuthUrl } = useAuth()
const toast = useToast()
const router = useRouter()

const email = ref('')
const password = ref('')
const confirmPassword = ref('')
const captchaToken = ref('')
const loading = ref(false)
const errorMessage = ref('')
const captchaError = ref('')

const handleSubmit = async () => {
  errorMessage.value = ''
  captchaError.value = ''

  if (!email.value || !password.value) {
    errorMessage.value = 'Email dan kata sandi wajib diisi.'
    return
  }

  if (password.value.length < 8) {
    errorMessage.value = 'Kata sandi minimal 8 karakter.'
    return
  }

  if (password.value !== confirmPassword.value) {
    errorMessage.value = 'Konfirmasi kata sandi tidak cocok.'
    return
  }

  if (!captchaToken.value) {
    captchaError.value = 'Silakan selesaikan verifikasi keamanan di bawah.'
    return
  }

  loading.value = true
  try {
    const res = await register({
      email: email.value.trim(),
      password: password.value,
      captcha_token: captchaToken.value
    })

    if (res.otp_sent) {
      if (res.preview_otp) {
        toast.info('OTP Dibuat', 'Kode OTP verifikasi Anda telah dibuat. Mengalihkan ke halaman verifikasi...')
        router.push(`/verify-otp?email=${encodeURIComponent(email.value.trim())}&preview_otp=${encodeURIComponent(res.preview_otp)}`)
      } else {
        toast.info('OTP Dikirim', res.message || 'Silakan periksa kotak masuk email Anda untuk kode OTP.')
        router.push(`/verify-otp?email=${encodeURIComponent(email.value.trim())}`)
      }
    } else {
      toast.success('Pendaftaran Berhasil', 'Akun berhasil dibuat. Silakan masuk.')
      router.push('/login')
    }
  } catch (err: unknown) {
    const apiErr = err as { message?: string; code?: string }
    if (apiErr.code === 'INVALID_CAPTCHA') {
      captchaError.value = 'Verifikasi keamanan tidak valid.'
      captchaToken.value = ''
    } else if (apiErr.code === 'CONFLICT' || apiErr.code === 'ALREADY_EXISTS') {
      errorMessage.value = 'Email ini sudah terdaftar. Silakan langsung masuk.'
    } else {
      errorMessage.value = apiErr.message || 'Pendaftaran gagal. Silakan periksa kembali data Anda.'
    }
  } finally {
    loading.value = false
  }
}

const handleGoogleLogin = async () => {
  try {
    const url = await getGoogleOAuthUrl()
    window.location.href = url
  } catch (err: unknown) {
    const apiErr = err as { message?: string }
    toast.error('Google OAuth', apiErr.message || 'Gagal memulai autentikasi Google.')
  }
}
</script>

<template>
  <div class="flex-1 flex items-center justify-center px-4 py-12">
    <div class="w-full max-w-md space-y-6">
      <!-- Header -->
      <div class="space-y-1 text-center sm:text-left">
        <h1 class="text-2xl font-bold text-ink-primary">
          Daftar Akun Laporin
        </h1>
        <p class="text-xs sm:text-sm text-ink-secondary">
          Buat akun untuk mulai menyusun laporan PKL Anda.
        </p>
      </div>

      <!-- Google OAuth -->
      <button
        type="button"
        class="w-full p-2.5 rounded-lg border border-border bg-surface hover:bg-surface-elevated text-ink-primary text-sm font-medium transition-colors flex items-center justify-center gap-3 cursor-pointer"
        @click="handleGoogleLogin"
      >
        <svg class="w-4 h-4 shrink-0" viewBox="0 0 24 24">
          <path fill="#EA4335" d="M12 5c1.6 0 3 .6 4.1 1.7l3.1-3.1C17.3 1.8 14.8 1 12 1 7.5 1 3.7 3.6 1.9 7.3l3.7 2.9C6.5 7.3 9 5 12 5z" />
          <path fill="#4285F4" d="M23.5 12.3c0-.8-.1-1.6-.2-2.3H12v4.5h6.5c-.3 1.5-1.1 2.8-2.4 3.7l3.7 2.9c2.2-2 3.7-5 3.7-8.8z" />
          <path fill="#FBBC05" d="M5.6 14.8c-.2-.7-.4-1.5-.4-2.3 0-.8.2-1.6.4-2.3L1.9 7.3C.7 9.7 0 12 0 14.5s.7 4.8 1.9 7.2l3.7-2.9z" />
          <path fill="#34A853" d="M12 23.5c3.2 0 6-1.1 8-3l-3.7-2.9c-1.1.7-2.5 1.2-4.3 1.2-3 0-5.5-2.3-6.4-5.2L1.9 16.5C3.7 20.2 7.5 23.5 12 23.5z" />
        </svg>
        <span>Daftar Cepat dengan Google</span>
      </button>

      <div class="relative flex items-center justify-center">
        <div class="border-t border-border w-full" />
        <span class="bg-canvas px-3 text-[11px] text-ink-muted uppercase font-mono tracking-wider absolute">atau dengan email</span>
      </div>

      <!-- Error Alert -->
      <BaseAlert v-if="errorMessage" type="error" dismissible @dismiss="errorMessage = ''">
        {{ errorMessage }}
      </BaseAlert>

      <!-- Registration Form -->
      <form class="space-y-4" @submit.prevent="handleSubmit">
        <BaseInput
          v-model="email"
          label="Alamat Email"
          type="email"
          placeholder="nama@email.com"
          required
          autocomplete="email"
          hint="Gunakan email aktif untuk verifikasi OTP"
        />

        <BaseInput
          v-model="password"
          label="Kata Sandi"
          type="password"
          placeholder="Minimal 8 karakter"
          required
          autocomplete="new-password"
        />

        <BaseInput
          v-model="confirmPassword"
          label="Ulangi Kata Sandi"
          type="password"
          placeholder="Ulangi kata sandi"
          required
          autocomplete="new-password"
        />

        <!-- CAPTCHA -->
        <CaptchaWidget
          v-model="captchaToken"
          :error="captchaError"
        />

        <BaseButton
          type="submit"
          variant="primary"
          size="lg"
          class="w-full"
          :loading="loading"
        >
          Daftar Akun
        </BaseButton>
      </form>

      <div class="text-center pt-2 text-xs text-ink-secondary">
        Sudah memiliki akun?
        <NuxtLink to="/login" class="text-accent-400 hover:text-accent-300 font-semibold ml-1 underline underline-offset-2">
          Masuk di sini
        </NuxtLink>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
const { verifyOtp } = useAuth()
const route = useRoute()
const router = useRouter()
const toast = useToast()

const email = ref((route.query.email as string) || '')
const otp = ref('')
const loading = ref(false)
const errorMessage = ref('')

const handleVerify = async () => {
  errorMessage.value = ''

  if (!email.value || !otp.value) {
    errorMessage.value = 'Email dan kode OTP wajib diisi.'
    return
  }

  loading.value = true
  try {
    await verifyOtp({
      email: email.value.trim(),
      otp: otp.value.trim()
    })
    toast.success('Verifikasi Berhasil', 'Akun Anda telah aktif. Silakan masuk.')
    router.push('/login')
  } catch (err: unknown) {
    const apiErr = err as { message?: string }
    errorMessage.value = apiErr.message || 'Kode OTP tidak valid atau telah kedaluwarsa.'
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <div class="flex-1 flex items-center justify-center px-4 py-12">
    <div class="w-full max-w-md space-y-6">
      <div class="space-y-1 text-center sm:text-left">
        <h1 class="text-2xl font-bold text-ink-primary">
          Verifikasi Kode OTP
        </h1>
        <p class="text-xs sm:text-sm text-ink-secondary">
          Masukkan kode OTP yang telah dikirimkan ke email Anda untuk mengaktifkan akun.
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

        <BaseInput
          v-model="otp"
          label="Kode OTP"
          type="text"
          placeholder="Masukkan kode OTP"
          required
          hint="Maksimal 3 kali percobaan"
        />

        <BaseButton
          type="submit"
          variant="primary"
          size="lg"
          class="w-full"
          :loading="loading"
        >
          Verifikasi OTP
        </BaseButton>
      </form>

      <div class="text-center pt-2 text-xs text-ink-secondary">
        Kembali ke
        <NuxtLink to="/login" class="text-accent-400 hover:text-accent-300 font-semibold ml-1 underline underline-offset-2">
          Halaman Masuk
        </NuxtLink>
      </div>
    </div>
  </div>
</template>

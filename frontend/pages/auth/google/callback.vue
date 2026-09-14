<script setup lang="ts">
const { handleGoogleCallback } = useAuth()
const route = useRoute()
const router = useRouter()
const toast = useToast()

const error = ref('')
const loading = ref(true)

onMounted(async () => {
  const code = route.query.code as string
  const state = route.query.state as string

  if (!code) {
    error.value = 'Kode otorisasi Google tidak ditemukan.'
    loading.value = false
    return
  }

  try {
    await handleGoogleCallback(code, state)
    toast.success('Berhasil Masuk', 'Selamat datang di Laporin.')
    router.replace('/dashboard')
  } catch (err: unknown) {
    const apiErr = err as { message?: string }
    error.value = apiErr.message || 'Gagal memproses autentikasi Google.'
    loading.value = false
  }
})
</script>

<template>
  <div class="flex-1 flex items-center justify-center px-4 py-16">
    <div class="max-w-md w-full text-center space-y-4">
      <div v-if="loading" class="space-y-3">
        <div class="inline-block animate-spin w-8 h-8 rounded-full border-2 border-accent-500 border-t-transparent" />
        <h2 class="text-lg font-semibold text-ink-primary">Menyelesaikan Autentikasi...</h2>
        <p class="text-xs text-ink-secondary">Mohon tunggu sebentar, kami sedang menghubungkan akun Google Anda.</p>
      </div>

      <div v-else-if="error" class="space-y-4">
        <BaseAlert type="error">
          {{ error }}
        </BaseAlert>
        <BaseButton to="/login" variant="primary">
          Kembali ke Halaman Masuk
        </BaseButton>
      </div>
    </div>
  </div>
</template>

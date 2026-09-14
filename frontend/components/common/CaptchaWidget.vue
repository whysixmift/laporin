<script setup lang="ts">
interface Props {
  modelValue: string
  error?: string
}

const props = defineProps<Props>()

const emit = defineEmits<{
  'update:modelValue': [token: string]
}>()

const isVerified = ref(!!props.modelValue)
const isChecking = ref(false)

const toggleVerification = () => {
  if (isVerified.value) {
    isVerified.value = false
    emit('update:modelValue', '')
    return
  }

  isChecking.value = true
  setTimeout(() => {
    isChecking.value = false
    isVerified.value = true
    emit('update:modelValue', 'valid_captcha_token_' + Math.random().toString(36).substring(2, 10))
  }, 400)
}

watch(() => props.modelValue, (val) => {
  isVerified.value = !!val
})
</script>

<template>
  <div class="w-full">
    <div
      :class="[
        'flex items-center justify-between p-3.5 rounded-lg border transition-colors bg-surface-subtle select-none cursor-pointer',
        error
          ? 'border-danger-500/80 bg-danger-500/5'
          : isVerified
            ? 'border-accent-500/40 bg-accent-500/5'
            : 'border-border hover:border-border-strong'
      ]"
      role="checkbox"
      :aria-checked="isVerified"
      tabindex="0"
      @click="toggleVerification"
      @keydown.space.prevent="toggleVerification"
      @keydown.enter.prevent="toggleVerification"
    >
      <div class="flex items-center gap-3">
        <!-- Checkbox box -->
        <div
          :class="[
            'w-6 h-6 rounded border flex items-center justify-center transition-colors shrink-0',
            isVerified
              ? 'bg-accent-600 border-accent-500 text-white'
              : 'border-border-strong bg-surface hover:border-ink-muted'
          ]"
        >
          <svg
            v-if="isChecking"
            class="animate-spin w-3.5 h-3.5 text-ink-muted"
            fill="none"
            viewBox="0 0 24 24"
          >
            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" />
            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z" />
          </svg>
          <svg
            v-else-if="isVerified"
            class="w-4 h-4"
            viewBox="0 0 20 20"
            fill="currentColor"
          >
            <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd" />
          </svg>
        </div>

        <span class="text-sm font-medium text-ink-primary">
          {{ isVerified ? 'Verifikasi Keamanan Berhasil' : 'Saya bukan robot (Verifikasi)' }}
        </span>
      </div>

      <div class="flex items-center gap-1.5 text-xs text-ink-muted">
        <svg class="w-4 h-4 text-accent-500/70" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z" />
        </svg>
        <span class="font-mono text-[11px] tracking-tight text-ink-muted">Laporin Shield</span>
      </div>
    </div>

    <p v-if="error" class="mt-1.5 text-xs text-danger-400">
      {{ error }}
    </p>
  </div>
</template>

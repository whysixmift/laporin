<script setup lang="ts">
import { onKeyStroke } from '@vueuse/core'

interface Props {
  modelValue: boolean
  title?: string
  maxWidth?: 'sm' | 'md' | 'lg' | 'xl'
}

const props = withDefaults(defineProps<Props>(), {
  maxWidth: 'md'
})

const emit = defineEmits<{
  'update:modelValue': [value: boolean]
  close: []
}>()

const close = () => {
  emit('update:modelValue', false)
  emit('close')
}

// Close on Escape
onKeyStroke('Escape', (e) => {
  if (props.modelValue) {
    e.preventDefault()
    close()
  }
})

const maxWidthClasses = computed(() => {
  switch (props.maxWidth) {
    case 'sm':
      return 'max-w-sm'
    case 'lg':
      return 'max-w-2xl'
    case 'xl':
      return 'max-w-4xl'
    case 'md':
    default:
      return 'max-w-lg'
  }
})
</script>

<template>
  <Teleport to="body">
    <Transition
      enter-active-class="transition duration-150 ease-out"
      enter-from-class="opacity-0"
      enter-to-class="opacity-100"
      leave-active-class="transition duration-100 ease-in"
      leave-from-class="opacity-100"
      leave-to-class="opacity-0"
    >
      <div
        v-if="modelValue"
        class="fixed inset-0 z-50 overflow-hidden bg-black/80 backdrop-blur-[2px] flex items-center justify-center p-3 sm:p-6"
        @click.self="close"
      >
        <div
          :class="[
            'w-full max-h-[90vh] flex flex-col bg-surface-elevated border border-border rounded-xl shadow-doc overflow-hidden transition-all text-ink-primary',
            maxWidthClasses
          ]"
          role="dialog"
          aria-modal="true"
        >
          <!-- Modal Header -->
          <div v-if="title || $slots.header" class="relative z-20 shrink-0 px-5 sm:px-6 py-4 border-b border-border flex items-center justify-between bg-surface-elevated">
            <slot name="header">
              <h3 class="text-base font-semibold text-ink-primary">
                {{ title }}
              </h3>
            </slot>
            <button
              type="button"
              class="p-1 rounded-md text-ink-muted hover:text-ink-primary hover:bg-surface-hover transition-colors"
              aria-label="Tutup"
              @click="close"
            >
              <svg class="w-5 h-5" viewBox="0 0 20 20" fill="currentColor">
                <path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd" />
              </svg>
            </button>
          </div>

          <!-- Modal Body -->
          <div class="relative z-10 flex-1 min-h-0 overflow-y-auto p-5 sm:p-6 overscroll-contain">
            <slot />
          </div>

          <!-- Modal Footer -->
          <div v-if="$slots.footer" class="relative z-20 shrink-0 px-5 sm:px-6 py-3.5 bg-surface-subtle border-t border-border flex items-center justify-end gap-3">
            <slot name="footer" />
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
interface Props {
  type?: 'info' | 'warning' | 'error' | 'success'
  title?: string
  dismissible?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  type: 'info',
  dismissible: false
})

const emit = defineEmits<{
  dismiss: []
}>()

const typeStyles = computed(() => {
  switch (props.type) {
    case 'error':
      return {
        bg: 'bg-danger-500/10',
        border: 'border-danger-500/30',
        text: 'text-danger-300',
        iconColor: 'text-danger-400'
      }
    case 'warning':
      return {
        bg: 'bg-ochre-500/10',
        border: 'border-ochre-500/30',
        text: 'text-ochre-300',
        iconColor: 'text-ochre-400'
      }
    case 'success':
      return {
        bg: 'bg-accent-500/10',
        border: 'border-accent-500/30',
        text: 'text-accent-300',
        iconColor: 'text-accent-400'
      }
    case 'info':
    default:
      return {
        bg: 'bg-surface-elevated',
        border: 'border-border',
        text: 'text-ink-secondary',
        iconColor: 'text-ink-muted'
      }
  }
})
</script>

<template>
  <div
    :class="[
      'p-4 rounded-lg border text-sm leading-relaxed flex items-start gap-3',
      typeStyles.bg,
      typeStyles.border,
      typeStyles.text
    ]"
    role="alert"
  >
    <div class="shrink-0 mt-0.5" :class="typeStyles.iconColor">
      <!-- Error icon -->
      <svg v-if="type === 'error'" class="w-5 h-5" viewBox="0 0 20 20" fill="currentColor">
        <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clip-rule="evenodd" />
      </svg>
      <!-- Warning icon -->
      <svg v-else-if="type === 'warning'" class="w-5 h-5" viewBox="0 0 20 20" fill="currentColor">
        <path fill-rule="evenodd" d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z" clip-rule="evenodd" />
      </svg>
      <!-- Success icon -->
      <svg v-else-if="type === 'success'" class="w-5 h-5" viewBox="0 0 20 20" fill="currentColor">
        <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd" />
      </svg>
      <!-- Info icon -->
      <svg v-else class="w-5 h-5" viewBox="0 0 20 20" fill="currentColor">
        <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z" clip-rule="evenodd" />
      </svg>
    </div>

    <div class="flex-1 min-w-0">
      <div v-if="title" class="font-semibold text-ink-primary mb-1">
        {{ title }}
      </div>
      <div>
        <slot />
      </div>
    </div>

    <button
      v-if="dismissible"
      type="button"
      class="shrink-0 p-1 text-ink-muted hover:text-ink-primary rounded transition-colors"
      @click="emit('dismiss')"
    >
      <svg class="w-4 h-4" viewBox="0 0 20 20" fill="currentColor">
        <path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd" />
      </svg>
    </button>
  </div>
</template>

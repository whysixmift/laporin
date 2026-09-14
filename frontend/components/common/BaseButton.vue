<script setup lang="ts">
interface Props {
  variant?: 'primary' | 'secondary' | 'subtle' | 'danger' | 'ghost'
  size?: 'sm' | 'md' | 'lg'
  type?: 'button' | 'submit' | 'reset'
  disabled?: boolean
  loading?: boolean
  to?: string
  href?: string
}

const props = withDefaults(defineProps<Props>(), {
  variant: 'primary',
  size: 'md',
  type: 'button',
  disabled: false,
  loading: false
})

const nuxtLink = resolveComponent('NuxtLink')

const componentTag = computed(() => {
  if (props.to) return nuxtLink
  if (props.href) return 'a'
  return 'button'
})

const componentProps = computed(() => {
  if (props.to) {
    return { to: props.to }
  }
  if (props.href) {
    return { href: props.href }
  }
  return {
    type: props.type,
    disabled: props.disabled || props.loading
  }
})

const variantClasses = computed(() => {
  switch (props.variant) {
    case 'primary':
      return 'bg-accent-600 hover:bg-accent-500 text-white font-medium shadow-subtle border border-accent-500/40 active:bg-accent-700'
    case 'secondary':
      return 'bg-surface-elevated hover:bg-surface-hover text-ink-primary border border-border font-medium active:bg-surface'
    case 'subtle':
      return 'bg-surface-subtle hover:bg-surface-hover text-ink-secondary hover:text-ink-primary border border-border-subtle'
    case 'danger':
      return 'bg-danger-600/15 hover:bg-danger-600/25 text-danger-400 border border-danger-500/30 active:bg-danger-600/35'
    case 'ghost':
      return 'bg-transparent hover:bg-surface-hover text-ink-secondary hover:text-ink-primary'
    default:
      return 'bg-accent-600 hover:bg-accent-500 text-white'
  }
})

const sizeClasses = computed(() => {
  switch (props.size) {
    case 'sm':
      return 'text-xs px-3 py-1.5 rounded-md gap-1.5'
    case 'lg':
      return 'text-base px-5 py-2.5 rounded-lg gap-2.5'
    case 'md':
    default:
      return 'text-sm px-4 py-2 rounded-md gap-2'
  }
})
</script>

<template>
  <component
    :is="componentTag"
    v-bind="componentProps"
    :class="[
      'inline-flex items-center justify-center transition-colors select-none focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-accent-500/50 cursor-pointer',
      disabled || loading ? 'opacity-50 cursor-not-allowed pointer-events-none' : '',
      variantClasses,
      sizeClasses
    ]"
  >
    <svg
      v-if="loading"
      class="animate-spin -ml-0.5 h-4 w-4 text-current shrink-0"
      xmlns="http://www.w3.org/2000/svg"
      fill="none"
      viewBox="0 0 24 24"
    >
      <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" />
      <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z" />
    </svg>
    <slot v-if="!loading" name="leading" />
    <slot />
    <slot v-if="!loading" name="trailing" />
  </component>
</template>

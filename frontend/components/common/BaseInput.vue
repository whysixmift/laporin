<script setup lang="ts">
interface Props {
  modelValue: string | number | null | undefined
  label?: string
  id?: string
  name?: string
  type?: string
  placeholder?: string
  required?: boolean
  disabled?: boolean
  error?: string
  hint?: string
  autocomplete?: string
}

const props = withDefaults(defineProps<Props>(), {
  type: 'text',
  required: false,
  disabled: false
})

const emit = defineEmits<{
  'update:modelValue': [value: string]
}>()

const generatedId = useId()
const inputId = computed(() => props.id || generatedId)

const onInput = (event: Event) => {
  const target = event.target as HTMLInputElement
  emit('update:modelValue', target.value)
}
</script>

<template>
  <div class="w-full">
    <div v-if="label" class="flex items-center justify-between mb-1.5">
      <label :for="inputId" class="text-xs font-medium text-ink-secondary cursor-pointer">
        {{ label }}
        <span v-if="required" class="text-accent-400 ml-0.5" aria-hidden="true">*</span>
      </label>
      <span v-if="hint && !error" class="text-xs text-ink-muted">{{ hint }}</span>
    </div>

    <div class="relative">
      <input
        :id="inputId"
        :name="name || inputId"
        :type="type"
        :value="modelValue"
        :placeholder="placeholder"
        :required="required"
        :disabled="disabled"
        :autocomplete="autocomplete"
        :aria-invalid="!!error"
        :aria-describedby="error ? `${inputId}-error` : undefined"
        :class="[
          'w-full bg-surface-subtle text-ink-primary placeholder-ink-muted text-sm px-3.5 py-2.5 rounded-md border transition-colors focus:outline-none focus:ring-1',
          error
            ? 'border-danger-500/80 focus:border-danger-500 focus:ring-danger-500/50'
            : 'border-border focus:border-accent-500 focus:ring-accent-500/30',
          disabled ? 'opacity-50 cursor-not-allowed bg-surface' : 'hover:border-border-strong'
        ]"
        @input="onInput"
      />
    </div>

    <p
      v-if="error"
      :id="`${inputId}-error`"
      class="mt-1.5 text-xs text-danger-400 flex items-center gap-1"
      role="alert"
    >
      <svg class="w-3.5 h-3.5 shrink-0" viewBox="0 0 20 20" fill="currentColor">
        <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7 4a1 1 0 11-2 0 1 1 0 012 0zm-1-9a1 1 0 00-1 1v4a1 1 0 102 0V6a1 1 0 00-1-1z" clip-rule="evenodd" />
      </svg>
      {{ error }}
    </p>
  </div>
</template>

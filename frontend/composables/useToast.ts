export interface Toast {
  id: string
  title: string
  description?: string
  type: 'info' | 'success' | 'warning' | 'error'
  duration?: number
}

export const useToast = () => {
  const toasts = useState<Toast[]>('app_toasts', () => [])

  const add = (toast: Omit<Toast, 'id'>) => {
    const id = Math.random().toString(36).substring(2, 9)
    const newToast: Toast = { ...toast, id }
    toasts.value.push(newToast)

    const duration = toast.duration ?? 4500
    if (duration > 0) {
      setTimeout(() => {
        remove(id)
      }, duration)
    }
    return id
  }

  const remove = (id: string) => {
    toasts.value = toasts.value.filter((t) => t.id !== id)
  }

  const success = (title: string, description?: string) => {
    return add({ title, description, type: 'success' })
  }

  const error = (title: string, description?: string) => {
    return add({ title, description, type: 'error', duration: 6000 })
  }

  const warning = (title: string, description?: string) => {
    return add({ title, description, type: 'warning', duration: 5000 })
  }

  const info = (title: string, description?: string) => {
    return add({ title, description, type: 'info' })
  }

  return {
    toasts,
    add,
    remove,
    success,
    error,
    warning,
    info
  }
}

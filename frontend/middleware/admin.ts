export default defineNuxtRouteMiddleware(async (to) => {
  const auth = useAuth()
  auth.initAuth()

  if (!auth.isAuthenticated.value) {
    return navigateTo(`/login?redirect=${encodeURIComponent(to.fullPath)}`)
  }

  // If role is not yet known or not admin, try fetching me from server
  if (auth.authState.value.role !== 'admin') {
    const me = await auth.fetchMe()
    if (!me || me.role !== 'admin') {
      const toast = useToast()
      toast.error('Akses Ditolak', 'Halaman ini hanya dapat diakses oleh Administrator.')
      return navigateTo('/dashboard')
    }
  }
})

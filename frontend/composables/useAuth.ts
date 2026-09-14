import type {
  LoginRequest,
  LoginResponse,
  RegisterRequest,
  RegistrationResponse,
  VerifyOtpRequest
} from '~/types/api'

interface AuthState {
  userId: string | null
  email: string | null
  expiresAt: string | null
  isInitialized: boolean
}

export const useAuth = () => {
  const api = useApi()
  const router = useRouter()
  const toast = useToast()

  const authState = useState<AuthState>('auth_state', () => ({
    userId: null,
    email: null,
    expiresAt: null,
    isInitialized: false
  }))

  const isAuthenticated = computed(() => !!authState.value.userId)

  // Initialize auth from stored metadata if present
  const initAuth = () => {
    if (import.meta.client && !authState.value.isInitialized) {
      try {
        const stored = localStorage.getItem('laporin_auth')
        if (stored) {
          const parsed = JSON.parse(stored)
          if (parsed.expiresAt && new Date(parsed.expiresAt) > new Date()) {
            authState.value.userId = parsed.userId
            authState.value.email = parsed.email
            authState.value.expiresAt = parsed.expiresAt
          } else {
            localStorage.removeItem('laporin_auth')
          }
        }
      } catch {
        // ignore parse error
      }
      authState.value.isInitialized = true
    }
  }

  const setAuth = (userId: string, email: string, expiresAt: string) => {
    authState.value.userId = userId
    authState.value.email = email
    authState.value.expiresAt = expiresAt
    if (import.meta.client) {
      localStorage.setItem(
        'laporin_auth',
        JSON.stringify({ userId, email, expiresAt })
      )
    }
  }

  const clearAuth = () => {
    authState.value.userId = null
    authState.value.email = null
    authState.value.expiresAt = null
    if (import.meta.client) {
      localStorage.removeItem('laporin_auth')
    }
  }

  const register = async (payload: RegisterRequest): Promise<RegistrationResponse> => {
    const res = await api.post<RegistrationResponse>('/auth/register', payload)
    return res
  }

  const login = async (payload: LoginRequest): Promise<LoginResponse> => {
    const res = await api.post<LoginResponse>('/auth/login', payload)
    setAuth(res.user_id, payload.email, res.expires_at)
    return res
  }

  const verifyOtp = async (payload: VerifyOtpRequest): Promise<void> => {
    await api.post('/auth/verify-otp', payload)
  }

  const getGoogleOAuthUrl = async (): Promise<string> => {
    const res = await api.get<{ url: string }>('/auth/google/oauth_url')
    return res.url
  }

  const handleGoogleCallback = async (code: string, state?: string): Promise<void> => {
    await api.post('/auth/google/callback', { code, state })
    // If successful, set initialized state and redirect
    authState.value.isInitialized = true
  }

  const logout = async (): Promise<void> => {
    try {
      await api.post('/auth/logout')
    } catch {
      // Continue client cleanup even if network fails
    } finally {
      clearAuth()
      toast.info('Keluar', 'Anda telah berhasil keluar.')
      router.push('/login')
    }
  }

  return {
    authState,
    isAuthenticated,
    initAuth,
    setAuth,
    clearAuth,
    register,
    login,
    verifyOtp,
    getGoogleOAuthUrl,
    handleGoogleCallback,
    logout
  }
}

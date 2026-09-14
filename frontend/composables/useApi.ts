import type { ErrorResponse } from '~/types/api'

export class ApiError extends Error {
  code: string
  requestId?: string
  status: number

  constructor(message: string, code: string, status: number, requestId?: string) {
    super(message)
    this.name = 'ApiError'
    this.code = code
    this.status = status
    this.requestId = requestId
  }
}

export const useApi = () => {
  const config = useRuntimeConfig()
  const apiBase = config.public.apiBase || '/api/v1'
  const toast = useToast()

  const request = async <T>(
    endpoint: string,
    options: RequestInit = {}
  ): Promise<T> => {
    const url = endpoint.startsWith('http') ? endpoint : `${apiBase}${endpoint.startsWith('/') ? '' : '/'}${endpoint}`

    const headers: Record<string, string> = {
      'Accept': 'application/json',
      ...(options.headers as Record<string, string> || {})
    }

    if (options.body && typeof options.body === 'string' && !headers['Content-Type']) {
      headers['Content-Type'] = 'application/json'
    }

    try {
      const response = await fetch(url, {
        ...options,
        headers,
        credentials: 'include'
      })

      // Rate limit check
      if (response.status === 429) {
        const resetHeader = response.headers.get('X-RateLimit-Reset')
        const resetSeconds = resetHeader ? parseInt(resetHeader, 10) : 60
        toast.warning(
          'Terlalu Banyak Permintaan',
          `Batas kuota tercapai. Silakan coba lagi dalam ${resetSeconds} detik.`
        )
      }

      if (!response.ok) {
        let errorCode = 'UNKNOWN_ERROR'
        let errorMessage = `Permintaan gagal (${response.status})`
        let requestId: string | undefined

        try {
          const errorJson = await response.json() as ErrorResponse
          if (errorJson.error) {
            errorCode = errorJson.error.code || errorCode
            errorMessage = errorJson.error.message || errorMessage
            requestId = errorJson.error.request_id
          }
        } catch {
          // If non-JSON response error
          if (response.status === 401) {
            errorCode = 'UNAUTHORIZED'
            errorMessage = 'Sesi Anda telah berakhir. Silakan masuk kembali.'
          } else if (response.status === 403) {
            errorCode = 'FORBIDDEN'
            errorMessage = 'Akses ditolak.'
          } else if (response.status === 404) {
            errorCode = 'NOT_FOUND'
            errorMessage = 'Data tidak ditemukan.'
          } else if (response.status === 500) {
            errorCode = 'INTERNAL_ERROR'
            errorMessage = 'Terjadi kendala pada server. Silakan coba beberapa saat lagi.'
          }
        }

        throw new ApiError(errorMessage, errorCode, response.status, requestId)
      }

      // Handle 204 No Content
      if (response.status === 204) {
        return {} as T
      }

      return await response.json() as T
    } catch (err: unknown) {
      if (err instanceof ApiError) {
        throw err
      }
      const message = err instanceof Error ? err.message : 'Koneksi jaringan gagal'
      throw new ApiError(message, 'NETWORK_ERROR', 0)
    }
  }

  const get = <T>(endpoint: string, options?: RequestInit) => {
    return request<T>(endpoint, { ...options, method: 'GET' })
  }

  const post = <T>(endpoint: string, body?: unknown, options?: RequestInit) => {
    return request<T>(endpoint, {
      ...options,
      method: 'POST',
      body: body ? JSON.stringify(body) : undefined
    })
  }

  const patch = <T>(endpoint: string, body?: unknown, options?: RequestInit) => {
    return request<T>(endpoint, {
      ...options,
      method: 'PATCH',
      body: body ? JSON.stringify(body) : undefined
    })
  }

  const getBlob = async (endpoint: string): Promise<Blob> => {
    const url = endpoint.startsWith('http') ? endpoint : `${apiBase}${endpoint.startsWith('/') ? '' : '/'}${endpoint}`
    const response = await fetch(url, {
      method: 'GET',
      credentials: 'include'
    })

    if (!response.ok) {
      let message = 'Gagal mengunduh dokumen'
      try {
        const errorJson = await response.json() as ErrorResponse
        if (errorJson.error?.message) {
          message = errorJson.error.message
        }
      } catch {
        // use default message
      }
      throw new ApiError(message, 'DOWNLOAD_FAILED', response.status)
    }

    return await response.blob()
  }

  return {
    apiBase,
    request,
    get,
    post,
    patch,
    getBlob
  }
}

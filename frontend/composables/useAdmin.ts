import type {
  AdminJobItem,
  AdminMetrics,
  AdminReportItem,
  AiPlaygroundRequest,
  AiPlaygroundResponse,
  Report,
  UserResponse
} from '~/types/api'

export const useAdmin = () => {
  const api = useApi()
  const toast = useToast()

  const loading = ref(false)

  const getMetrics = async (): Promise<AdminMetrics> => {
    return await api.get<AdminMetrics>('/admin/metrics')
  }

  const getUsers = async (
    page = 1,
    limit = 20,
    search?: string
  ): Promise<{ users: UserResponse[]; total: number; page: number; limit: number; total_pages: number }> => {
    let url = `/admin/users?page=${page}&limit=${limit}`
    if (search) url += `&search=${encodeURIComponent(search)}`
    return await api.get(url)
  }

  const updateUserRole = async (userId: string, role: string): Promise<void> => {
    await api.patch(`/admin/users/${userId}/role`, { role })
    toast.success('Berhasil', `Role pengguna berhasil diubah menjadi ${role}.`)
  }

  const updateUserStatus = async (userId: string, isActive: boolean): Promise<void> => {
    await api.patch(`/admin/users/${userId}/status`, { is_active: isActive })
    toast.success('Berhasil', `Status pengguna diperbarui.`)
  }

  const deleteUser = async (userId: string): Promise<void> => {
    await api.delete(`/admin/users/${userId}`)
    toast.success('Berhasil', 'Pengguna berhasil dihapus.')
  }

  const getReports = async (
    page = 1,
    limit = 20,
    status?: string,
    search?: string
  ): Promise<{ reports: AdminReportItem[]; total: number; page: number; limit: number; total_pages: number }> => {
    let url = `/admin/reports?page=${page}&limit=${limit}`
    if (status) url += `&status=${encodeURIComponent(status)}`
    if (search) url += `&search=${encodeURIComponent(search)}`
    return await api.get(url)
  }

  const getReport = async (reportId: string): Promise<Report> => {
    return await api.get<Report>(`/admin/reports/${reportId}`)
  }

  const unlockReportFree = async (reportId: string): Promise<void> => {
    await api.post(`/admin/reports/${reportId}/unlock`)
    toast.success('Sukses Terbuka Gratis!', 'Laporan telah dibuka kuncinya tanpa bayar.')
  }

  const regenerateReport = async (reportId: string): Promise<void> => {
    await api.post(`/admin/reports/${reportId}/regenerate`)
    toast.success('Regenerasi Dimulai', 'Proses pembuatan ulang laporan dimasukkan ke antrean.')
  }

  const deleteReport = async (reportId: string): Promise<void> => {
    await api.delete(`/admin/reports/${reportId}`)
    toast.success('Dihapus', 'Laporan berhasil dihapus.')
  }

  const getJobs = async (
    page = 1,
    limit = 30,
    jobType?: string,
    status?: string
  ): Promise<{ jobs: AdminJobItem[]; page: number; limit: number }> => {
    let url = `/admin/jobs?page=${page}&limit=${limit}`
    if (jobType) url += `&job_type=${encodeURIComponent(jobType)}`
    if (status) url += `&status=${encodeURIComponent(status)}`
    return await api.get(url)
  }

  const retryJob = async (jobId: string): Promise<void> => {
    await api.post(`/admin/jobs/${jobId}/retry`)
    toast.success('Tugas Dimulai Ulang', 'Tugas berhasil dimasukkan kembali ke antrean.')
  }

  const cancelJob = async (jobId: string): Promise<void> => {
    await api.post(`/admin/jobs/${jobId}/cancel`)
    toast.info('Dibatalkan', 'Tugas berhasil dibatalkan.')
  }

  const runAiPlayground = async (
    payload: AiPlaygroundRequest
  ): Promise<AiPlaygroundResponse> => {
    return await api.post<AiPlaygroundResponse>('/admin/ai/playground', payload)
  }

  const getSystemHealth = async (): Promise<any> => {
    return await api.get('/admin/system/health')
  }

  return {
    loading,
    getMetrics,
    getUsers,
    updateUserRole,
    updateUserStatus,
    deleteUser,
    getReports,
    getReport,
    unlockReportFree,
    regenerateReport,
    deleteReport,
    getJobs,
    retryJob,
    cancelJob,
    runAiPlayground,
    getSystemHealth
  }
}

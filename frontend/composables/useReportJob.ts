import type { JobInfo } from '~/types/api'

export const useReportJob = () => {
  const api = useApi()
  const toast = useToast()

  const startResearch = async (reportId: string): Promise<JobInfo> => {
    return await api.post<JobInfo>(`/reports/${reportId}/research/start`)
  }

  const getResearchStatus = async (reportId: string): Promise<JobInfo> => {
    return await api.get<JobInfo>(`/reports/${reportId}/research/status`)
  }

  const startGeneration = async (reportId: string): Promise<JobInfo> => {
    return await api.post<JobInfo>(`/reports/${reportId}/generation/start`)
  }

  const getGenerationStatus = async (reportId: string): Promise<JobInfo> => {
    return await api.get<JobInfo>(`/reports/${reportId}/generation/status`)
  }

  /**
   * Polls a job endpoint until completion, error, or timeout.
   */
  const pollJob = async (
    fetchStatus: () => Promise<JobInfo>,
    onUpdate: (job: JobInfo) => void,
    options: {
      intervalMs?: number
      maxTimeoutMs?: number
    } = {}
  ): Promise<JobInfo> => {
    const intervalMs = options.intervalMs ?? 2500
    const maxTimeoutMs = options.maxTimeoutMs ?? 12 * 60 * 1000 // 12 minutes
    const startTime = Date.now()

    return new Promise((resolve, reject) => {
      const check = async () => {
        try {
          if (Date.now() - startTime > maxTimeoutMs) {
            reject(new Error('Waktu tunggu proses telah habis (timeout). Silakan periksa kembali beberapa saat lagi.'))
            return
          }

          const job = await fetchStatus()
          onUpdate(job)

          if (job.status === 'succeeded') {
            resolve(job)
            return
          }

          if (job.status === 'failed') {
            const msg = job.error_message || 'Proses gagal dijalankan oleh sistem.'
            reject(new Error(msg))
            return
          }

          if (job.status === 'cancelled') {
            reject(new Error('Proses telah dibatalkan.'))
            return
          }

          // Continue polling if pending or running
          setTimeout(check, intervalMs)
        } catch (err) {
          reject(err)
        }
      }

      // Initial check immediately
      check()
    })
  }

  return {
    startResearch,
    getResearchStatus,
    startGeneration,
    getGenerationStatus,
    pollJob
  }
}

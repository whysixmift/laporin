import type { Payment, PaymentResponse, Report } from '~/types/api'

export const usePayment = () => {
  const api = useApi()

  const createPayment = async (reportId: string, returnUrl?: string): Promise<PaymentResponse> => {
    const defaultReturnUrl = import.meta.client
      ? `${window.location.origin}/reports/${reportId}?payment_return=1`
      : `/reports/${reportId}`

    return await api.post<PaymentResponse>('/payments', {
      report_id: reportId,
      return_url: returnUrl || defaultReturnUrl
    })
  }

  const getPayment = async (paymentId: string): Promise<Payment> => {
    return await api.get<Payment>(`/payments/${paymentId}`)
  }

  /**
   * Polls report and payment status until backend grants entitlement (status: 'unlocked')
   */
  const pollPaymentStatus = async (
    paymentId: string,
    reportId: string,
    onStatusChange: (status: string) => void,
    options: { intervalMs?: number; maxTimeoutMs?: number } = {}
  ): Promise<boolean> => {
    const intervalMs = options.intervalMs ?? 3000
    const maxTimeoutMs = options.maxTimeoutMs ?? 5 * 60 * 1000 // 5 min
    const startTime = Date.now()

    return new Promise((resolve) => {
      const check = async () => {
        try {
          if (Date.now() - startTime > maxTimeoutMs) {
            resolve(false)
            return
          }

          // 1. Check report status directly (authoritative unlock flag)
          const report = await api.get<Report>(`/reports/${reportId}`)
          if (report.status === 'unlocked' || report.status === 'paid') {
            onStatusChange(report.status)
            resolve(true)
            return
          }

          // 2. Check payment status
          const payment = await getPayment(paymentId)
          onStatusChange(payment.status)

          if (payment.status === 'succeeded') {
            // Wait brief moment for report update or re-fetch
            const updatedReport = await api.get<Report>(`/reports/${reportId}`)
            if (updatedReport.status === 'unlocked') {
              resolve(true)
              return
            }
          }

          if (payment.status === 'failed' || payment.status === 'expired' || payment.status === 'cancelled') {
            resolve(false)
            return
          }

          setTimeout(check, intervalMs)
        } catch {
          setTimeout(check, intervalMs)
        }
      }

      check()
    })
  }

  return {
    createPayment,
    getPayment,
    pollPaymentStatus
  }
}

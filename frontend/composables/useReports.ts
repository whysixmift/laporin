import type {
  GeneratedSections,
  GeneratedSectionsUpdate,
  LogbookEntry,
  LogbookEntryCreate,
  Report,
  ReportCreate,
  ReportStatus,
  ReportUpdate
} from '~/types/api'

export const useReports = () => {
  const api = useApi()

  const listReports = async (status?: ReportStatus): Promise<Report[]> => {
    const endpoint = status ? `/reports?status=${encodeURIComponent(status)}` : '/reports'
    return await api.get<Report[]>(endpoint)
  }

  const getReport = async (id: string): Promise<Report> => {
    return await api.get<Report>(`/reports/${id}`)
  }

  const createReport = async (payload: ReportCreate): Promise<Report> => {
    return await api.post<Report>('/reports', payload)
  }

  const updateReport = async (id: string, payload: ReportUpdate): Promise<void> => {
    await api.patch(`/reports/${id}`, payload)
  }

  const updateSections = async (id: string, payload: GeneratedSectionsUpdate): Promise<GeneratedSections> => {
    return await api.patch<GeneratedSections>(`/reports/${id}/sections`, payload)
  }

  const getLogbookEntries = async (id: string): Promise<LogbookEntry[]> => {
    return await api.get<LogbookEntry[]>(`/reports/${id}/logbook`)
  }

  const createLogbookEntry = async (id: string, payload: LogbookEntryCreate): Promise<LogbookEntry> => {
    return await api.post<LogbookEntry>(`/reports/${id}/logbook`, payload)
  }

  const deleteLogbookEntry = async (id: string, entryId: string): Promise<void> => {
    await api.delete(`/reports/${id}/logbook/${entryId}`)
  }

  const downloadDocx = async (id: string, filename?: string): Promise<void> => {
    const blob = await api.getBlob(`/reports/${id}/download`)
    const url = window.URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    a.download = filename || `laporan-pkl-${id.slice(0, 8)}.docx`
    document.body.appendChild(a)
    a.click()
    window.URL.revokeObjectURL(url)
    document.body.removeChild(a)
  }

  return {
    listReports,
    getReport,
    createReport,
    updateReport,
    updateSections,
    getLogbookEntries,
    createLogbookEntry,
    deleteLogbookEntry,
    downloadDocx
  }
}

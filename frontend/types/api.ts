export type ReportStatus =
  | 'draft'
  | 'researching'
  | 'research_completed'
  | 'generating'
  | 'generated'
  | 'preview_ready'
  | 'payment_pending'
  | 'paid'
  | 'unlocked'
  | 'failed'
  | 'cancelled'
  | 'expired'

export type JobType = 'research' | 'generation'

export type JobStatus = 'pending' | 'running' | 'succeeded' | 'failed' | 'cancelled'

export type PaymentStatus = 'created' | 'pending' | 'succeeded' | 'failed' | 'cancelled' | 'expired'

export interface StudentInfo {
  full_name: string
  student_id: string
  school: string
  major?: string | null
  semester?: string | null
}

export interface InternshipInfo {
  company_name: string
  company_address?: string | null
  department?: string | null
  role?: string | null
  start_date?: string | null
  end_date?: string | null
  company_supervisor?: string | null
  school_supervisor?: string | null
  description?: string | null
}

export interface ResearchSource {
  url: string
  title?: string | null
  fetched_at: string
  confidence?: number | null
}

export interface ResearchFact {
  claim: string
  sources: ResearchSource[]
}

export interface GeneratedSections {
  cover: string
  introduction: string
  company_profile: string
  activities: string
  conclusion: string
  is_user_edited?: boolean
}

export interface GeneratedSectionsUpdate {
  cover?: string
  introduction?: string
  company_profile?: string
  activities?: string
  conclusion?: string
}

export interface LogbookEntry {
  id: string
  report_id: string
  entry_date: string
  activity_title: string
  tasks_performed: string
  tools_technologies?: string | null
  problems_encountered?: string | null
  solutions_applied?: string | null
  skills_learned?: string | null
  evidence_notes?: string | null
  created_at: string
  updated_at: string
}

export interface LogbookEntryCreate {
  entry_date: string
  activity_title: string
  tasks_performed: string
  tools_technologies?: string
  problems_encountered?: string
  solutions_applied?: string
  skills_learned?: string
  evidence_notes?: string
}

export interface Report {
  id: string
  title: string
  status: ReportStatus
  created_at: string
  updated_at?: string
  student: StudentInfo
  internship: InternshipInfo
  research_facts?: ResearchFact[]
  generated_sections?: GeneratedSections
  generated_doc_path?: string | null
  logbook_count?: number
}

export interface ReportCreate {
  title: string
  student: StudentInfo
  internship: InternshipInfo
}

export interface ReportUpdate {
  student?: Partial<StudentInfo>
  internship?: Partial<InternshipInfo>
}

export interface JobInfo {
  job_id: string
  type: JobType
  status: JobStatus
  started_at?: string | null
  finished_at?: string | null
  error_code?: string | null
  error_message?: string | null
}

export interface PaymentCreate {
  report_id: string
  return_url: string
}

export interface PaymentResponse {
  payment_id: string
  payment_url: string
}

export interface Payment {
  payment_id: string
  report_id: string
  status: PaymentStatus
  amount?: number
  created_at?: string
}

export interface ErrorDetail {
  code: string
  message: string
  request_id?: string
}

export interface ErrorResponse {
  error: ErrorDetail
}

export interface RegisterRequest {
  email: string
  password: string
  captcha_token: string
}

export interface RegistrationResponse {
  user_id: string
  otp_sent: boolean
  preview_otp?: string
  message?: string
}

export interface ResendOtpResponse {
  message: string
  otp_sent: boolean
  preview_otp?: string
}

export interface LoginRequest {
  email: string
  password: string
  captcha_token: string
}

export interface LoginResponse {
  user_id: string
  email?: string
  role?: string
  expires_at: string
}

export interface UserResponse {
  id: string
  email: string
  role: string
  is_active: boolean
  created_at: string
  report_count?: number
}

export interface SystemMetrics {
  memory_used_mb: number
  memory_total_mb: number
  memory_percentage: number
  disk_free_gb: number
  libreoffice_available: boolean
  environment: string
}

export interface AdminMetrics {
  total_users: number
  total_admins: number
  total_reports: number
  unlocked_reports: number
  total_revenue_idr: number
  active_jobs: number
  status_breakdown: Record<string, number>
  system: SystemMetrics
}

export interface AdminReportItem {
  id: string
  user_id: string
  user_email: string
  title: string
  status: ReportStatus
  student_name?: string | null
  company_name?: string | null
  created_at: string
  updated_at?: string
}

export interface AdminJobItem {
  job_id: string
  report_id: string
  report_title?: string | null
  user_email?: string | null
  job_type: 'research' | 'generation'
  status: JobStatus
  attempts: number
  max_attempts: number
  created_at: string
  started_at?: string | null
  finished_at?: string | null
  error_code?: string | null
  error_message?: string | null
}

export interface AiPlaygroundRequest {
  type: 'llm' | 'crawl'
  prompt?: string
  system_prompt?: string
  url?: string
}

export interface AiPlaygroundResponse {
  test_type: string
  success: boolean
  output: Record<string, any>
  duration_ms: number
  error?: string | null
}

export interface VerifyOtpRequest {
  email: string
  otp: string
}

export interface GoogleCallback {
  code: string
  state?: string
}


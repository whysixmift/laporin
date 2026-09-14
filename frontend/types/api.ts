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
}

export interface LoginRequest {
  email: string
  password: string
  captcha_token: string
}

export interface LoginResponse {
  user_id: string
  expires_at: string
}

export interface VerifyOtpRequest {
  email: string
  otp: string
}

export interface GoogleCallback {
  code: string
  state?: string
}

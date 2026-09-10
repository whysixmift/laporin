---
name: frontend-ux-states
description: Comprehensive UI state modeling (idle, loading, processing, error, locked, preview_ready, unlocked) across the report lifecycle.
---

# Frontend UX States & Lifecycle Skill

## 1. Scope & Ownership
Claude Code owns the state-machine-driven presentation layer in the Nuxt UI.

## 2. Canonical UI State Machine Mapping
The frontend UI renders views strictly corresponding to `report.status`:

| Report Status | UI State | Primary Actions / Display |
|---|---|---|
| `draft` | Edit Mode | Form inputs for student info & internship data; "Start Research" CTA. |
| `researching` | Active Job | Polling spinner, real-time research progress status message. |
| `research_completed` | Review Facts | List of extracted `research_facts` with verifiable source links; "Generate Report" CTA. |
| `generating` | Active Job | Document assembly progress indicator / spinner. |
| `generated` | Finalizing | Transitioning to preview conversion. |
| `preview_ready` | Paywall / Preview | Embedded PDF viewer displaying watermarked preview; "Pay to Unlock" Mayar button. |
| `payment_pending` | Awaiting Webhook | Spinner / countdown polling payment status; "Refresh Payment Status" button. |
| `paid` | Processing Entitlement | Temporary backend unlock confirmation state. |
| `unlocked` | Unlocked / Complete | Full unlocked badge; "Download DOCX" button enabled. |
| `failed` | Error State | Actionable error banner with specific `error_message` and "Retry Job" button. |
| `cancelled` | Inactive | Informational notice; option to create new report. |
| `expired` | Expired State | Notice that file retention period has elapsed (30 days); prompt to regenerate. |

## 3. Critical UI Invariant: Payment Unlock
- ⚠️ **NEVER imply a report is unlocked based on client assumptions**.
- Even after redirecting back from Mayar, the UI must display `payment_pending` and query `GET /reports/{id}` until the backend returns `status: "unlocked"`.
- The `.docx` download button is ONLY enabled when `report.status === 'unlocked'`.

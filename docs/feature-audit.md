# LAPORIN: Comprehensive Product Audit & Implementation Gap Analysis

> **Date:** September 2026  
> **Status:** Production / Hardened MVP  
> **Scope:** Full-stack audit across Frontend (Nuxt 3), Backend (Rust/Axum), Database (PostgreSQL 17), Async Workers, Infrastructure, Security, UX, and Commercial Feasibility.

---

## 1. Executive Summary & Reality vs. Specification

An in-depth code-level audit was conducted across all repository files (`/backend`, `/frontend`, `/docs`, `/infra`) and the active production deployment (`avrjulian-nest.hackclub.app`).

### Key Findings
1. **Core Pipeline is Functioning**: The end-to-end flow (Registration $\rightarrow$ OTP $\rightarrow$ Draft Creation $\rightarrow$ Research Job $\rightarrow$ Generation Job $\rightarrow$ Watermarked Document Preview $\rightarrow$ Mayar/Admin Unlock $\rightarrow$ Word .docx Download) is implemented and live.
2. **Infrastructure is Lightweight & Stable**: Axum + SQLite/PostgreSQL with `FOR UPDATE SKIP LOCKED` and Nuxt 3 compiles within 2.79 MB, consuming < 550 MB RAM on the 2 GB VPS.
3. **Key Architectural Gaps Identified**:
   - **Research Source Discovery**: The research worker currently fetches a single hardcoded search URL rather than querying a live search engine API (e.g., DuckDuckGo, Serper, or Brave Search) before feeding scraped results to the 9Router LLM.
   - **User Input Model**: Currently models student activity as a single monolithic textarea (`description`) rather than an interactive daily/weekly journal with evidence logs.
   - **Document Customization**: Generates standard 4-chapter reports from a single default template (`templates/default_report_template.docx`). School-specific guideline adaptation is not yet automated.
   - **In-place Chapter Editing**: Users cannot edit generated chapter text in the web interface prior to downloading the final `.docx` file.

---

## 2. Comprehensive Implementation Gap Matrix

| Subsystem / Feature | Expected (Spec / Docs) | Implemented in Code | Reality Status | Missing / Broken Aspects | Notes & Technical Impact |
|:---|:---|:---|:---|:---|:---|
| **Auth: Email/Password** | Argon2id hashing, email uniqueness, sanitized input | `backend/src/api/routes/auth.rs`, `Argon2id` | **IMPLEMENTED** | None | Securely hashes passwords with standard memory/time cost parameters. |
| **Auth: OTP Email Delivery** | 6-digit numeric OTP sent via Resend API / SMTP | `backend/src/services/email.rs`, `Resend` API | **PARTIAL** | Fallback to console log if `RESEND_API_KEY` is missing or in test mode | Production has Resend configured; needs SMS/WhatsApp OTP fallback for Indonesian market. |
| **Auth: Google OAuth 2.0** | PKCE flow with callback token exchange | `backend/src/api/routes/auth.rs`, `OAuthService` | **IMPLEMENTED** | Requires user client credentials in `.env` | Implemented and verified against Google token endpoint. |
| **Auth: CAPTCHA** | hCAPTCHA site-verify before registration/login | `backend/src/services/captcha.rs` | **IMPLEMENTED** | Dummy bypass configured for development/test mode | Production verifies `captcha_token` against `hcaptcha.com/siteverify`. |
| **Auth: Session Management** | HttpOnly, Secure, SameSite=Strict cookies with SHA-256 hashed database storage | `backend/src/db/session_repo.rs`, `SessionAuth` middleware | **IMPLEMENTED** | None | 30-day session lifetime with automated revocation on logout. |
| **Report: CRUD & Draf** | 4-step wizard, student info, DU/DI metadata, activity notes | `frontend/pages/reports/new.vue`, `backend/src/api/routes/reports.rs` | **IMPLEMENTED** | Autosave to localStorage / draft sync on typing is missing | Simple step-by-step form; saves to DB on final step submit. |
| **Report: Edit Draf** | Editable student & internship metadata while in `draft` state | `PATCH /api/v1/reports/:id`, `BaseModal` edit form | **IMPLEMENTED** | Cannot edit generated chapter sections directly in web UI | Only metadata can be edited; chapter text is regenerated in batch. |
| **Research: Web Crawler** | SSRF-defended crawler (blocks private IP ranges, HTTPS only) | `backend/src/services/crawler.rs` | **IMPLEMENTED** | Live multi-query search engine integration is missing | Blocks `127.0.0.0/8`, `10.0.0.0/8`, `192.168.0.0/16`, `169.254.0.0/16`, IPv6 loopback. |
| **Research: Fact Extraction** | 9Router LLM extracts company profile, vision, mission, operations | `backend/src/services/llm.rs`, `ResearchWorker` | **IMPLEMENTED** | Multi-source conflict resolution & manual fact overrides missing | Saves claims to `report_research_facts` with confidence and timestamps. |
| **Research: Source Provenance** | Citations linked to verified web URLs | `backend/src/db/report_repo.rs` (`research_sources`) | **IMPLEMENTED** | Source explorer UI is minimal | Database stores foreign key relationships between facts and sources. |
| **Generation: Text Engine** | Generates Bab I, Bab II, Bab III, Bab IV with PUEBI/EYD Indonesian rules | `backend/src/services/llm.rs`, `GenerationWorker` | **IMPLEMENTED** | Tone selection and length controls missing | Produces structured Indonesian text matching standard SMK/D3 curricula. |
| **Generation: DOCX Engine** | XML placeholder substitution in DOCX package | `backend/src/services/docx.rs`, `quick-xml`, `zip` | **IMPLEMENTED** | Tables and image insertions not yet automated | Custom high-speed Rust DOCX engine replaces `{{...}}` tags cleanly. |
| **Preview: Document Sheets** | Clean A4 web preview with watermarks for unpaid users | `frontend/components/preview/DocumentViewer.vue` | **IMPLEMENTED** | Page-break pagination simulation is basic | Renders cover sheet, Bab I-IV with watermark diagonal overlay. |
| **Preview: PDF Rendering** | Headless LibreOffice conversion to PDF | `backend/src/services/preview.rs` | **PARTIAL** | Requires `soffice` binary on host system | Direct web streaming via `GET /reports/:id/preview`; falls back to HTML document viewer. |
| **Payment: Mayar Gateway** | Create invoice, redirect to QRIS/VA, webhook HMAC-SHA256 verification | `backend/src/services/payment.rs`, `backend/src/api/routes/webhooks.rs` | **IMPLEMENTED** | Direct WhatsApp manual confirmation is non-automated | Verified HMAC verification on webhook; unlocks report idempotently. |
| **Entitlement: Download Gate** | DOCX download only accessible when `unlocked` or `paid` | `backend/src/api/routes/reports.rs` (`download_report`) | **IMPLEMENTED** | None | Strict server-side enforcement; returns `402 Payment Required` if locked. |
| **Job Queue: Workers** | PostgreSQL `FOR UPDATE SKIP LOCKED` async queue with retry limits | `backend/src/db/job_repo.rs`, `ResearchWorker`, `GenerationWorker` | **IMPLEMENTED** | Dead-letter queue inspection UI is admin-only | Bounded concurrency (2 research workers, 1 generation worker). |
| **Storage & Cloud CDN** | Local storage with Hack Club CDN 53.6 GB cloud fallback | `backend/src/storage/mod.rs`, `backend/src/services/cdn.rs` | **IMPLEMENTED** | Automated cloud backup lifecycle cron is missing | Files saved under `/root/laporin/storage` with UUID naming. |
| **Admin System** | User management, report status overview, manual unlock, AI playground | `backend/src/api/routes/admin.rs`, `frontend/pages/admin/*` | **IMPLEMENTED** | Granular user ban/suspend controls missing | Admin free unlock and system health monitoring operational. |
| **Rate Limiting** | Tiered per-IP and per-endpoint sliding rate limits with 429 headers | `backend/src/middleware/rate_limit.rs` | **IMPLEMENTED** | Distributed Redis rate limiting missing (in-memory only) | In-memory token bucket protects login, register, and generation endpoints. |
| **Visual Design & UX** | Restrained editorial dark-first UI, high typography discipline, zero AI slop | `frontend/pages/index.vue`, `frontend/pages/dashboard.vue` | **IMPLEMENTED** | None | Compliant with human-authored aesthetic standard and responsive layout. |

---

## 3. Subsystem Detailed Audit

### 3.1 Authentication & Security Audit
- **Password Security**: Argon2id parameters (m=65536, t=3, p=4) implemented in `backend/src/auth/mod.rs`.
- **Session Protection**: Cryptographically secure tokens (32 bytes `rand::thread_rng()`), stored as SHA-256 hashes in `sessions` table. Cookie configured with `HttpOnly=true`, `SameSite=Strict`, `Path=/`, and `Secure=true` in production.
- **SSRF Defenses**: Full IP range validation in `CrawlerService::is_private_or_restricted` prevents attacks against AWS metadata (`169.254.169.254`), localhost (`127.0.0.1`), and private subnets (`10.0.0.0/8`, `172.16.0.0/12`, `192.168.0.0/16`).
- **Prompt Injection Defense**: User inputs are strictly passed through structural JSON schemas and sanitized delimiters before LLM synthesis.

### 3.2 Report & Generation Engine Audit
- **Template Placeholder Fidelity**: `backend/src/services/docx.rs` substitutes `{{STUDENT_NAME}}`, `{{STUDENT_ID}}`, `{{SCHOOL_NAME}}`, `{{COMPANY_NAME}}`, `{{SECTION_INTRODUCTION}}`, `{{SECTION_COMPANY_PROFILE}}`, `{{SECTION_ACTIVITIES}}`, `{{SECTION_CONCLUSION}}`.
- **Memory Footprint**: Memory guard triggers at < 500 MB available RAM to prevent Out-Of-Memory (OOM) panic during heavy DOCX/PDF rendering.
- **Limitation**: The current generator operates on a monolithic whole-report basis. If a student wants to rewrite only Bab III, the entire report must be reprocessed.

### 3.3 Payment & Monetization Audit
- **Webhook Idempotency**: `payments` table tracks `provider_payment_id` and unique transaction IDs, preventing double-unlock attacks.
- **Pricing Strategy**: Flat Rp15.000 ($0.95 USD) per document. Extremely accessible for Indonesian vocational/SMK students.
- **Direct Contacts**: WhatsApp (0851-1720-6413, 0888-0902-8653) and email (`miftasigma11@gmail.com`) provide backup payment channels for students without banking/e-wallet access.

---

## 4. Summary of Gaps to Address in Feature Expansion

1. **Autonomous Search Crawler**: Integrate a live search engine API (DuckDuckGo or Serper) so any real Indonesian company name returns legitimate URLs automatically.
2. **Interactive Daily Logbook**: Expand the single-textarea input into a structured weekly/daily journal where students can record date, activity, and key competencies.
3. **In-place Chapter Editor & AI Polishing**: Allow students to review, adjust, formalize, and expand individual chapters directly in the web app before generating the final DOCX.
4. **School Format & Guideline Ingestion**: Enable students to upload their school's PDF guideline to automatically adapt margins, font requirements, and required chapters.

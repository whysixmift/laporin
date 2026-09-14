# Laporin (Full Stack) 📋⚡

> High-reliability, automated internship report generator (*Laporan PKL / Magang*) engineered for Indonesian students (SMK, Politeknik, Universitas).

Laporin automates the end-to-end generation of standardized academic internship reports. It leverages a modern Nuxt 3 frontend with dark-first typography and document-first preview, an SSRF-protected web crawler, 9Router LLM structured extraction, a PostgreSQL `FOR UPDATE SKIP LOCKED` worker queue, DOCX placeholder substitution, headless PDF preview rendering, and Mayar payment gateway integration.

---

## 🏗️ System Architecture

```
                                  ┌───────────────────────────┐
                                  │       Client Browser      │
                                  └─────────────┬─────────────┘
                                                │ HTTP / Vue 3
                                                ▼
┌─────────────────────────────────────────────────────────────────────────────────────────┐
│                                Nuxt 3 Frontend (Port 3000)                              │
│                                                                                         │
│  - Dark-First Visual Identity & Academic Document Preview                               │
│  - Multi-step Report Wizard & 12-state Lifecycle Management                             │
│  - Session-authenticated fetch wrapper with 429 rate limit countdown                    │
│  - Nitro reverse proxy for `/api/v1/**`                                                 │
└───────────────────────────────────────────────┬─────────────────────────────────────────┘
                                                │ REST API / Session Cookie
                                                ▼
┌─────────────────────────────────────────────────────────────────────────────────────────┐
│                                Laporin Backend Axum (Port 8080)                         │
│                                                                                         │
│  ┌───────────────────────┐  ┌────────────────────────┐  ┌────────────────────────────┐  │
│  │   Auth & Middleware   │  │   Report CRUD Routes   │  │   Mayar Webhook Handler    │  │
│  │ (Argon2id, OAuth, OTP)│  │ (Bab I - Bab IV Schema)│  │  (HMAC-SHA256 Signature)   │  │
│  └───────────┬───────────┘  └───────────┬────────────┘  └─────────────┬──────────────┘  │
│              │                          │                             │                 │
│              ▼                          ▼                             ▼                 │
│  ┌───────────────────────────────────────────────────────────────────────────────────┐  │
│  │                               PostgreSQL Database                                 │  │
│  │      - Users & Sessions (SHA-256 token hash, HttpOnly SameSite=Strict)            │  │
│  │      - Reports, Facts (USER_FACT, RESEARCH_FACT, AI_DERIVED), & Generated Sections│  │
│  │      - Async Job Queue (`FOR UPDATE SKIP LOCKED`) & Payment Ledger                │  │
│  └──────────────────────────────────────┬────────────────────────────────────────────┘  │
│                                         │                                               │
│             ┌───────────────────────────┴───────────────────────────┐                   │
│             ▼                                                       ▼                   │
│  ┌───────────────────────────┐                         ┌─────────────────────────────┐  │
│  │   Research Worker Pool    │                         │  Document Generation Pool   │  │
│  │ - SSRF-Defended Crawler   │                         │ - XML Placeholder Engine    │  │
│  │ - 9Router LLM Extractor   │                         │ - Headless LibreOffice PDF  │  │
│  │ - Provenance Citation Link│                         │ - Entitlement Gated Download│  │
│  └───────────────────────────┘                         └─────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────────────────────────┘
```

---

## ✨ Features & Capabilities

1. **Authentication & Session Security**
   - User registration with password hashing via **Argon2id**.
   - Google OAuth 2.0 exchange and auto-linking.
   - 6-digit numeric OTP generation & verification (SHA-256 hashed in DB) with per-email rate limiting and max attempt threshold.
   - Opaque 32-byte cryptographically secure session IDs hashed with SHA-256 and stored in `HttpOnly; SameSite=Strict; Path=/; Secure` cookies.
   - hCaptcha verification middleware with mock support for local testing.

2. **Report Project Lifecycle & Data Model**
   - Canonical 5-chapter report structure compliant with Indonesian academic standards:
     - **Bab I**: Pendahuluan (Latar Belakang, Maksud & Tujuan, Waktu & Tempat).
     - **Bab II**: Tinjauan Umum Perusahaan (Profil Perusahaan, Sejarah, Struktur Organisasi).
     - **Bab III**: Pelaksanaan Praktik Kerja (Bidang Pekerjaan, Pelaksanaan, Kendala & Solusi).
     - **Bab IV**: Pembahasan & Hasil (Topik Khusus, Analisis Teknis, Hasil Kerja).
     - **Bab V**: Penutup (Kesimpulan & Saran).
   - Strict multi-tenant data isolation guaranteeing users can only access their own reports and generation jobs.

3. **Autonomous AI Research Pipeline**
   - **SSRF Defenses**: Validates DNS resolutions, strictly blocking all private and loopback IPv4/IPv6 ranges (`127.0.0.0/8`, `10.0.0.0/8`, `172.16.0.0/12`, `192.168.0.0/16`, `::1`, `fc00::/7`), link-local metadata addresses (`169.254.169.254`), and enforces a 2 MB content size limit.
   - **Fact Extraction**: 9Router LLM extraction pipeline with fact classification (`USER_FACT`, `RESEARCH_FACT`, `AI_DERIVED_TEXT`) and provenance URLs.

4. **Async Worker Queue**
   - Built on top of PostgreSQL with `FOR UPDATE SKIP LOCKED` semantics, ensuring no two workers pick the same task.
   - Configurable bounded worker pools with concurrency locks for low memory footprint on VPS (2 GB RAM budget).

5. **DOCX Document Generation & PDF Preview**
   - Fast zip-based XML template placeholder substitution (`{{student_name}}`, `{{company_name}}`, `{{chapter1_content}}`, etc.).
   - Low-memory PDF preview rendering via headless LibreOffice (`soffice`) with built-in fallbacks.

6. **Mayar Payment Gateway Integration**
   - Dynamic payment link creation with fixed Rp15.000 pricing.
   - HMAC-SHA256 webhook signature verification (`x-mayar-signature`).
   - Idempotent transaction handling based on `transaction_id`.
   - Backend-authoritative unlock gating for clean `.docx` file downloads.

---

## 📁 Repository Structure

```
laporin/
├── .agents/skills/            # Agent skills and engineering guidelines
├── backend/                   # Axum Rust Backend (Port 8080)
│   ├── Cargo.toml             # Rust dependencies & metadata
│   ├── .env.example           # Backend environment configuration template
│   ├── migrations/            # SQLx database migration scripts
│   │   └── 20260910000000_initial_schema.sql
│   ├── src/
│   │   ├── main.rs            # Server bootstrapper & signal handling
│   │   ├── lib.rs             # Axum app router & state initialization
│   │   ├── config/            # Strongly-typed environment configuration
│   │   ├── errors/            # Centralized AppError & OpenAPI-compliant error response
│   │   ├── domain/            # Domain models (Auth, Report, Job, Payment)
│   │   ├── db/                # Repository layer (Users, Sessions, OTPs, Reports, Jobs, Payments)
│   │   ├── middleware/        # Auth, Rate limiting, Request ID, Tracing
│   │   ├── services/          # Crawler (SSRF), LLM, DOCX, Preview, Mayar, OAuth, Captcha
│   │   ├── storage/           # Local file storage, path traversal guard, disk quotas
│   │   ├── workers/           # Background job processors (Research, Generation, Cleanup)
│   │   └── api/routes/        # REST controllers (Auth, Reports, Research, Generation, Payments, Webhook)
│   └── tests/                 # Integration test suite (10/10 passed)
├── frontend/                  # Nuxt 3 / Vue 3 Frontend (Port 3000)
│   ├── components/            # Reusable UI components & document viewers
│   ├── composables/           # Type-safe API clients, auth, jobs, payment
│   ├── pages/                 # Routing (Landing, Auth, Dashboard, Report Workspace, Preview)
│   ├── types/                 # TypeScript interfaces matching OpenAPI 3.1
│   ├── assets/css/            # Dark-first tokens & print stylesheet
│   └── tests/                 # Playwright E2E & Full Lifecycle suite (12/12 passed)
├── docs/                      # Architectural & frozen contract specifications
│   ├── openapi.yaml           # OpenAPI 3.1.0 contract
│   ├── database-schema.md     # PostgreSQL ER schema and indexes
│   └── report-schema.md       # Indonesian 5-chapter report template schema
├── storage/                   # Storage mount points (templates, reports, tmp)
│   ├── templates/
│   ├── reports/
│   └── tmp/
├── .gitignore
└── README.md
```

---

## 🚀 Quick Start

### 1. Prerequisites
- **Rust** 1.75+ (`rustup default stable`)
- **PostgreSQL** 14+
- **LibreOffice** (optional, for PDF preview conversion: `soffice`)

### 2. Database Setup
Create the database in PostgreSQL:
```bash
createdb laporin -U postgres
```

### 3. Environment Configuration
Copy the sample `.env.example` inside the `backend` folder:
```bash
cp backend/.env.example backend/.env
```

Key environment variables:
| Variable | Description | Default / Example |
|---|---|---|
| `DATABASE_URL` | PostgreSQL connection string | `postgres://postgres@localhost:5432/laporin` |
| `SERVER_HOST` | Host address to bind | `127.0.0.1` |
| `SERVER_PORT` | Port number to bind | `3000` |
| `SESSION_SECRET` | Secret key for session signature | `32+ character random string` |
| `MAYAR_API_KEY` | Mayar Gateway API key | `test_key` |
| `MAYAR_WEBHOOK_SECRET` | HMAC-SHA256 secret for webhook validation | `test_webhook_secret` |
| `NINEROUTER_API_KEY` | 9Router AI API key | `test_llm_key` |
| `HCAPTCHA_SECRET` | hCaptcha secret key | `test_captcha_secret` |
| `STORAGE_BASE_PATH` | Local file storage root | `../storage` |

### 4. Run Migrations & Build
```bash
cd backend
sqlx migrate run
cargo build
```

### 5. Run the Backend Server
```bash
cd backend
cargo run
```
The backend server will start listening at `http://127.0.0.1:8080`.

### 6. Run the Frontend App
```bash
cd frontend
pnpm install
pnpm run build
node .output/server/index.mjs  # Production server on http://127.0.0.1:3000
# or for development:
# pnpm run dev
```

---

## 🧪 Running Automated Tests

### 1. Backend Integration Tests (Rust)
```bash
cd backend
cargo test -- --nocapture
```

The backend test suite covers:
- **Authentication & Sessions**: Registration, Argon2id verification, Google OAuth linking, OTP generation and attempt throttling, session cookie issuance and revocation.
- **Report CRUD**: Report creation, multi-chapter retrieval, validation, and multi-tenant isolation.
- **SSRF Defense**: Strict rejection of private loopback, 10.x, 172.x, 192.x, AWS metadata endpoints, and DNS rebinding attacks.
- **Async Processing**: PostgreSQL `FOR UPDATE SKIP LOCKED` worker execution for research extraction and document generation.
- **Document Generation**: DOCX zip template manipulation, XML sanitization, and fallback PDF generation.
- **Payment & Webhooks**: Mayar payment creation, HMAC-SHA256 signature verification, and duplicate webhook idempotency.
- **Entitlement Download**: 403 Forbidden enforcement on unpaid reports, 200 OK binary stream on unlocked reports.
- **End-to-End User Flow**: Full multi-step simulation from user signup to final report download.

### 2. Frontend E2E & Lifecycle Tests (Playwright)
```bash
cd frontend
pnpm exec playwright test
```

The frontend Playwright suite verifies:
- **Landing & Marketing**: Academic value proposition, chapter preview showcase, and Rp15.000 pricing clarity.
- **Authentication Flow**: Account creation, bot protection CAPTCHA, error alerts, and login session initialization.
- **Report Management**: Empty states, active reports grid, status indicators, and 4-step wizard.
- **Interactive Workspace**: Draft edits, live polling status during AI research & document compilation.
- **Academic Preview & Paywall**: Formatted A4 sheet rendering, watermarked preview, Mayar checkout modal, and DOCX binary download.
- **Cross-Device Responsiveness**: Full desktop and mobile viewports (iPhone / Android).

---

## 📡 API Contract Overview

All endpoints adhere strictly to the frozen `docs/openapi.yaml` OpenAPI 3.1 specification.

| Method | Endpoint | Description | Auth Required |
|---|---|---|---|
| `POST` | `/auth/register` | Register a new user account with email & password | No |
| `POST` | `/auth/login` | Authenticate user and receive session cookie | No |
| `GET` | `/auth/google/oauth_url`| Get Google OAuth 2.0 authorization redirect URL | No |
| `POST` | `/auth/google/callback` | Exchange Google OAuth code for session | No |
| `POST` | `/auth/verify-otp` | Verify 6-digit numeric OTP | No |
| `POST` | `/auth/logout` | Revoke session and clear session cookie | Yes |
| `POST` | `/reports` | Create a new internship report project | Yes |
| `GET` | `/reports` | List all reports owned by the authenticated user | Yes |
| `GET` | `/reports/:id` | Get report details, facts, and generated sections | Yes |
| `PATCH` | `/reports/:id` | Update report metadata and student/company info | Yes |
| `POST` | `/reports/:id/research/start` | Trigger background SSRF-safe AI research | Yes |
| `GET` | `/reports/:id/research/status`| Poll research worker status & progress | Yes |
| `POST` | `/reports/:id/generation/start`| Trigger background report document rendering | Yes |
| `GET` | `/reports/:id/generation/status`| Poll document generation worker status | Yes |
| `GET` | `/reports/:id/preview` | Download watermark PDF preview | Yes |
| `POST` | `/payments` | Create Mayar payment transaction for report | Yes |
| `GET` | `/payments/:id` | Check payment status | Yes |
| `POST` | `/webhook/mayar` | Webhook receiver for Mayar payment notification | HMAC Verified |
| `GET` | `/reports/:id/download` | Download unlocked clean `.docx` document | Entitlement (Paid) |

---

## 🔒 Security & Invariants

- **Zero-Trust SSRF Engine**: Binds to standard socket resolution and parses IPv4/IPv6 addresses against RFC1918, RFC3927, RFC4291, and RFC6890 private blocks before making HTTP requests.
- **Password Security**: Argon2id with recommended parameters (`m_cost=19456`, `t_cost=2`, `p_cost=1`).
- **Session Protection**: HttpOnly, SameSite=Strict cookies with SHA-256 server-side hashing preventing session hijacking.
- **Idempotent Webhooks**: Uses database transaction locks and checks existing `transaction_id` states to avoid double unlocks or race conditions.
- **Resource Constraints**: Strict limits on payload sizes (max 2 MB for crawler), bounded thread pools, and memory-safe stream pipelines suitable for a 2 GB VPS.

---

## 📄 License
This project is licensed under the MIT License.

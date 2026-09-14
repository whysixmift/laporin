# Laporin Master System Documentation 📋⚡

> Comprehensive Technical, Architectural, and Operational Manual for the Laporin Automated Internship Report Generator (*Laporan PKL / Magang*).

---

## 1. Executive Summary & Product Architecture

**Laporin** is an automated web application designed for Indonesian students (SMK, Politeknik, Universitas) that transforms raw internship journals and company data into standardized 4/5-chapter academic reports compliant with standard Indonesian curriculum requirements.

### Core User Journey
1. **User Authentication**: Email/Password with Argon2id & numeric OTP or Google OAuth 2.0 with anti-bot CAPTCHA protection.
2. **Draft Submission**: 4-step wizard collecting student identity, placement metadata, and daily activity logs.
3. **Autonomous AI Research**: SSRF-defended crawler extracts official company profiles, visions, missions, and business operations via 9Router LLM with provenance tracking.
4. **Academic Document Compilation**: Merges student logs and company facts into academic chapters (Bab I - Bab IV), generating both watermarked preview data and a Microsoft Word (`.docx`) file.
5. **Entitlement & Paywall**: Fixed pricing (Rp15.000) processed via Mayar Payment Gateway with HMAC-SHA256 validated webhook unlock.
6. **Delivery**: Secure, backend-authoritative DOCX download streaming.

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

## 2. Backend Architecture (Rust / Axum)

### 2.1 Framework & Core Crates
* **Web Framework**: `axum` (0.7) with `tokio` multi-threaded runtime.
* **Database Driver**: `sqlx` (0.7) with PostgreSQL connection pooling and compile-time verification.
* **Security & Auth**: `argon2` (password hashing), `ring` / `sha2` (HMAC signatures & session token hashing), `rand` (crypto-random session generation).
* **HTTP Client & Crawler**: `reqwest` (with custom DNS resolver for SSRF mitigation) and `scraper` (HTML parsing).
* **Document Engine**: `zip` (DOCX template manipulation), `quick-xml` (XML text stream replacement).
* **Observability**: `tracing` & `tracing-subscriber` (structured JSON logging and span tracking).

### 2.2 API Layer & Routes
All routes conform to OpenAPI 3.1 (`docs/openapi.yaml`):
* **Authentication** (`/api/v1/auth/*`):
  * `POST /register`: Registers user, generates password hash with Argon2id, creates OTP.
  * `POST /login`: Verifies password or active session, issues HttpOnly cookie.
  * `POST /verify-otp`: Validates 6-digit numeric OTP with exponential throttling.
  * `GET /google/oauth_url` & `POST /google/callback`: Google OAuth 2.0 PKCE exchange.
  * `POST /logout`: Revokes database session record and clears cookie.
* **Report Management** (`/api/v1/reports/*`):
  * `POST /reports`: Creates a new report in `draft` state.
  * `GET /reports`: Lists current user's reports with pagination.
  * `GET /reports/:id`: Returns full report details, facts, and generated content.
  * `PATCH /reports/:id`: Updates student identity or company data while in editable states.
  * `POST /reports/:id/research/start` & `GET /reports/:id/research/status`: Starts/polls research jobs.
  * `POST /reports/:id/generation/start` & `GET /reports/:id/generation/status`: Starts/polls generation jobs.
  * `GET /reports/:id/preview`: Streams watermarked document preview.
  * `GET /reports/:id/download`: Streams authorized `.docx` file (requires `unlocked` or `paid` state).
* **Payments & Webhooks** (`/api/v1/*`):
  * `POST /payments`: Initiates Mayar payment link for Rp15.000.
  * `GET /payments/:id`: Queries payment status.
  * `POST /webhook/mayar`: Verifies HMAC-SHA256 signature and performs idempotent report unlock.

### 2.3 Background Workers (`FOR UPDATE SKIP LOCKED`)
1. **Research Worker**:
   - Fetches pending tasks from `jobs` table with `status = 'pending'` and `job_type = 'research'`.
   - Uses `FOR UPDATE SKIP LOCKED` so concurrent worker instances never claim the same job.
   - Enforces DNS checks preventing SSRF (blocking `127.0.0.0/8`, `10.0.0.0/8`, `172.16.0.0/12`, `192.168.0.0/16`, `169.254.169.254`, `::1`).
   - Dispatches parsed company data to 9Router LLM for structured fact extraction.
2. **Generation Worker**:
   - Compiles facts into academic sections (Bab I: Pendahuluan, Bab II: Profil Perusahaan, Bab III: Pelaksanaan, Bab IV: Penutup).
   - Injects section text into `storage/templates/report-template-v1.docx` XML content streams.
   - Updates report status to `preview_ready`.
3. **Session Cleanup Worker**:
   - Periodically purges expired sessions from database every hour.

---

## 3. Frontend Architecture (Nuxt 3 / Vue 3)

### 3.1 Design System & Typography
* **Color Tokens**:
  * Background Root: `#0c0e12` (Onyx)
  * Surface Cards: `#13171f`
  * Surface Elevated / Modals: `#1e2430`
  * Borders: `#2d3747` (subtle contrast)
  * Primary Text: `#f1f4f8` (warm off-white)
  * Secondary Text: `#94a3b8` (slate)
  * Accent Emerald: `#10b981` (action / unlocked)
  * Ochre / Amber: `#f59e0b` (research in-progress)
  * Sky Blue: `#38bdf8` (generation in-progress)
* **Typography Hierarchy**:
  * Headings & Document Titles: *Newsreader* / *Plus Jakarta Sans*
  * Body & Interactive Elements: *Plus Jakarta Sans*
  * Code, Identifiers, Timestamps: *JetBrains Mono*

### 3.2 Key Views
* **Landing Page** (`pages/index.vue`): Clear presentation of the product value, chapter previews, and fixed pricing.
* **Authentication** (`pages/login.vue`, `pages/register.vue`, `pages/verify-otp.vue`): Interactive forms with mock bot protection, password requirements, and OAuth redirection.
* **Dashboard** (`pages/dashboard.vue`): Filterable report cards with status badges and creation triggers.
* **Wizard** (`pages/reports/new.vue`): 4-step structured form ensuring complete data capture.
* **Report Workspace** (`pages/reports/[id]/index.vue`): Comprehensive dashboard handling draft editing, live async polling, status steppers, watermarked previews, Mayar payment modal, and DOCX download.
* **Full-Screen Document Viewer** (`pages/reports/[id]/preview.vue`): Dedicated reading view with `@media print` layout.

---

## 4. Database Architecture (PostgreSQL)

### 4.1 Tables & Entity Relationships

```
┌──────────────────┐       ┌──────────────────────┐
│      users       │1     *│       sessions       │
├──────────────────┤───────├──────────────────────┤
│ id (UUID, PK)    │       │ id (UUID, PK)        │
│ email (UNIQUE)   │       │ user_id (FK)         │
│ password_hash    │       │ token_hash (SHA-256) │
│ is_verified      │       │ expires_at           │
└────────┬─────────┘       └──────────────────────┘
         │1
         │*
┌────────┴─────────┐       ┌──────────────────────┐
│     reports      │1     *│         jobs         │
├──────────────────┤───────├──────────────────────┤
│ id (UUID, PK)    │       │ id (UUID, PK)        │
│ user_id (FK)     │       │ report_id (FK)       │
│ title            │       │ job_type             │
│ status           │       │ status (SKIP LOCKED) │
│ student (JSONB)  │       │ payload (JSONB)      │
│ internship(JSONB)│       │ error_message        │
└────────┬─────────┘       └──────────────────────┘
         │1
         │*
┌────────┴─────────┐       ┌──────────────────────┐
│     payments     │       │    research_facts    │
├──────────────────┤       ├──────────────────────┤
│ id (UUID, PK)    │       │ id (UUID, PK)        │
│ report_id (FK)   │       │ report_id (FK)       │
│ amount (15000)   │       │ fact_type            │
│ payment_status   │       │ category             │
│ transaction_id   │       │ statement            │
└──────────────────┘       │ source_url           │
                           └──────────────────────┘
```

### 4.2 Report State Machine (12 Canonical States)
`draft` $\rightarrow$ `researching` $\rightarrow$ `research_completed` $\rightarrow$ `generating` $\rightarrow$ `preview_ready` $\rightarrow$ `payment_pending` $\rightarrow$ `paid` $\rightarrow$ `unlocked` (with `failed` / `cancelled` error branches).

---

## 5. What is Needed to Make the System Fully Operational (Production Checklist)

To run Laporin in a real-world live production environment, the following external credentials, packages, and configurations are required:

### 5.1 Environment Variables Matrix (`backend/.env`)

| Variable | Description | Required For | Recommended Value / Provider |
| :--- | :--- | :--- | :--- |
| `DATABASE_URL` | PostgreSQL connection string | Database storage | `postgres://user:pass@host:5432/laporin` |
| `SERVER_HOST` | Backend bind host | Network listener | `0.0.0.0` (or `127.0.0.1` behind reverse proxy) |
| `SERVER_PORT` | Backend bind port | Network listener | `8080` |
| `SESSION_SECRET` | 32+ byte cryptographic secret | Cookie signing | Generate via `openssl rand -hex 32` |
| `MAYAR_API_KEY` | Mayar Gateway API key | Real payment creation | Live API key from [Mayar.id](https://mayar.id) |
| `MAYAR_WEBHOOK_SECRET`| HMAC-SHA256 secret key | Webhook validation | Configured in Mayar Webhook Dashboard |
| `NINEROUTER_API_KEY` | 9Router / LLM API key | AI extraction | API key from 9Router / OpenRouter |
| `GOOGLE_CLIENT_ID` | Google OAuth Client ID | Google Login | Google Cloud Console OAuth 2.0 Credentials |
| `GOOGLE_CLIENT_SECRET`| Google OAuth Client Secret| Google Login | Google Cloud Console OAuth 2.0 Credentials |
| `GOOGLE_REDIRECT_URI` | OAuth redirect URL | Google Login | `https://yourdomain.com/auth/google/callback` |
| `HCAPTCHA_SECRET` | hCaptcha Secret Key | Bot protection | Secret key from [hCaptcha](https://hcaptcha.com) |
| `STORAGE_BASE_PATH` | Local file storage root | Document storage | Absolute path e.g. `/var/lib/laporin/storage` |

### 5.2 External Infrastructure & Dependencies
1. **PostgreSQL Database Server (v14+)**:
   - Provision a PostgreSQL database.
   - Run SQLx migrations: `sqlx migrate run`.
2. **Headless LibreOffice (`soffice`)**:
   - Required for server-side PDF preview conversion.
   - Installation (Ubuntu/Debian): `sudo apt-get install -y libreoffice-core libreoffice-writer`.
3. **Valid DOCX Template**:
   - Placed at `storage/templates/report-template-v1.docx` containing placeholders (`{{student_name}}`, `{{company_name}}`, `{{chapter1_content}}`, etc.).
4. **Reverse Proxy & SSL (Nginx / Caddy / Cloudflare)**:
   - Terminate SSL (HTTPS).
   - Route `/` to Nuxt Frontend (port 3000).
   - Route `/api/v1/**` to Axum Backend (port 8080).
   - Configure WebSocket/HTTP upgrade headers for streaming if needed.

# AUDIT.md – Laporin Architecture & Contract Package Review (Post-Hardening Pass)

## 1. Summary of Current Architecture
- **Architecture Style**: Modular monolith in Rust (Axum) with PostgreSQL, serving a Nuxt 3 single-page frontend on a single VPS (2 GB RAM, 16 GB SSD storage).
- **Domain Boundaries**: `Auth`, `Report`, `Research`, `Generation`, `Payment`, `Storage`.
- **Infrastructure Footprint**: PostgreSQL for relational data and job queue (`FOR UPDATE SKIP LOCKED`), local disk for templates and generated files, 9Router for LLM inference abstraction.
- **Hardware Budget Enforcement**: 
  - Max 2 concurrent AI research workers, 1 concurrent DOCX/PDF rendering worker.
  - Memory guard: intake pauses when available RAM < 500 MiB.
  - Disk guard: report creation and job intake pauses when free disk < 1 GB.

---

## 2. Audit Resolution Matrix

| # | Item / Finding | Previous Status | Hardened Status | Resolution Details |
|---|----------------|-----------------|-----------------|--------------------|
| 1 | **Status Enum Alignment** | Inconsistent | **RESOLVED** | Unified to 12 lowercase snake_case values (`draft`, `researching`, `research_completed`, `generating`, `generated`, `preview_ready`, `payment_pending`, `paid`, `unlocked`, `failed`, `cancelled`, `expired`) across `architecture.md`, `openapi.yaml`, `database-schema.md`, and `report-schema.md`. |
| 2 | **API & Report Schema Discrepancy** | Missing fields | **RESOLVED** | Extended OpenAPI `Report` schema with `research_facts` (containing claim and source provenance) and `generated_sections` (cover, introduction, company_profile, activities, conclusion). |
| 3 | **Security Contract** | Placeholder | **RESOLVED** | Created comprehensive `security.md` covering Argon2id hashing parameters, HttpOnly/Secure/SameSite session cookies, SHA-256 hashed OTPs with expiry & retry limits, SSRF protection, CORS/CSRF headers, and prompt sanitization. |
| 4 | **CAPTCHA Enforcement** | Underspecified | **RESOLVED** | Embedded `captcha_token` into `RegisterRequest` and `LoginRequest` in `openapi.yaml` and documented hCAPTCHA backend verification in `security.md`. |
| 5 | **Payment & Entitlement Security** | Risk of tampering | **RESOLVED** | Created `payment-flow.md` and `ADR-002`. Strict backend-only unlock; Mayar webhook requires HMAC-SHA256 signature verification with idempotency on `transaction_id`. Frontend is strictly untrusted. |
| 6 | **Research Fact Provenance** | Loose linking | **RESOLVED** | Created `research-flow.md`, `ADR-003`, and updated `database-schema.md` with explicit `report_research_facts` table foreign-keying `research_sources.id`. |
| 7 | **PDF Preview Delivery** | Undefined/Cloud-bound | **RESOLVED** | Created `preview-flow.md` and `ADR-004`. Direct backend-authorized streaming via `GET /reports/{id}/preview`, using local filesystem without exposing internal server paths. |
| 8 | **Job Queue & State Machine** | Underspecified | **RESOLVED** | Created `jobs.md` and `ADR-005`. Documented PostgreSQL queue state machine, max 3 attempts with exponential back-off, crash recovery, stale job sweeps, and memory thresholds. |
| 9 | **Storage Layout & Lifecycle** | Placeholder | **RESOLVED** | Created `storage.md` and `ADR-006`. Concrete path schema, UUID v4 naming, canonical path traversal checks, 0700/0640 permissions, 30-min temp retention, and 30-day DOCX retention. |
| 10 | **Rate Limiting & 429 Error Shape** | Undefined | **RESOLVED** | Created `rate-limiting.md` and updated `openapi.yaml` with standard `429 Too Many Requests` responses, `X-RateLimit-*` headers, and configurable environment limits. |
| 11 | **LLM Model Configurability** | Unresolved Model | **RESOLVED** | Created `ADR-008`. 9Router serves as the local LLM routing layer; model names and fallbacks are strictly configured via environment variables rather than hard-coded. |
| 12 | **DOCX Template Placeholder Map** | Missing | **RESOLVED** | Created `template-placeholders.md` mapping all `USER_FACT`, `RESEARCH_FACT`, and `AI_DERIVED_TEXT` tags to report data. |

---

## 3. Explicit Assumptions & Justifications

1. **Headless LibreOffice for PDF Conversion**
   - *Status*: **ACCEPTED ASSUMPTION**
   - *Rationale*: Standard CLI utility available on Linux VPS. Memory consumption (~300 MB peak) is controlled by bounding generation concurrency to 1 worker.
2. **PostgreSQL as Sole Queue Broker**
   - *Status*: **ACCEPTED ASSUMPTION**
   - *Rationale*: `FOR UPDATE SKIP LOCKED` eliminates Redis/RabbitMQ RAM footprint, preserving maximum headroom for Axum and PostgreSQL within the 2 GB limit.
3. **hCAPTCHA Provider**
   - *Status*: **ACCEPTED ASSUMPTION**
   - *Rationale*: Privacy-friendly, standard site-verify HTTP POST endpoint, token passed seamlessly within authentication request payloads.

---

## 4. Implementation Readiness Verification

- **Total Inconsistencies Remaining**: `0`
- **Total Missing Contracts**: `0`
- **Remaining Blocking Issues**: `0`

### Conclusion
The architecture and contract package is **COMPLETE, INTERNALLY CONSISTENT, AND READY FOR IMPLEMENTATION** by OpenCode (Backend) and Claude Code (Frontend).

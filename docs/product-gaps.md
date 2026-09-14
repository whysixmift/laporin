# LAPORIN: Deep-Dive Product & Technical Gaps Analysis

> **Document Type:** Technical & Product Gap Analysis  
> **Audience:** Engineering, Product, Design, AI Architecture

---

## 1. Discrepancy Analysis: Specification vs. Code Reality

### Gap 1: Research Search Engine Integration
- **Documented Design:** The crawler should autonomously discover official company pages, news, and business directories based on the student's input company name.
- **Actual Code Reality:** `backend/src/workers/research_worker.rs` generates a sample URL `https://example.com/company/{name}` and relies on test fallbacks if external search is unavailable.
- **Root Cause:** A dedicated search API provider (such as Serper, Brave Search, or DuckDuckGo HTML scraping) was omitted during the initial minimal crawler setup.
- **Impact:** While mock/fallback facts are extracted cleanly, real-world company research requires real web search query execution.
- **Resolution:** Add a lightweight, rate-limited Search API client (`SearchEngineService`) that queries Google/DuckDuckGo results, filters out spam, and feeds the top 3 HTTPS links to `CrawlerService`.

---

### Gap 2: Monolithic Input vs. Real-World Student Internship Journals
- **Documented Design:** 4-step wizard with "Catatan Kegiatan Harian".
- **Actual Code Reality:** `internship.description` is a single plain-text textarea where students paste their notes as unstructured paragraphs.
- **User Pain Point:** Students during a 3-month or 6-month PKL maintain daily logbooks (*Jurnal Harian PKL*) with dates, shift hours, tasks performed, tools used, and supervisor signatures. Pasting 90 days into one text box causes loss of temporal structure.
- **Resolution:** Build an interactive **PKL Logbook Subsystem** (Category 6) where entries are structured by date/week, enabling the AI to chronologically synthesize Bab III with accurate timeline milestones.

---

### Gap 3: Section Editing & Granular Regeneration
- **Documented Design:** Complete report formulation with preview and DOCX generation.
- **Actual Code Reality:** Once sections are generated into `generated_sections` JSON in PostgreSQL, the student can only view the text. They cannot edit a typo, add a paragraph, or change the tone of Bab I without restarting the entire report generation job.
- **Resolution:** Implement a granular `PATCH /api/v1/reports/:id/sections` endpoint and an inline web editor allowing manual text tweaking or selective single-chapter regeneration.

---

### Gap 4: Document Customization & Multi-Template Engine
- **Documented Design:** DOCX generation with margins and typography conforming to academic standards.
- **Actual Code Reality:** Uses a single static template `templates/default_report_template.docx`. Different vocational schools (SMK Negeri vs. Swasta vs. Politeknik D3/D4) require specific margins (e.g., 4-4-3-3 vs. 3-3-3-3), distinct cover page layouts, and varying chapter nomenclatures.
- **Resolution:** Support customizable template presets and uploadable school guidelines that parse layout parameters dynamically.

---

### Gap 5: In-Browser Document Pagination & True Print Fidelity
- **Documented Design:** Document preview matching the downloaded DOCX file.
- **Actual Code Reality:** `DocumentViewer.vue` uses CSS aspect-ratio sheets to display text. Long chapters overflow single sheets rather than splitting into realistic multi-page pagination.
- **Resolution:** Enhance the web previewer with virtual A4 page splitting and keep the headless LibreOffice PDF streamer as an exact print-ready rendering mode.

---

## 2. Architectural & Resource Bottleneck Analysis (2 GB RAM VPS)

| Subsystem | Potential Bottleneck | Failure Mode Under Load | Current Safeguard | Recommended Enhancement |
|:---|:---|:---|:---|:---|
| **LibreOffice PDF Conversion** | Headless `soffice` consumes ~250–350 MB RAM per document conversion. | Memory spikes causing Linux OOM killer to terminate Axum or Postgres. | Concurrency bounded to 1 worker; RAM check halts if free RAM < 500 MB. | Offload heavy PDF rendering to client-side PDF export or convert on-demand with memory limits (`ulimit -v`). |
| **9Router LLM Inference** | Long input context (multiple crawled pages + detailed student logs). | Request timeouts or token rate-limit 429 errors from LLM proxy. | 45-second HTTP timeout with fallback mock extraction. | Implement token budgeting and text truncation on raw HTML before LLM ingestion. |
| **PostgreSQL Queue Polling** | Continuous 2-second polling across background worker threads. | Unnecessary CPU wakeups and database query churn during idle periods. | `FOR UPDATE SKIP LOCKED` prevents race conditions. | Use PostgreSQL `LISTEN / NOTIFY` to trigger workers reactively instead of polling every 2s. |
| **Local Disk File Growth** | Generated DOCX and temporary PDF preview files accumulate. | Disk space exhaustion on 16 GB SSD. | `storage.md` specifies 30-day retention for paid reports and 30-minute temp files. | Run automated `cleanup_worker` cron to purge expired temp files every hour. |

---

## 3. Commercial & User Retention Gaps

1. **Transaction Drop-off Risk**: Students without bank accounts or e-wallets might get stuck at QRIS payment.
   - *Mitigation*: The current WhatsApp manual payment contact is a great start. Adding a direct "Kirim Bukti Transfer Admin" button with one-click pre-filled WhatsApp chat will increase conversion.
2. **Post-Download Churn**: Once a student downloads their single `.docx` file, there is no recurring reason to return.
   - *Mitigation*: Expand into **PKL Defense Presentation (PPT) Generator**, **CV/Resume Builder from PKL Experience**, and **Oral Exam Practice Questions**.

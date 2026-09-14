# LAPORIN: Product & Engineering Phased Roadmap

> **Strategic Vision:** Evolve Laporin from a single-pass report generator into the definitive **Internship & Vocational Career Operating System** for Indonesian students.

---

## 🗺️ Execution Phases Overview

```
┌──────────────────────────────────────────────────────────────────────────────────┐
│  Phase A: MVP Hardening & Critical Essentials (Current Horizon)                  │
│  - Live Search Engine Crawler Integration (DuckDuckGo / Serper API)              │
│  - In-Place Chapter Editor & Polish Controls                                     │
│  - Form Autosave & Draft Local Storage Recovery                                  │
│  - Enhanced Direct WhatsApp Payment Automation                                   │
└─────────────────────────────────────────┬────────────────────────────────────────┘
                                          │
                                          ▼
┌──────────────────────────────────────────────────────────────────────────────────┐
│  Phase B: Student Workflow & Interactive Logbook                                 │
│  - Structured Daily PKL Logbook & Activity Journal Subsystem                     │
│  - Weekly Milestones to Bab III Synthesis Engine                                 │
│  - Major-Specific Vocational Templates (TKJ, RPL, AKL, Mesin, Otomotif)          │
│  - Automated Native Word Table of Contents & Figure Lists                        │
└─────────────────────────────────────────┬────────────────────────────────────────┘
                                          │
                                          ▼
┌──────────────────────────────────────────────────────────────────────────────────┐
│  Phase C: Competitive Moat & Academic Intelligence                               │
│  - School Guideline PDF Ingestion (Auto Margin, Heading & Typography Parsing)    │
│  - Timeline, Date & Supervisor Consistency Auditor                               │
│  - Photo & Certificate Attachment Gallery Ingestion                              │
│  - Classroom Referral & Peer Discount Engine ("Ajak Teman Sekelas")              │
└─────────────────────────────────────────┬────────────────────────────────────────┘
                                          │
                                          ▼
┌──────────────────────────────────────────────────────────────────────────────────┐
│  Phase D: Post-PKL Career Expansion & Monetization Multipliers                   │
│  - PKL Defense Presentation (.pptx) Generator for Sidang Magang                  │
│  - Sidang PKL Oral Exam Practice & Question Simulator                            │
│  - ATS CV & Portfolio Generator from Verified PKL Accomplishments                │
│  - OCR Vision Scanner for Physical Handwritten Logbooks                          │
└─────────────────────────────────────────┬────────────────────────────────────────┘
                                          │
                                          ▼
┌──────────────────────────────────────────────────────────────────────────────────┐
│  Phase E: Platform & Institutional Ecosystem                                     │
│  - School / Institutional Voucher Licensing Dashboard                            │
│  - Teacher & Mentor Review / Sign-off Portal                                     │
│  - Community Template Library & Vocational Standard Marketplace                  │
└──────────────────────────────────────────────────────────────────────────────────┘
```

---

## 📌 Phase A: MVP Hardening & Critical Essentials

* **Goal**: Close all immediate runtime gaps and optimize draft creation UX.

| Feature | Why / Problem Solved | Dependencies | Complexity | Expected Product Impact |
|:---|:---|:---|:---|:---|
| **Autonomous Search Engine Crawler** | Resolves the research worker bottleneck so real company names discover live HTTPS websites automatically. | Serper / DuckDuckGo API key, `CrawlerService` | Low (2 days) | 100% accurate company profile extraction across all Indonesian cities. |
| **In-Place Chapter Editor & Polisher** | Empowers students to tweak text or fix names before exporting final Word DOCX. | `PATCH /api/v1/reports/:id/sections` | Low (2 days) | Eliminates user frustration when a single sentence needs adjustment. |
| **Form Autosave & Draft Recovery** | Prevents data loss when students accidentally refresh or switch tabs. | Nuxt `useLocalStorage` composable | Low (1 day) | Zero lost drafts; boosts form completion rate by ~25%. |
| **Direct WhatsApp Transfer Bot Link** | Streamlines manual payments for students without QRIS / e-wallets. | WhatsApp pre-filled URL generator | Low (0.5 day) | Immediately recovers 20–30% of payment drop-offs. |

---

## 📌 Phase B: High-Value Student Workflow (The PKL Logbook)

* **Goal**: Transform Laporin into an everyday tool during the 3–6 month internship period.

| Feature | Why / Problem Solved | Dependencies | Complexity | Expected Product Impact |
|:---|:---|:---|:---|:---|
| **Interactive PKL Daily Logbook** | Students record day-to-day tasks with dates, divisions, and hours rather than a single text block. | PostgreSQL `report_journal_entries` table | Medium (4 days) | Massive user retention during the entire internship semester. |
| **Logbook $\rightarrow$ Bab III Synthesis** | Converts 30–90 daily entries into chronological, coherent academic sub-chapters automatically. | 9Router LLM Prompt Pipeline | Medium (3 days) | Solves the hardest part of report writing; unbeatable value proposition. |
| **Major-Specific Vocational Templates** | Adapts report terminology specifically for TKJ (networking), RPL (software), AKL (finance), and D3 engineering. | Template DOCX presets | Medium (3 days) | High relevance across diverse vocational school departments. |
| **Automated Table of Contents & Tables** | Microsoft Word files include native XML TOC and Table lists that update with 1 click. | `DocxService` XML schema update | Low (2 days) | Eliminates manual page numbering chores in Word. |

---

## 📌 Phase C: Differentiation & Academic Moat

* **Goal**: Build features that generic AI chatbots (ChatGPT, Claude) cannot replicate.

| Feature | Why / Problem Solved | Dependencies | Complexity | Expected Product Impact |
|:---|:---|:---|:---|:---|
| **School Guideline PDF Ingestion** | Extracts formatting guidelines (custom margins, font sizes, required chapters) from school PDFs. | PDF parsing + LLM structure extractor | High (5 days) | Total customization to any specific school's guideline rules. |
| **Timeline & Integrity Auditor** | Checks that student journal dates and company details are strictly non-contradictory. | Internal rule-engine | Low (2 days) | Eliminates teacher rejections due to date/timeline discrepancies. |
| **Lampiran Documentation Gallery** | Formats photos of student activities into a neat 2-column appendix grid in Word. | Multipart file upload + DOCX image insertion | Medium (3 days) | Complete, ready-to-print final report package. |
| **Classroom Referral System** | Students invite classmates to unlock discounts or free downloads. | Referral code tracking in DB | Low (2 days) | Organic viral loop across student cohorts. |

---

## 📌 Phase D: Post-PKL Career Expansion & Monetization

* **Goal**: Maximize revenue per user after report completion.

| Feature | Why / Problem Solved | Dependencies | Complexity | Expected Product Impact |
|:---|:---|:---|:---|:---|
| **Sidang PKL Defense Presentation (.pptx)** | Automatically creates a 10-slide PowerPoint presentation from the finished report. | Rust `pptx` generation library | Medium (4 days) | Additional monetization tier (e.g. +Rp10.000 bundle). |
| **Sidang PKL Question Simulator** | AI generates 10 predicted teacher defense questions and sample answers based on Bab III. | 9Router LLM | Low (2 days) | High student confidence before oral exams. |
| **ATS CV / Resume Generator** | Converts verified PKL activities into impactful CV bullet points for first-job applications. | Resume template engine | Low (2 days) | Extends student lifetime value beyond graduation. |
| **OCR for Handwritten Logbooks** | Allows uploading photos of physical paper journals to auto-populate the digital logbook. | Vision LLM API | Medium (3 days) | Huge relief for students whose schools enforce physical paper books. |

---

## 📌 Phase E: Platform & Institutional Ecosystem

* **Goal**: Institutional partnerships and B2B school adoption.

| Feature | Why / Problem Solved | Dependencies | Complexity | Expected Product Impact |
|:---|:---|:---|:---|:---|
| **School Bulk Licensing Portal** | Vocational schools purchase bulk licenses for entire grade cohorts. | Multi-tenant school accounts | High (6 days) | Predictable institutional annual recurring revenue. |
| **Teacher Review Portal** | Supervising teachers review and leave comments on student drafts online. | Role-based permissions & notifications | High (7 days) | Official classroom endorsement and deep moat. |
| **Vocational Template Marketplace** | Teachers and universities publish and share custom report templates. | Template publishing CMS | High (6 days) | Community-driven network effects. |

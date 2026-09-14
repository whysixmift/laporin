# LAPORIN: Complete Feature Backlog & Product Innovation Catalog

> **Version:** 2.0 Product Discovery  
> **Target Audience:** Indonesian Vocational Students (SMK), Applied Science Students (Politeknik D3/D4), and University Interns.

---

## 1. Feature Categories (1 through 18)

### CATEGORY 1 — REPORT CREATION & INPUT
1. **Guided Multi-Step Report Wizard (Current MVP)**: Step-by-step form capturing student identity, institution, DU/DI metadata, and journal notes.
2. **Local Autosave & Session Draft Recovery**: Saves form data automatically to browser `localStorage` and periodic background server sync so students never lose progress on accidental tab closure.
3. **Interactive Section Completion Indicators**: Visual progress meters showing required vs. optional fields completed before triggering generation.
4. **Smart Form Suggestions & Auto-Complete**: Suggestions for common vocational majors (RPL, TKJ, Akuntansi, Multimedia, Otomotif, Perhotelan) and standard school supervisor titles.
5. **Missing Information Warning & Hallucination Guard**: Warns the student if critical inputs (e.g. supervisor name, specific project role) are missing, preventing AI hallucinated names.
6. **Smart Defaults from User Profile**: Automatically reuses student name, school, NIS/NIM, and major across multiple reports or revisions.
7. **"Saya Tidak Tahu Data Ini" Assistant**: Contextual guidance explaining where students can find missing data (e.g., asking HR for company structure or checking the internship acceptance letter).

---

### CATEGORY 2 — COMPANY RESEARCH & PROVENANCE
8. **Autonomous Multi-Source Web Search Engine**: Integrates live search query engine (Serper / DuckDuckGo) to discover official website, Wikipedia, and LinkedIn company overviews automatically.
9. **Structured Company Fact Extraction**: Dissects company research into 4 distinct facets: History & Background, Vision & Mission, Organizational Structure, and Business Operations.
10. **Multiple-Source Cross Verification**: Cross-references claims across 2 or more sources to verify company location and industry sector.
11. **Conflicting Source Detection & User Override**: Alerts students if conflicting addresses or branch names are detected, allowing manual source selection.
12. **Source Explorer & Interactive Citation Viewer**: Lets students inspect exactly which web URL backed every paragraph in Bab II with direct links and timestamps.
13. **Manual Source & Fact Addition**: Allows students to paste private internal company documentation or company profiles not accessible via public internet.
14. **One-Click "Riset Ulang" with New Keywords**: Regenerates company facts if the initial query returned an incorrect branch or subsidiary.

---

### CATEGORY 3 — REPORT INTELLIGENCE & ACADEMIC AUDITOR
15. **Timeline & Date Consistency Auditor**: Verifies that daily journal activities fall strictly within the reported internship start and end dates.
16. **Company & Supervisor Name Consistency Engine**: Guarantees identical spelling and academic titles of supervisors across Cover, Lembar Pengesahan, Bab I, and Bab III.
17. **Unsupported Claim & Hallucination Detector**: Flags statements in Bab III not derived from student journal entries, preventing fabricated technical achievements.
18. **Indonesian Academic Style & PUEBI/EYD Checker**: Validates formal capitalization (e.g. *di mana* vs *dimana*, *analisis* vs *analisa*, *praktikan* vs *penulis*).
19. **Report Completeness Score (0–100%)**: Factual, data-driven score assessing whether all required sub-chapters (Latar Belakang, Tujuan, Manfaat, Profil, Pelaksanaan, Kesimpulan, Saran) are populated.

---

### CATEGORY 4 — DOCUMENT & EXPORT ENGINES
20. **DOCX XML Template Engine (Current MVP)**: High-speed placeholder substitution preserving strict 4-4-3-3 cm margins and Times New Roman 12pt typography.
21. **Multi-Track Template Presets**: Distinct pre-built templates for SMK Teknik (TKJ/RPL/Mesin), SMK Bisnis Manajemen (Akuntansi/Pemasaran), and Politeknik D3/D4.
22. **Automated Table of Contents (Daftar Isi, Daftar Tabel, Daftar Gambar)**: Generates Word-native TOC fields that update page numbers automatically in Microsoft Word.
23. **Lembar Pengesahan with Signature Boxes**: Formats official sign-off pages for School Principal, DU/DI Field Mentor, and Supervising Teacher with standard NIP/NIK lines.
24. **Document Attachment & Gallery Ingestion**: Embeds captioned internship photos (*Dokumentasi Kegiatan*) into Lampiran with 2-up grid layout.
25. **DOCX Integrity & Formatting Validator**: Automated server-side validation ensuring generated `.docx` packages are uncorrupted and pass OpenXML schema checks.

---

### CATEGORY 5 — USER INPUT / EVIDENCE INGESTION
26. **Photo of Notes & Certificate Upload**: Upload photos of printed internship acceptance letters or daily task log sheets.
27. **OCR Text Extraction for Handwritten Notes**: Extracts student handwriting from physical logbook photos using vision-capable models, converting them into structured activity notes.
28. **Upload PDF School Guidelines / Template**: Uploads school-issued PKL guideline PDFs to extract chapter names and specific formatting requirements.

---

### CATEGORY 6 — PKL DAILY LOGBOOK / JOURNAL SUBSYSTEM (CORE DIFFERENTIATOR)
29. **Structured Daily Journal / Logbook Tracker**: Dedicated sub-application where students log daily activities (Date, Hours, Division, Task Description, Tools Used).
30. **Weekly Milestone Summarizer**: Groups daily notes into structured weekly summaries (*Minggu ke-1 s.d. Minggu ke-12*).
31. **One-Click Journal to Bab III Transformation**: Automatically compiles all chronological logbook entries into formal academic prose in Bab III (*Pelaksanaan Praktik Kerja Lapangan*).
32. **Skill & Competency Tagging**: Tags entries with vocational skills (e.g., *MikroTik Configuration*, *Database Backup*, *Frontend Slicing*) for automatic inclusion in Bab IV conclusion.

---

### CATEGORY 7 — AI WRITING & EDITORIAL TOOLS
33. **In-place Chapter Text Editor**: Direct web-based editing of generated chapter paragraphs before final Word download.
34. **"Formalize / Bakukan Bahasa" Button**: Rewrites slang or conversational notes into standard formal Indonesian academic prose (PUEBI).
35. **"Perluas / Ringkas" Paragraph Controls**: Expands brief task descriptions into detailed technical execution steps or tightens wordy paragraphs.
36. **Automatic Abstract / Ringkasan Eksekutif Generator**: Synthesizes a 1-page executive summary in Indonesian and English for vocational final reports.

---

### CATEGORY 8 — SPECIALIZED VOCATIONAL TEMPLATES
37. **SMK Teknik Komputer & Jaringan (TKJ) Template**: Tailored for networking, server administration, fiber optics, and hardware maintenance.
38. **SMK Rekayasa Perangkat Lunak (RPL) Template**: Formatted for web/mobile software development, system analysis, and database schema documentation.
39. **SMK Akuntansi & Keuangan Lembaga (AKL) Template**: Specialized for financial ledger entries, tax reporting, and administrative journal tasks.
40. **Politeknik / Universitas D3/D4 Internship Template**: Academic-grade formatting with formal methodology and literature review sections.

---

### CATEGORY 9 — SCHOOL GUIDELINE ADAPTATION ENGINE
41. **Guideline PDF Ingestion & Parameter Extraction**: Parses school guideline PDFs to detect custom margins (e.g., 3-3-3-3 vs 4-4-3-3), custom font (Arial vs Times New Roman), and required chapter headings.
42. **Dynamic Chapter Nomenclature Adaptor**: Automatically switches terminology between *Praktik Kerja Lapangan (PKL)*, *Praktik Kerja Industri (Prakerin)*, and *Kuliah Kerja Nyata / Magang Industri*.

---

### CATEGORY 10 — COLLABORATION & SUPERVISOR REVIEW
43. **Read-Only Shareable Draft Link**: Generates a secure, watermarked review URL students can send to their teacher or DU/DI supervisor for feedback.
44. **Reviewer Comments & Revision Notes**: Allows mentors to leave timestamped feedback on specific chapters.

---

### CATEGORY 11 — USER EXPERIENCE & WORKSPACE
45. **Report Duplicate / Clone**: Clones an existing report structure for a classmate from the same company (clearing personal journal logs while keeping verified company facts).
46. **Report Version History & Restore**: Restores previous drafts or generations if a student wants to roll back custom edits.
47. **Archive & Trash Management**: Safely archives past semester reports with 30-day soft-delete recovery.

---

### CATEGORY 12 — PAYMENT & COMMERCIALIZATION
48. **Per-Report Paywall (Rp15.000 / $0.95 USD - Current MVP)**: Flat pricing for instant watermarked-free Word DOCX download.
49. **Direct WhatsApp Payment Notification Bot**: Pre-filled WhatsApp transfer confirmation link directly to admin for students without e-wallets.
50. **Classroom / Group Referral Codes (*Promo Teman Sekelas*)**: Students share a code: "Ajak 3 teman, dapatkan 1 laporan gratis".
51. **School / Institutional Licensing**: Bulk voucher bundles for vocational school departments (e.g., 100 student licenses at discounted rate).

---

### CATEGORY 13 — INTERNAL ADMIN & OPERATIONS
52. **Admin Operations Center (Current MVP)**: User management, live report monitor, system health, and free unlock tool.
53. **Dead-Letter Job Inspector & Retry Engine**: Admin UI to inspect failed LLM/crawler jobs and trigger instantaneous retries.
54. **Storage Quota & Cloud Sync Monitor**: Tracks disk consumption and Cloud CDN upload synchronization.

---

### CATEGORY 14 — PRODUCT ANALYTICS
55. **Funnel Drop-off Tracking**: Measures conversion at each stage: Landing $\rightarrow$ Register $\rightarrow$ Draft Creation $\rightarrow$ Research $\rightarrow$ Preview $\rightarrow$ Payment.
56. **AI Quality & Latency Telemetry**: Tracks average generation duration and user satisfaction across LLM models.

---

### CATEGORY 15 — SECURITY, TRUST & ETHICAL AI
57. **Session Management & Device Revocation**: Allows students to terminate active sessions across internet cafe / school computer lab PCs.
58. **AI Transparency Label & Human Authorship Declaration**: Generates a standard *Lembar Pernyataan Keaslian Naskah* compliant with academic integrity rules.

---

### CATEGORY 16 — QUALITY ASSURANCE & INTEGRITY
59. **Cross-Chapter Reference Validator**: Verifies that figures mentioned in text (e.g., "lihat Gambar 2.1") actually exist in the document appendices.
60. **Missing Table / Figure Detector**: Flags empty image placeholders before final rendering.

---

### CATEGORY 17 — CORE MOAT & DIFFERENTIATION
61. **The PKL Ecosystem**: The combination of **Interactive Daily Logbook $\rightarrow$ Verified DU/DI Research $\rightarrow$ School Guideline Ingestion $\rightarrow$ Standard DOCX Export**.

---

### CATEGORY 18 — POST-PKL CAREER EXPANSION (FUTURE)
62. **PKL Defense Presentation (PowerPoint .pptx) Generator**: Auto-generates a 10-slide PowerPoint presentation summarizing the report for oral examination (*Sidang PKL*).
63. **Viva / Sidang PKL Practice Simulator**: AI generates 10 predicted defense questions and sample answers based on the student's Bab III activities.
64. **ATS CV / Resume Generator from PKL Tasks**: Converts verified internship accomplishments into high-impact bullet points for student job applications.

---

## 2. Feature Prioritization & Scoring Matrix

| # | Feature Name | Cat | Value (1-5) | Complexity (1-5) | Cost (1-5) | Rev Impact (1-5) | Differentiation (1-5) | Priority |
|:---|:---|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
| 1 | **Autonomous Web Search Engine Integration** | 2 | 5 | 2 | 2 | 4 | 5 | **P0** |
| 2 | **In-Place Chapter Text Editor & Polisher** | 7 | 5 | 2 | 1 | 4 | 4 | **P0** |
| 3 | **Interactive PKL Daily Logbook Subsystem** | 6 | 5 | 3 | 1 | 5 | 5 | **P0** |
| 4 | **Local Autosave & Draft Recovery** | 1 | 5 | 1 | 1 | 3 | 3 | **P0** |
| 5 | **Direct WhatsApp Transfer Pre-filled Bot** | 12 | 5 | 1 | 1 | 5 | 3 | **P0** |
| 6 | **School Guideline PDF Ingestion Engine** | 9 | 5 | 4 | 2 | 5 | 5 | **P1** |
| 7 | **Major-Specific Vocational Templates (TKJ/RPL/AKL)**| 8 | 4 | 2 | 1 | 4 | 4 | **P1** |
| 8 | **Automated Table of Contents & Figure Lists** | 4 | 4 | 2 | 1 | 3 | 4 | **P1** |
| 9 | **PKL Defense PPTX Presentation Generator** | 18 | 5 | 3 | 2 | 5 | 5 | **P1** |
| 10 | **Student Referral Codes ("Ajak Teman")** | 12 | 4 | 2 | 1 | 5 | 3 | **P1** |
| 11 | **Timeline & Supervisor Consistency Auditor** | 3 | 4 | 2 | 1 | 3 | 4 | **P1** |
| 12 | **Photo Attachment Gallery for Lampiran** | 4 | 4 | 2 | 2 | 3 | 4 | **P1** |
| 13 | **OCR for Handwritten Logbooks** | 5 | 4 | 3 | 3 | 4 | 5 | **P2** |
| 14 | **Sidang PKL Question Simulator** | 18 | 4 | 2 | 2 | 4 | 5 | **P2** |
| 15 | **ATS Resume Generator from PKL Tasks** | 18 | 4 | 2 | 2 | 4 | 4 | **P2** |
| 16 | **Read-Only Shareable Draft Links** | 10 | 3 | 2 | 1 | 3 | 3 | **P2** |
| 17 | **Report Cloning for Group Mates** | 11 | 4 | 1 | 1 | 3 | 3 | **P2** |
| 18 | **Institutional School Bulk Voucher Bundles** | 12 | 4 | 3 | 1 | 5 | 3 | **P3** |
| 19 | **Supervisor Review & Approval Workflow** | 10 | 3 | 4 | 1 | 2 | 3 | **P3** |
| 20 | **Template Marketplace for Community Formats** | 8 | 3 | 4 | 2 | 3 | 4 | **P3** |

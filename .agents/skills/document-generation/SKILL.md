---
name: document-generation
description: DOCX template placeholder substitution, headless LibreOffice PDF conversion, low-memory execution, and preview delivery.
---

# Document Generation & Preview Skill

## 1. Scope & Ownership
OpenCode owns the DOCX document assembly engine and the LibreOffice PDF conversion worker. The specifications are governed by `docs/template-placeholders.md`, `docs/preview-flow.md`, and `docs/storage.md`.

## 2. DOCX Placeholder Replacement Rules
1. Load master template from `storage/templates/report-template-v1.docx` (read-only mode).
2. Replace all tags according to `docs/template-placeholders.md`:
   - `{{report_title}}`, `{{student_name}}`, `{{company_name}}`, etc. from `USER_FACT`s.
   - `{{cover_section}}`, `{{introduction_section}}`, `{{company_profile_section}}`, `{{activities_section}}`, `{{conclusion_section}}` from `AI_DERIVED_TEXT`s.
3. Validate OpenXML schema validity before saving to prevent corrupt Word documents.
4. Save resulting document to `storage/reports/{report_id}/docx/{uuid}.docx` with `0640` permissions.

## 3. PDF Preview Generation
1. Spawn headless LibreOffice with isolated user profile:
   ```bash
   soffice --headless --convert-to pdf --outdir <scratch_dir> <docx_path>
   ```
2. Apply execution timeout (60 seconds) and kill child process on failure.
3. Store converted PDF at `storage/reports/{report_id}/preview/{uuid}.pdf`.
4. Transition report status to `preview_ready`.

## 4. Resource Constraints
- Strictly enforce concurrency limit: **1 document rendering worker** at a time.
- Clean up all scratch files immediately after conversion.

# preview-flow.md – Preview Generation & Delivery Contract

## 1. Overview
Laporin delivers PDF previews directly from the Axum backend on a single VPS without relying on external cloud object storage or presigned URLs. This eliminates external cloud dependencies and prevents direct exposure of the server's filesystem.

## 2. Preview Generation Process
1. **Trigger**: When a Generation Job succeeds, it creates both the final `.docx` report and a low-resolution / watermarked `.pdf` preview.
2. **Converter**: The worker converts the generated DOCX to PDF using headless LibreOffice (`soffice --headless --convert-to pdf`) in an isolated temporary working directory with strict memory and CPU quotas.
3. **Storage**:
   - The generated PDF is stored at `storage/reports/{report_id}/preview/{uuid}.pdf`.
   - Permissions: `0640`, owner `laporin:laporin`.
   - Recorded in `generation_jobs.preview_path` and `file_entries` with `kind='preview'`.
4. **State Transition**: Once the preview PDF is written to disk, the report status advances to `preview_ready`.

## 3. Preview Authorization & Delivery Contract
- **Endpoint**: `GET /api/v1/reports/{id}/preview`
- **Authentication**: `sessionCookie` (HttpOnly cookie containing valid session token).
- **Authorization Checks**:
  1. Authenticated `user_id` must match `report_projects.user_id`. (Returns `403 Forbidden` if mismatched).
  2. Report `status` must be one of `preview_ready`, `payment_pending`, `paid`, or `unlocked`. (Returns `400 Bad Request` if report is in `draft`, `researching`, or `generating`).
  3. Physical preview file must exist on disk and resolve within the storage root. (Returns `404 Not Found` if missing).
- **Streaming Response**:
  - `Content-Type`: `application/pdf`
  - `Content-Disposition`: `inline; filename="preview-{report_id}.pdf"`
  - `Cache-Control`: `private, no-cache, no-store, must-revalidate`
  - `X-Content-Type-Options`: `nosniff`
  - The backend streams the file using chunked transfer encoding directly to the HTTP client.
- **Path Protection**:
  - The frontend and user NEVER receive internal server filesystem paths.
  - The path is resolved internally using `storage::resolve_preview_path(report_id)`.

## 4. Frontend Rendering Contract
- The Nuxt frontend renders the preview inside an embedded PDF viewer (e.g. PDF.js or `<iframe :src="'/api/v1/reports/' + report.id + '/preview'">` with session credentials enabled `credentials: 'include'`).
- If the endpoint returns HTTP 403 or 404, the frontend displays an actionable error message and prompts the user to refresh or regenerate.

---
*All preview file lifecycles conform to the cleanup and retention rules defined in `storage.md`.*

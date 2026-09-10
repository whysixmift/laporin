# research-flow.md – Research Job Contract

## 1. Overview
The **Research** job crawls up to **5 external sources** to gather factual information required for the report. It runs as a background worker triggered via `POST /reports/{id}/research/start`.

## 2. Input Validation
- **URL Scheme**: Only `https` is allowed.
- **Host Resolution**: Hostname must resolve to a public IP (no private ranges: 10/8, 172.16/12, 192.168/16, ::1, etc.).
- **Maximum Depth**: The crawler follows links up to **3 hops** from the original URL.
- **Source Limit**: At most **5 distinct sources** per report. The worker stops once the limit is reached.
- **Response Size**: Each HTTP response is limited to **2 MiB**. Larger bodies cause the fetch to abort and are marked as a failed source.

## 3. Timeout & Retry
- **Job Timeout**: 10 minutes total wall‑clock time. If exceeded, the job is marked `failed` with error `timeout`.
- **Retry Policy**: For transient network errors, the worker retries **up to 2 times** with exponential back‑off (30 s → 2 min).
- **Max Attempts**: The job record (`research_jobs.attempts`) caps at **3**. After the third failure the job is marked `failed` permanently.

## 4. Provenance & Storage
- Each fetched fact is stored in `research_facts` with a foreign key `source_id` referencing `research_sources.id`.
- `research_sources` table columns: `id (UUID)`, `url`, `title`, `fetched_at`, `confidence` (0‑1).
- The API response for a report includes `research_facts` where each fact contains an array of its source objects (as defined in `report-schema.md`).

## 5. Security
- URLs are sanitized; any attempt to access a disallowed scheme/host results in immediate job failure.
- Fetched HTML is stripped of `<script>`, `<iframe>`, and other executable elements before being passed to the LLM.
- The worker runs under a restricted OS user with no file‑system write permissions beyond its temporary directory.

## 6. Reporting
- Endpoint `GET /reports/{id}/research/status` returns a `JobInfo` object with fields `status`, `started_at`, `finished_at`, `error_code`, `error_message`.
- On success, `status = succeeded` and the report status transitions to `research_completed`.

---
*All configurable limits (max sources, timeout, retry counts) are exposed via environment variables for easy tuning.*

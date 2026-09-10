---
name: frontend-api-integration
description: Type-safe consumption of OpenAPI endpoints, session management, polling async job status, and robust error handling without fake mocks.
---

# Frontend API Integration Skill

## 1. Scope & Ownership
Claude Code owns the frontend API client layer, connecting Vue components to the Axum backend according to `docs/openapi.yaml`.

## 2. Core API Client Guidelines
1. **Source of Truth**: `docs/openapi.yaml` is the contract. Generate or maintain TypeScript types that mirror OpenAPI components (`Report`, `JobInfo`, `PaymentResponse`, `ErrorResponse`).
2. **Authentication & Session**:
   - Requests use `credentials: 'include'` to send `session_id` cookies automatically.
   - Attach `X-CSRF-Token` header for non-GET state-modifying requests if configured.
3. **No Undocumented Mocks**: Never create fake local endpoints as permanent replacements for missing backend features. If an endpoint is missing or returns unexpected shapes, raise a contract issue.
4. **Standardized Error Handling**:
   - Extract `error.code` and `error.message` from `ErrorResponse`.
   - Map `RATE_LIMIT_EXCEEDED` (HTTP 429) to a user-friendly countdown toast with retry timing from `X-RateLimit-Reset`.
5. **Asynchronous Job Polling**:
   - Poll `GET /reports/{id}/research/status` and `GET /reports/{id}/generation/status` every **2-3 seconds**.
   - Stop polling immediately when status becomes `succeeded`, `failed`, or `cancelled`.
   - Implement max polling timeout (12 minutes) to prevent endless background network loops.

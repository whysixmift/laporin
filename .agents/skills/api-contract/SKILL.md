---
name: api-contract
description: Backend implementation of OpenAPI 3.1 contracts, request validation, standardized error responses, rate-limiting headers, and session cookies.
---

# API Contract Implementation Skill

## 1. Scope & Ownership
OpenCode is responsible for implementing and maintaining all endpoints defined in `docs/openapi.yaml`.

## 2. Core Implementation Rules
1. **Endpoint Routing & Methods**: Exact match with OpenAPI specification (e.g. `POST /api/v1/reports`, `GET /api/v1/reports/{id}/preview`).
2. **Request Validation**: Use `validator` or serde deserialization with custom validation to reject malformed data before reaching domain services.
3. **Session Cookie Security**:
   - Set cookie `session_id=<token>; HttpOnly; Secure; SameSite=Strict; Path=/; Max-Age=86400`.
4. **Standardized Error Response**:
   All 4xx and 5xx responses must match:
   ```json
   {
     "error": {
       "code": "BAD_REQUEST",
       "message": "Human-readable description",
       "request_id": "uuid"
     }
   }
   ```
5. **Rate Limiting Headers**:
   Include `X-RateLimit-Limit`, `X-RateLimit-Remaining`, and `X-RateLimit-Reset` on all rate-limited routes.
6. **Payload Sizing**:
   Return only fields specified in schemas; avoid payload bloat.

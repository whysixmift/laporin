# rate-limiting.md – Rate Limiting & 429 Policy Contract

## 1. Overview
Rate limiting is enforced at the Axum middleware layer using a configurable in-memory token bucket / sliding window rate limiter per IP address and per authenticated `user_id`.

## 2. Standardized HTTP Response Shape
When a client exceeds the allowed request rate, the server terminates the request immediately with `HTTP 429 Too Many Requests`:

### Response Headers
Every response (both permitted and throttled) includes standard rate limit headers:
- `X-RateLimit-Limit`: Maximum requests permitted in the current window.
- `X-RateLimit-Remaining`: Remaining requests permitted in the current window.
- `X-RateLimit-Reset`: Unix timestamp (in seconds) when the current window resets.

### Response Body (JSON)
```json
{
  "error": {
    "code": "RATE_LIMIT_EXCEEDED",
    "message": "Too many requests. Please retry after the reset window.",
    "request_id": "c1f76d91-49b2-4d5e-85e7-0130dbf2a514"
  }
}
```

## 3. Configurable Limits
All rate limits are configured via environment variables and loaded at startup:

| Route Group / Endpoint | Environment Variable | Default Limit | Window | Key |
|------------------------|----------------------|---------------|--------|-----|
| `/auth/register`, `/auth/login` | `AUTH_RATE_LIMIT_PER_MIN` | 10 requests | 1 minute | Client IP |
| `/auth/otp/verify` | `OTP_RATE_LIMIT_PER_HOUR` | 5 requests | 1 hour | `user_id` / IP |
| `/reports/*/research/start` | `RESEARCH_START_RATE_LIMIT_PER_HOUR` | 5 requests | 1 hour | `user_id` |
| `/reports/*/generation/start` | `GEN_START_RATE_LIMIT_PER_HOUR` | 5 requests | 1 hour | `user_id` |
| Global Authenticated API | `API_RATE_LIMIT_PER_MIN` | 60 requests | 1 minute | `user_id` |
| Public Static / Status | `PUBLIC_RATE_LIMIT_PER_MIN` | 120 requests | 1 minute | Client IP |

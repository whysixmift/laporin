# security.md – Security Contract

## 1. Password Handling
- **Hashing algorithm**: Argon2id (memory cost ≥ 64 MiB, parallelism = 1, iterations = 3). Stored hash includes version, salt, and parameters in the PHC string format.
- **Storage**: `users.password_hash` column stores only the Argon2id hash. Plain passwords are never persisted.
- **Verification**: Constant‑time comparison of the hash.

## 2. Session Management
- Sessions are represented by a signed, random UUID stored in `sessions.id` and a `token_hash` column (SHA‑256 of the session token). The raw token is sent to the client in an **HttpOnly**, **Secure**, **SameSite=Strict** cookie named `session_id`.
- **Expiration**: Configurable (default 24 h). Expired sessions are purged nightly.
- **Revocation**: `/auth/logout` sets `revoked = true`. A revoked session is rejected on all authenticated calls.

## 3. OTP (Email Verification / Password Reset)
- OTPs are 6‑digit numeric strings generated with a cryptographically secure RNG.
- **Storage**: Store only a SHA‑256 hash of the OTP in `email_otps.otp_hash` (column renamed from `otp`).
- **Expiry**: 10 minutes after creation.
- **Rate limiting**: Max 5 OTP generation attempts per hour per user. Max 3 verification attempts per OTP before it is invalidated.
- **Verification flow**: client submits plain OTP; backend hashes and compares.

## 4. CAPTCHA Enforcement
- Provider: **hCAPTCHA** (default). The frontend obtains a site‑key and sends `captcha_token` with every registration and login request.
- Backend verifies the token by POSTing to `https://api.hcaptcha.com/siteverify` with the secret key.
- The verification step must succeed (`success: true`) before proceeding with auth logic.
- The token is **not** stored; it is validated on each request.

## 5. OAuth (Google)
- Use the standard OAuth 2.0 Authorization Code flow.
- Validate the `id_token` signature using Google’s public keys (JWKS endpoint).
- On successful verification, look up or create an `oauth_identities` record.
- **Account linking**: If an existing email already has a password account, link the Google identity to the same user record. Duplicate Google accounts for the same email are rejected.
- **Logout**: Revokes the session as above; does not need to call Google.

## 6. CSRF & CORS
- **CSRF**: Since we use a SameSite‑Strict cookie for session, CSRF risk is minimal. For any non‑GET state‑changing endpoint, require an `X‑CSRF‑Token` header that matches a per‑session token stored server‑side (optional but recommended).
- **CORS**: Allow only the origin `https://app.laporin.example.com` (the Nuxt frontend). Reject all others.

## 7. Authorization
- All protected endpoints require the `sessionCookie` security scheme.
- Authorization is scoped to the authenticated user’s `user_id`. Resources (reports, payments, files) contain a `user_id` foreign key and must be checked against the session owner.

## 8. SSRF & External Content
- Research job input URLs are validated against a whitelist of schemes (`http`, `https`). Hostnames are resolved and checked against a deny‑list of private IP ranges (10/8, 172.16/12, 192.168/16, ::1, etc.).
- Maximum response size: 2 MiB. Larger bodies are truncated and flagged as `failed`.

## 9. Filesystem Security
- All file paths are built from a root directory (`/home/avrjulian/laporin/storage`). User‑supplied filenames are never used; files are named deterministically using UUIDs.
- Path traversal is prevented by joining with the storage root and then canonicalizing (`realpath`). Any path escaping the root is rejected.
- Files are owned by the `laporin` Unix user with permissions `0640` (readable only by the owner and group).

## 10. Webhook Verification (Mayar)
- Mayar includes a `signature` header computed as HMAC‑SHA256 over the JSON payload using a secret (`MAYAR_WEBHOOK_SECRET`). The backend must verify this signature before processing.
- The `transaction_id` is treated as an idempotency key; duplicate webhook deliveries with the same `transaction_id` must be ignored after the first successful handling.

## 11. Prompt Injection & Hostile Research Content
- All user‑provided text that is interpolated into LLM prompts is **escaped** (e.g., JSON‑encoded) and never concatenated as raw strings.
- The research worker sanitizes fetched HTML by stripping scripts, iframes, and other executable content before feeding it to the LLM.
- Confidence scores < 0.5 are flagged; the generation step must not use low‑confidence facts without explicit reviewer approval (out of scope for MVP, but the flag is stored).

## 12. Rate Limiting (Global)
- Configurable per‑endpoint limits (default values listed in `rate-limit-config.yaml`).
- Exceeding limits returns `429 Too Many Requests` with the `TooManyRequests` response schema and the headers `X-RateLimit-Limit`, `X-RateLimit-Remaining`, `X-RateLimit-Reset`.

---
*All cryptographic secrets are loaded from environment variables; they must never be committed to source control.*

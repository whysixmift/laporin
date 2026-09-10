# ADR-007: Security, Authentication, and Threat Mitigation Policy

## Status
Accepted

## Context
Laporin handles student data, payments, and external web crawling. Robust protection against credential stuffing, CSRF, SSRF, and prompt injection is required.

## Decision
- Passwords hashed with Argon2id.
- Session tokens stored as SHA-256 hashes and delivered via `HttpOnly`, `Secure`, `SameSite=Strict` cookies.
- Email OTPs stored as SHA-256 hashes with 10-minute expiry and attempt throttling.
- CAPTCHA tokens (hCAPTCHA) embedded into register/login requests.
- SSRF mitigation: external URLs must resolve to public non-private IP addresses.
- Prompt injection mitigation: user text is structured and escaped before feeding into LLM prompts.

## Consequences
- **Positive**: Comprehensive defense-in-depth security posture suitable for production.
- **Negative**: Adds verification latencies for CAPTCHA and Argon2id hashing.

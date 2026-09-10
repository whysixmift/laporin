# ADR-002: Backend-Authoritative Payment & Entitlement Verification

## Status
Accepted

## Context
Payment processing is handled via Mayar. Frontend redirect flows are susceptible to client-side tampering or spoofed success parameters.

## Decision
- The backend is the **sole authority** for unlocking reports.
- Payment completion is verified exclusively through cryptographically signed webhooks (HMAC-SHA256 with `MAYAR_WEBHOOK_SECRET`) and reconciliation workers.
- The `transaction_id` is stored uniquely to guarantee idempotency.
- The frontend is strictly untrusted for unlock state.

## Consequences
- **Positive**: Immune to client-side unlock tampering; robust duplicate event handling.
- **Negative**: Requires reliable webhook delivery and fallback reconciliation worker.

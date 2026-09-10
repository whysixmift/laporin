---
name: payment-integration
description: Mayar payment gateway integration, HMAC-SHA256 webhook validation, transaction_id idempotency, and backend-authoritative unlock.
---

# Payment Integration & Entitlement Skill

## 1. Scope & Ownership
OpenCode owns all payment gateway interactions, webhook endpoints, signature verification, and report unlocking logic. The specification is defined in `docs/payment-flow.md` and `docs/architecture-decisions/ADR-002-payment-flow.md`.

## 2. Invariants
- **Backend is Sole Unlock Authority**: The frontend is NEVER trusted for payment success or unlock state.
- **Webhook Signature Verification**: Every Mayar webhook request must have its HMAC-SHA256 signature verified against `MAYAR_WEBHOOK_SECRET` before parsing or processing.
- **Idempotency**: Treat `transaction_id` from Mayar as a unique key. If a payment is already `succeeded`, return `HTTP 200` and take no action.

## 3. Webhook Flow
```
Incoming POST /api/v1/webhook/mayar
              │
              ▼
Verify HMAC-SHA256(body, MAYAR_WEBHOOK_SECRET) == Header["signature"]
              │
         ┌────┴────┐
       [Valid]  [Invalid] ──> Return 400 Bad Request
         │
         ▼
Lookup payment by provider_tx_id = transaction_id
         │
         ├─── If status == 'succeeded' ──> Return 200 OK (Idempotent ignore)
         │
         └─── If status == 'pending':
                BEGIN TRANSACTION;
                UPDATE payments SET status = 'succeeded', updated_at = NOW() WHERE id = $payment_id;
                UPDATE report_projects SET status = 'unlocked', updated_at = NOW() WHERE id = $report_id;
                COMMIT;
                Return 200 OK;
```

## 4. Reconciliation
- A scheduled background job runs hourly to identify `pending` payments older than 30 minutes with no webhook, marking them `expired`.

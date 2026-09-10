# payment-flow.md – Payment & Entitlement Contract

## 1. Payment State Machine (canonical)
The payment object (`payments` table) progresses through the following states **exactly as listed** (lower‑case snake case). All state transitions are performed **only by the backend**.

| Current State | Event / Trigger                              | New State   | Remarks |
|---------------|----------------------------------------------|-------------|---------|
| created       | `POST /payments` creates the record          | pending     | `payment_url` sent to frontend for user to complete payment. |
| pending       | Mayar webhook with `status: success` and a **valid signature** | succeeded   | Entitlement granted, report status → `unlocked`. |
| pending       | Mayar webhook with `status: failure`          | failed      | User may retry; report stays in `payment_pending`. |
| pending       | No webhook received after **30 min** (configurable) | expired    | Payment considered abandoned; report status → `payment_pending` (user can retry). |
| succeeded     | Backend marks payment as `succeeded` (idempotent) | succeeded   | Duplicate webhook with same `transaction_id` is ignored. |
| failed / expired | User initiates a new payment (`POST /payments`) | pending    | New payment record created. |
| any           | User cancels before payment is completed (frontend action) | cancelled   | Backend deletes the pending payment record. |

### 1.1. Idempotency & Duplicate Webhook Handling
- The `transaction_id` field from Mayar is treated as a **global idempotency key**. On receipt of a webhook:
  1. Look up a payment by `provider_tx_id = transaction_id`.
  2. If a payment with status `succeeded` already exists, **ignore** the webhook (return 200).
  3. If the payment is in `pending`, update its status accordingly.
- Webhook processing must be **atomic** (SQL transaction) to avoid race conditions.

### 1.2. Reconciliation
- Periodic background task (once per hour) queries Mayar for any `pending` payments older than 30 min that have no webhook. If such payments are found, they are marked `expired`.
- The task also verifies that every `succeeded` payment has an associated **unlock** flag on the related report (`report_projects.status = unlocked`). If not, it performs the unlock.

## 2. Backend‑Only Unlock Logic
- The frontend **must never** set `report.status = unlocked`.
- Unlock occurs **only** after the backend processes a successful webhook and updates the payment record.
- The endpoint `GET /reports/{id}` will reflect the unlocked status.

## 3. Security of the Webhook
- Mayar signs the JSON payload with HMAC‑SHA256 using a secret (`MAYAR_WEBHOOK_SECRET`).
- Backend verifies the signature **before any state change**.
- The secret is provided via environment variable; never logged.

## 4. Error Handling
- Invalid signature → HTTP 400, log and discard.
- Missing required fields → HTTP 400.
- Any database error → HTTP 500, retry later.

---
*All state names and transition rules are shared with `architecture.md` and the OpenAPI `Payment` schema.*

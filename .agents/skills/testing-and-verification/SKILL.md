---
name: testing-and-verification
description: Test-first verification workflow (implement -> test -> inspect -> fix -> verify flows -> inspect diff -> report).
---

# Testing and Verification Skill

## 1. Core Rule: "Compiling is Not Verification"
An agent must never claim a task is completed merely because the code compiles without errors or linter warnings. Real behavior must be exercised and validated.

## 2. Verification Loop
```
Implement / Modify Code
         │
         ▼
Run Targeted Automated Tests
         │
         ▼
Inspect Test Outputs / Logs
         │
    ┌────┴────┐
 [Fail]    [Pass]
    │         │
    ▼         ▼
Fix Bug   Verify Affected Full Flows (API / E2E)
    │         │
    └─────────┼─────────┐
              ▼         ▼
       Inspect Diff  Report Completion
```

## 3. Test Coverage Boundaries

### Unit Tests
- Business logic validation (student info constraints, date parsing, fact sanitization).
- Rate limit bucket calculations and time window expiry.
- HMAC-SHA256 signature verification functions.
- Canonical path validation logic.

### Integration Tests (Backend)
- Database repository queries (`FOR UPDATE SKIP LOCKED`, cascading deletes).
- State transitions (e.g. `pending` -> `running` -> `succeeded`).
- Idempotent webhook handling (duplicate `transaction_id` ignored).
- Session cookie creation, validation, and revocation.

### API & End-to-End Tests
- OpenAPI request/response schema validation tests.
- Full asynchronous job lifecycle (start -> poll status -> complete).
- Report preview streaming endpoint authorization.

### Browser / UI Tests (Frontend)
- End-to-end user journeys (Register -> Login -> Report Creation -> Research -> Generation -> Preview -> Payment -> Download).
- Reactive state updates (polling indicators, error banners, download buttons).

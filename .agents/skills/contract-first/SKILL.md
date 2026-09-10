---
name: contract-first
description: Strict contract-first development workflow ensuring OpenAPI, DB schema, report schema, and state machines are checked and followed before code is written.
---

# Contract-First Development Skill

## 1. The Contract Hierarchy
1. **API Contracts**: `docs/openapi.yaml`
2. **Domain Data Models**: `docs/report-schema.md`
3. **Relational Database**: `docs/database-schema.md`
4. **State Transitions**: `docs/architecture.md`, `docs/payment-flow.md`, `docs/jobs.md`
5. **Security & Auth**: `docs/security.md`
6. **Filesystem & Preview**: `docs/storage.md`, `docs/preview-flow.md`, `docs/template-placeholders.md`

## 2. Strict Pre-Implementation Protocol
Before implementing any feature:
1. **Identify**: Locate the exact section in `docs/` governing the feature.
2. **Read**: Review the schemas, status enums, request/response models, and error structures.
3. **Verify Ownership**: Backend owns API implementation and DB schema; Frontend owns UI and API client consumption.
4. **Check Existing Contracts**: If an endpoint or field is not in `docs/openapi.yaml` or `docs/report-schema.md`, **STOP**. Do NOT invent it silently.

## 3. Explicit Prohibitions
- ❌ **No Silently Invented Endpoints**: Never add unapproved routes to the backend.
- ❌ **No Ad-Hoc Payload Changes**: Never return undocumented JSON fields or change data types.
- ❌ **No Frontend Mock Sprawl**: Frontend must never build persistent mock endpoints that diverge from OpenAPI.
- ❌ **No Direct State Manipulation**: Frontend must never bypass state machines (e.g. setting report status to `unlocked` directly).

## 4. Contract Amendment Protocol
If a genuine business or technical requirement necessitates changing a contract:
1. Document the proposed modification in an RFC/discussion format.
2. Update the contract file in `docs/` (`openapi.yaml`, `database-schema.md`, etc.).
3. Update all dependent contracts to preserve complete cross-document consistency.
4. Update tests and ADRs accordingly.

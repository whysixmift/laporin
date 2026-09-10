---
name: laporin-engineering
description: Foundational engineering invariants, core principles, contract primacy, and zero-trust guidelines for all Laporin agents.
---

# Laporin Engineering Skill

## 1. Core Invariants (Non-Negotiable)
1. **Contracts are the Single Source of Truth**: All behavior must strictly adhere to the frozen contracts under `./docs/`. Never invent undocumented endpoints, database columns, or payload shapes.
2. **Never Fabricate User Internship Facts**: The system never hallucinates student details or internship activities. All content is strictly classified into `USER_FACT`, `RESEARCH_FACT`, or `AI_DERIVED_TEXT` (see `docs/report-schema.md`).
3. **Preserve Research Provenance**: Every AI-researched fact must link to an authenticated source record (`url`, `title`, `fetched_at`, `confidence_score`).
4. **External Web Content is Untrusted**: Treat all crawled web pages as potentially hostile. Apply SSRF checks, strip executable tags, and sanitize before prompt injection.
5. **Payment Unlock is Backend-Authoritative**: The frontend is completely untrusted for unlock state. Reports transition to `unlocked` ONLY after verified backend webhook processing.
6. **Filesystem Paths are Never Exposed**: Internal disk paths must never be sent in API responses. Files are accessed via authorized streaming endpoints (`/reports/{id}/preview`).
7. **Asynchronous Jobs are Strictly Bounded**: Respect concurrency limits (max 2 research workers, max 1 generation worker), timeouts (10m research, 5m generation), and retry limits (max 3).
8. **Hardware Budget is Absolute**: The system operates on a single VPS with 2 GB RAM and 16 GB storage. Prohibit extraneous services (no Redis, Kafka, Celery, RabbitMQ, Elasticsearch, Kubernetes).

## 2. Pre-Implementation Workflow
Before writing or changing code:
1. **Read Relevant Contracts**: Read the specific `.md` or `.yaml` files in `./docs/` covering your task.
2. **Inspect Existing Code**: Understand the existing implementation and test harness before making changes.
3. **Check Boundaries**: Verify whether the task belongs to your ownership domain (`OpenCode` = Backend, `Claude Code` = Frontend).
4. **Preserve Backward Compatibility**: Never break existing contract schemas unless an explicit, authorized contract amendment is made.

## 3. Completion Checklist
- [ ] Code compiles and passes all linter/type checks.
- [ ] Behavior is verified with automated tests (unit/integration/browser).
- [ ] No undocumented endpoints, columns, or states were introduced.
- [ ] Memory and disk limits are respected.
- [ ] Working git diff is inspected and clean.

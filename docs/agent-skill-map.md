# agent-skill-map.md – Agent Skill Mapping & Ownership Matrix

This document maps all operational skills available under `.agents/skills/` to their respective agents, trigger conditions, and dependent architecture contracts.

---

## 1. Skill Mapping Matrix

| Skill Name | Scope / Category | Assigned Agent | When to Invoke | Dependent Contract Documents |
|---|---|---|---|---|
| `laporin-engineering` | **Shared** | OpenCode & Claude Code | Always active; before any planning, design, or implementation step. | `architecture.md`, all `./docs/*` |
| `contract-first` | **Shared** | OpenCode & Claude Code | When preparing to implement new features, APIs, or data schemas. | `openapi.yaml`, `database-schema.md`, `report-schema.md` |
| `testing-and-verification` | **Shared** | OpenCode & Claude Code | After code changes, during verification loops, and before claiming completion. | All contracts |
| `git-workflow` | **Shared** | OpenCode & Claude Code | During branch creation, commit staging, and working copy synchronization. | N/A |
| `browser-verification` | **Shared** | Claude Code (primary), OpenCode (E2E) | When verifying end-to-end user journeys, UX states, and UI responsiveness. | `openapi.yaml`, `preview-flow.md` |
| `resource-awareness` | **Shared** | OpenCode & Claude Code | When evaluating server architecture, job concurrency, or data retention. | `architecture.md`, `jobs.md`, `storage.md` |
| `rust-backend` | **Backend** | OpenCode | When implementing Axum routers, middlewares, error handling, or app state. | `architecture.md`, `security.md` |
| `postgresql` | **Backend** | OpenCode | When writing SQL migrations, queries, indexes, or worker queue locks. | `database-schema.md`, `jobs.md` |
| `api-contract` | **Backend** | OpenCode | When implementing or testing API endpoints and serialization models. | `openapi.yaml`, `rate-limiting.md` |
| `ai-research` | **Backend** | OpenCode | When building the web crawler, 9Router LLM client, or fact extractor. | `research-flow.md`, `ADR-003`, `ADR-008` |
| `document-generation` | **Backend** | OpenCode | When implementing DOCX placeholder replacement and headless LibreOffice PDF previews. | `template-placeholders.md`, `preview-flow.md`, `storage.md` |
| `payment-integration` | **Backend** | OpenCode | When implementing Mayar checkout, webhook signature verification, and report unlocking. | `payment-flow.md`, `ADR-002` |
| `nuxt-frontend` | **Frontend** | Claude Code | When structuring Vue 3 pages, components, composables, and styling. | `architecture.md` |
| `ui-from-figma` | **Frontend** | Claude Code | When translating Figma designs and design tokens into Tailwind CSS / Vue. | Design specs |
| `frontend-api-integration` | **Frontend** | Claude Code | When creating API client services, polling jobs, or managing auth sessions. | `openapi.yaml`, `rate-limiting.md` |
| `frontend-ux-states` | **Frontend** | Claude Code | When building reactive UI state machines (loading, researching, preview, payment). | `report-schema.md`, `payment-flow.md`, `jobs.md` |

---

## 2. Agent Ownership Boundaries

### OpenCode (Backend Owner)
- **Primary Scope**: Rust (Axum), PostgreSQL, SQLx migrations, 9Router integration, LibreOffice rendering, Mayar webhook processing, job queues, local storage management.
- **Contract Primacy**: Enforces `database-schema.md`, `openapi.yaml`, `security.md`, `jobs.md`, `storage.md`, `payment-flow.md`, `research-flow.md`.
- **Strict Boundary**: Never modifies frontend Vue/Nuxt files without prior coordination.

### Claude Code (Frontend Owner)
- **Primary Scope**: Nuxt 3, Vue 3 Composition API, TypeScript types, Pinia/useState, Tailwind CSS, embedded PDF preview rendering, browser verification.
- **Contract Primacy**: Consumes `openapi.yaml`, implements UX states matching `report.status` and `payment.status`.
- **Strict Boundary**: Never modifies Rust backend source files or database schemas; never creates permanent fake mock endpoints that diverge from `openapi.yaml`.
